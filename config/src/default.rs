use crate::{Cert, Configs, Database, Jwt, Log, Server, System, Web};
use std::path::Path;

/// Build strongly-typed default configuration values.
///
/// This is the single source of truth for generated `config.toml` defaults.
pub fn build_default_configs(app_dir: &Path) -> Configs {
    let web_dir = normalize_path(&app_dir.join("data/_web"));
    let upload_dir = normalize_path(&app_dir.join("data/_upload"));
    let log_dir = normalize_path(&app_dir.join("data/log"));
    let cert_path = normalize_path(&app_dir.join("config/cert/cert.pem"));
    let cert_key_path = normalize_path(&app_dir.join("config/cert/key.pem"));
    let regexes_path = normalize_path(&app_dir.join("config/regexes.yaml"));
    let sqlite_path = normalize_path(&app_dir.join("data/app.db"));

    Configs {
        server: Server {
            name: "axum-admin".to_string(),
            version: "0.1.0".to_string(),
            address: "0.0.0.0:3000".to_string(),
            ssl: false,
            content_gzip: true,
            cache_time: 600,
            cache_method: 1,
            api_prefix: "/api".to_string(),
        },
        web: Web {
            dir: web_dir,
            index: "index.html".to_string(),
            upload_dir,
            upload_url: "/upload".to_string(),
        },
        cert: Cert {
            cert: cert_path,
            key: cert_key_path,
        },
        system: System {
            super_user: vec![],
            user_agent_parser: regexes_path,
        },
        database: Database {
            link: format!("sqlite://{sqlite_path}?mode=rwc"),
        },
        jwt: Jwt {
            jwt_secret: "4tfLEJpyNVMkh59ZNfCr".to_string(),
            jwt_exp: 14400,
        },
        log: Log {
            log_level: "DEBUG".to_string(),
            dir: log_dir,
            file: "app_log".to_string(),
            enable_oper_log: true,
        },
    }
}

fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
