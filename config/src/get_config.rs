use serde::Deserialize;
use dotenv::dotenv;

#[derive(Debug, Deserialize)]
struct Config {
    // 会读取 DATABASE_URL
    database_url: String,

    // 会读取 PORT；如果缺失，用默认值
    #[serde(default = "default_port")]
    port: u16,

    // 会读取 RUST_LOG；缺失则 None
    rust_log: Option<String>,
}

fn default_port() -> u16 { 8080 }

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    // 从当前目录加载 .env（找不到也不报错就用 dotenvy::dotenv().ok()）
    dotenv::dotenv().ok();

    // 从环境变量反序列化到 Config
    let cfg = envy::from_env::<Config>()?;
    Ok(cfg)
}




// test
#[cfg(test)]
mod tests {
        #[test]
    fn test_load_config() {
        let config = super::load_config().unwrap();
        println!("{:?}", config);
    }
}