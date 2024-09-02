use crate::error::CodeError;

pub struct DbClient {}


impl DbClient {
    pub async fn connect() -> Result<DbClient, CodeError> {
        // 连接数据库
        // 如果连接失败，返回错误
        let db = DbClient {};
        Ok(db)
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
                println!("connect success")
            }
            Err(e) => {
                println!("{:?}", e.msg);
            }
        }
    }
}