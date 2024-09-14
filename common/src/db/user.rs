use crate::db::DbClient;
use crate::error::{db_error, CodeError};
use crate::model::users;
use sea_orm::sqlx::types::chrono;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel};


impl DbClient {
    pub async fn add_user(&self, user_name: String) -> Result<bool, CodeError> {
        let user = users::ActiveModel {
            username: Set(user_name.to_owned()),
            mobile: Set("18888888888".to_owned()),
            created_at: Set(chrono::Local::now().timestamp()),
            ..Default::default()
        };
        match user.insert(&self.client).await {
            Ok(res) => {
                println!("add user success: {:?}", res);
                Ok(true)
            }
            Err(error) => {
                // match e {  }
                Err(db_error(&format!("add user error: {:?}", error)))
            }
        }
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
                match r {
                    Ok(_) => {
                        println!("add user success");
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