mod user;

use crate::error::{db_error, CodeError};
use sea_orm::{ConnectOptions, Database, DbConn};
use std::time::Duration;

pub struct DbClient {
    pub db: DbConn,
}


impl DbClient {
    /**
     * 连接数据库
     */
    pub async fn connect() -> Result<DbClient, CodeError> {
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
                Ok(DbClient {
                    db: c,
                })
            }
            Err(e) => {
                Err(db_error(&format!("connect error: {:?}", e)))
            }
        }
    }

    /**
     * 关闭数据库连接
     */
    pub async fn close(self) -> Result<bool, CodeError> {
        match self.db.close().await {
            Ok(_) => {
                Ok(true)
            }
            Err(e) => {
                Err(db_error(&format!("close error: {:?}", e)))
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connect() {
        let db = DbClient::connect().await;
        match db {
            Ok(_) => {
                println!("connect success");
                let close = db.unwrap().close().await;
                match close {
                    Ok(_) => {
                        println!("close success")
                    }
                    Err(e) => {
                        println!("{:?}", e.msg);
                    }
                }
            }
            Err(e) => {
                println!("{:?}", e.msg);
            }
        }
    }
}