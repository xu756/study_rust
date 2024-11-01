use crate::db::DbClient;
use crate::model::users;
use sea_orm::sqlx::types::chrono;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DbErr};

impl DbClient {
    pub async fn add_user(&self, user_name: String) -> Result<users::ActiveModel, DbErr> {
        users::ActiveModel {
            username: Set(user_name.to_owned()),
            mobile: Set("18888888818".to_owned()),
            created_at: Set(chrono::Local::now().timestamp()),
            ..Default::default()
        }.save(&self.db).await
    }
}


#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_add_user() {
        let db = crate::db::DbClient::connect().await;
        match db {
            Ok(c) => {
                let r = c.add_user(
                    "test".to_string(),
                ).await;
                if r.is_ok() {
                    println!("1{:?}", r.unwrap());
                } else {
                    println!("2{:?}", r.err());
                }
            }
            Err(e) => {
                println!("{:?}", e.msg);
            }
        }
    }
}