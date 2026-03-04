use crate::state::AppState;
use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use common::db::user::{create_user, find_user_by_id, list_users, CreateUserInput};
use common::error::db_error;
use common::result::{error, param_error, success};
use serde::Deserialize;
use serde_json::Value;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users_handler).post(create_user_handler))
        .route("/{id}", get(get_user_handler))
}

#[derive(Debug, Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

async fn list_users_handler(State(state): State<AppState>) -> Json<Value> {
    match list_users(&state.db).await {
        Ok(users) => success(users),
        Err(err) => error(db_error(&err.to_string())),
    }
}

async fn create_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Json<Value> {
    if payload.name.trim().is_empty() {
        return param_error("name 不能为空");
    }
    if payload.email.trim().is_empty() {
        return param_error("email 不能为空");
    }

    let input = CreateUserInput {
        name: payload.name.trim().to_string(),
        email: payload.email.trim().to_string(),
    };
    match create_user(&state.db, input).await {
        Ok(user) => success(user),
        Err(err) => error(db_error(&err.to_string())),
    }
}

async fn get_user_handler(State(state): State<AppState>, Path(id): Path<i64>) -> Json<Value> {
    match find_user_by_id(&state.db, id).await {
        Ok(Some(user)) => success(user),
        Ok(None) => param_error("user 不存在"),
        Err(err) => error(db_error(&err.to_string())),
    }
}
