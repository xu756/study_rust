use super::cfgs::Configs;
use dotenv::dotenv;
use once_cell::sync::Lazy;
use std::env;
use std::{fs::File, io::Read};

//  只要是配置文件中的配置项，都可以通过这个结构体来获取，
// 只要读取一次值后保存到内存，一直可供使用
pub static CFG: Lazy<Configs> = Lazy::new(Configs::init);

impl Configs {
    pub fn init() -> Self {
        dotenv().ok();
        match env::var("CONFIG_PATH") {
            Ok(path) => {
                let mut file = match File::open(&path) {
                    Ok(f) => f,
                    Err(e) => panic!("不存在配置文件：{}，错误信息：{}", path, e),
                };
                let mut cfg_contents = String::new();
                match file.read_to_string(&mut cfg_contents) {
                    Ok(s) => s,
                    Err(e) => panic!("读取配置文件失败，错误 信息：{}", e),
                };
                toml::from_str(&cfg_contents).expect("解析配置文件错误")
            }
            Err(e) => panic!("读取环境变量失败，错误信息：{}", e),
        }
    }

    pub fn get_config() -> Configs {
        Configs::init()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_config() {
        let cfg = Configs::init();
        println!("{:?}", cfg);
    }
}