use crate::error::{db_error, CodeError};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

pub struct DbClient {
    pub client: DatabaseConnection,
}


impl DbClient {
    pub async fn connect() -> Result<DbClient, CodeError> {
        let mut opt = ConnectOptions::new(config::CFG.database.link.clone());
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info);
        // .set_schema_search_path("my_schema"); // Setting default PostgreSQL schema

        let db = Database::connect(opt).await;
        match db {
            Ok(c) => {
                Ok(DbClient {
                    client: c
                })
            }
            Err(e) => {
                Err(db_error(&format!("connect error: {:?}", e)))
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
                assert!(true);
            }
            Err(e) => {
                println!("{:?}", e.msg);
            }
        }
    }
}