use crate::error::{db_error, CodeError};
use  sea_orm::*;
use std::time::Duration;


pub async fn connect() -> Result<DbConn, CodeError> {
    let mut opt = ConnectOptions::new(config::CFG.database.link.clone());
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8)) // 连接超时时间
            .acquire_timeout(Duration::from_secs(8))    // 获取连接超时时间
            .idle_timeout(Duration::from_secs(8))   // 空闲连接超时时间
            .max_lifetime(Duration::from_secs(8))   // 连接最大生命周期
            .sqlx_logging(true) // 开启日志
            .sqlx_logging_level(log::LevelFilter::Info);  // 日志级别
        // .set_schema_search_path("my_schema"); // 设置schema

        let db = Database::connect(config::CFG.database.link.clone()).await;
        match db {
            Ok(c) => {
                Ok(c)
            }
            Err(e) => {
                Err(db_error(&format!("连接数据库错误 {:?}", e)))
            }
        }
    
}

#[cfg(test)]
mod tests {
    use super::*;


    #[tokio::test]
    async fn test_connect() {
        let db: Result<DbConn, CodeError> = connect().await;
        match db {
            Ok(c) => {
                println!("{:?}", c);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}
