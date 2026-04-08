use crate::{build_default_configs, Configs};
use once_cell::sync::Lazy;
use std::env;
use std::fmt::{Display, Formatter};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const DEFAULT_CONFIG_PATH: &str = "config.toml";
const DEFAULT_APP_DIR: &str = "study_rust";

#[derive(Debug)]
pub enum ConfigError {
    ReadFile(std::io::Error),
    ParseToml(toml::de::Error),
    SerializeToml(toml::ser::Error),
    InvalidNumber { key: &'static str, value: String },
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReadFile(err) => write!(f, "read config file failed: {err}"),
            Self::ParseToml(err) => write!(f, "parse config toml failed: {err}"),
            Self::SerializeToml(err) => write!(f, "serialize config toml failed: {err}"),
            Self::InvalidNumber { key, value } => {
                write!(f, "parse env `{key}` to number failed, value: `{value}`")
            }
        }
    }
}

impl std::error::Error for ConfigError {}

pub static CFG: Lazy<Configs> = Lazy::new(|| {
    load_config().unwrap_or_else(|err| {
        panic!("load config failed: {err}");
    })
});

pub fn load_config() -> Result<Configs, ConfigError> {
    let _ = dotenv::dotenv();
    let config_path = locate_config_file()?;
    let content = fs::read_to_string(&config_path).map_err(ConfigError::ReadFile)?;
    let mut cfg: Configs = toml::from_str(&content).map_err(ConfigError::ParseToml)?;
    normalize_config_paths(&mut cfg, &config_path);
    apply_env_overrides(&mut cfg)?;
    Ok(cfg)
}

fn locate_config_file() -> Result<PathBuf, ConfigError> {
    if let Ok(custom_path) = env::var("APP_CONFIG_PATH") {
        let path = PathBuf::from(custom_path.trim());
        if path.exists() {
            return Ok(path);
        }
    }

    let cwd = env::current_dir().map_err(ConfigError::ReadFile)?;
    for dir in cwd.ancestors() {
        let candidate = dir.join(DEFAULT_CONFIG_PATH);
        if candidate.exists() {
            return Ok(candidate);
        }
    }

    let fallback = ensure_home_config_file()?;
    Ok(fallback)
}

fn ensure_home_config_file() -> Result<PathBuf, ConfigError> {
    let home_dir = resolve_home_dir()?;
    let app_dir = home_dir.join(DEFAULT_APP_DIR);
    let config_path = app_dir.join(DEFAULT_CONFIG_PATH);

    if !config_path.exists() {
        fs::create_dir_all(&app_dir).map_err(ConfigError::ReadFile)?;
        fs::create_dir_all(app_dir.join("data")).map_err(ConfigError::ReadFile)?;
        fs::create_dir_all(app_dir.join("data/_web")).map_err(ConfigError::ReadFile)?;
        fs::create_dir_all(app_dir.join("data/_upload")).map_err(ConfigError::ReadFile)?;
        fs::create_dir_all(app_dir.join("data/log")).map_err(ConfigError::ReadFile)?;

        let default_cfg = build_default_configs(&app_dir);
        let cfg_content =
            toml::to_string_pretty(&default_cfg).map_err(ConfigError::SerializeToml)?;
        fs::write(&config_path, cfg_content).map_err(ConfigError::ReadFile)?;
    }

    Ok(config_path)
}

fn resolve_home_dir() -> Result<PathBuf, ConfigError> {
    if let Ok(home) = env::var("HOME") {
        return Ok(PathBuf::from(home));
    }

    if let Ok(home) = env::var("USERPROFILE") {
        return Ok(PathBuf::from(home));
    }

    Err(ConfigError::ReadFile(io::Error::new(
        io::ErrorKind::NotFound,
        "cannot resolve home directory from HOME/USERPROFILE",
    )))
}

fn normalize_config_paths(cfg: &mut Configs, config_path: &Path) {
    cfg.database.link = normalize_sqlite_link(&cfg.database.link, config_path);
}

fn normalize_sqlite_link(link: &str, config_path: &Path) -> String {
    const PREFIX: &str = "sqlite://";
    if !link.starts_with(PREFIX) {
        return link.to_string();
    }

    let sqlite_target = &link[PREFIX.len()..];
    let (raw_path, raw_query) = match sqlite_target.split_once('?') {
        Some((path, query)) => (path, Some(query)),
        None => (sqlite_target, None),
    };

    let db_path = Path::new(raw_path);
    if db_path.is_absolute() {
        return link.to_string();
    }

    let base_dir = config_path.parent().unwrap_or_else(|| Path::new("."));
    let absolute_db_path = base_dir.join(db_path);
    let absolute_db_path = absolute_db_path.to_string_lossy().replace('\\', "/");

    match raw_query {
        Some(query) => format!("{PREFIX}{absolute_db_path}?{query}"),
        None => format!("{PREFIX}{absolute_db_path}"),
    }
}

fn apply_env_overrides(cfg: &mut Configs) -> Result<(), ConfigError> {
    if let Ok(address) = env::var("SERVER_ADDRESS") {
        cfg.server.address = address;
    }
    if let Ok(api_prefix) = env::var("SERVER_API_PREFIX") {
        cfg.server.api_prefix = api_prefix;
    }
    if let Ok(database_link) = env::var("DATABASE_LINK") {
        cfg.database.link = database_link;
    }
    if let Ok(jwt_secret) = env::var("JWT_SECRET") {
        cfg.jwt.jwt_secret = jwt_secret;
    }
    if let Ok(jwt_exp) = env::var("JWT_EXP") {
        cfg.jwt.jwt_exp = jwt_exp.parse().map_err(|_| ConfigError::InvalidNumber {
            key: "JWT_EXP",
            value: jwt_exp,
        })?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_jwt_exp_error_when_invalid_number() {
        let mut cfg = Configs {
            server: crate::Server {
                name: "name".into(),
                version: "v1".into(),
                address: "127.0.0.1:3000".into(),
                ssl: false,
                content_gzip: false,
                cache_time: 0,
                cache_method: 0,
                api_prefix: "/api".into(),
            },
            web: crate::Web {
                dir: "web".into(),
                index: "index.html".into(),
                upload_dir: "upload".into(),
                upload_url: "/upload".into(),
            },
            cert: crate::Cert {
                cert: "cert".into(),
                key: "key".into(),
            },
            system: crate::System {
                super_user: vec![],
                user_agent_parser: "parser".into(),
            },
            database: crate::Database {
                link: "sqlite://data/app.db?mode=rwc".into(),
            },
            jwt: crate::Jwt {
                jwt_secret: "secret".into(),
                jwt_exp: 10,
            },
            log: crate::Log {
                log_level: "INFO".into(),
                dir: "log".into(),
                file: "app".into(),
                enable_oper_log: false,
            },
        };

        std::env::set_var("JWT_EXP", "abc");
        let result = apply_env_overrides(&mut cfg);
        std::env::remove_var("JWT_EXP");

        assert!(matches!(result, Err(ConfigError::InvalidNumber { .. })));
    }
}
