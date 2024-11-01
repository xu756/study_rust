use crate::error::*;
use crate::model::prelude::*;
use crate::model::{self, user::*};
use prelude::Expr;
use sea_orm::*;
use sqlx::types::chrono;

/// 用户模型
pub struct UserModel {}
impl UserModel {
    /// 获取用户信息
    /// - Ok(users::Model) 用户信息
    pub async fn get_user_by_id(db: &DbConn, id: i32) -> Result<Model, CodeError> {
        let user = User::find_by_id(id).one(db).await;
        match user {
            Ok(u) => match u {
                Some(u) => Ok(u),
                None => Err(db_error(&format!("用户【{}】不存在", id))),
            },
            Err(e) => Err(db_error(&format!("获取用户【{}】信息失败: {:?}", id, e))),
        }
    }
    /// 添加用户
    /// - Ok(bool) 是否添加成功
    pub async fn add_user(db: &DbConn, user: ActiveModel) -> Result<bool, CodeError> {
        // now time
        match user.save(db).await {
            Ok(_) => Ok(true),
            Err(e) => Err(db_error(&format!("添加用户失败: {:?}", e))),
        }
    }
    /// 更新用户
    /// - Ok(bool) 是否更新成功
    pub async fn update_user(db: &DbConn, id: i32, new_user: Model) -> Result<bool, CodeError> {
        let res = User::update_many()
            .col_expr(Column::Username, Expr::value(new_user.username))
            .col_expr(Column::Mobile, Expr::value(new_user.mobile))
            .filter(Column::Id.eq(id))
            .exec(db)
            .await;
        match res {
            Ok(_) => Ok(true),
            Err(e) => Err(db_error(&format!("更新用户失败: {:?}", e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connect::*;

    #[tokio::test]
    async fn test_get_user_by_id() {
        let db: sea_orm::DbConn = connect().await.unwrap();
        let user: Result<Model, CodeError> = UserModel::get_user_by_id(&db, 3).await;
        match user {
            Ok(u) => {
                println!("{:?}", u);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_add_user() {
        let db: sea_orm::DbConn = connect().await.unwrap();
        let res: Result<bool, CodeError> = UserModel::add_user(
            &db,
            ActiveModel {
                username: Set("test".to_string()),
                mobile: Set("123456789011".to_string()),
                created_at: Set(chrono::Utc::now().timestamp()),
                ..Default::default()
            },
        )
        .await;
        if let Err(e) = res {
            print!("{:?}", e);
        }
        print!("创建用户成功");
    }

    #[tokio::test]
    async fn test_update_user() {
        let db: sea_orm::DbConn = connect().await.unwrap();
        let res: Result<bool, CodeError> = UserModel::update_user(
            &db,
            3,
            Model {
                username: "test".to_string(),
                mobile: "123456789011".to_string(),
                id: 3,
                created_at: todo!(),
                updated_at: todo!(),
                deleted: todo!(),
                editor: todo!(),
                password: todo!(),
                avatar: todo!(),
                desc: todo!(),
            },
        )
        .await;
        if let Err(e) = res {
            print!("{:?}", e);
        }
        print!("更新用户成功");
    }
}

#[inline]
pub fn default<T: Default>() -> T {
    std::default::Default::default()
}
