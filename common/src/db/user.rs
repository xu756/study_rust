use crate::entity::user;
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryOrder};

#[derive(Debug, Clone)]
pub struct CreateUserInput {
    pub name: String,
    pub email: String,
}

pub async fn create_user(
    db: &DatabaseConnection,
    input: CreateUserInput,
) -> Result<user::Model, sea_orm::DbErr> {
    let active_model = user::ActiveModel {
        name: Set(input.name),
        email: Set(input.email),
        created_at: Set(Utc::now().timestamp()),
        ..Default::default()
    };
    active_model.insert(db).await
}

pub async fn list_users(db: &DatabaseConnection) -> Result<Vec<user::Model>, sea_orm::DbErr> {
    user::Entity::find()
        .order_by_desc(user::Column::Id)
        .all(db)
        .await
}

pub async fn find_user_by_id(
    db: &DatabaseConnection,
    id: i64,
) -> Result<Option<user::Model>, sea_orm::DbErr> {
    user::Entity::find_by_id(id).one(db).await
}
