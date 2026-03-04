use axum::extract::rejection::JsonRejection;
use axum::routing::post;
use axum::{Json, Router};
use common::result::{param_error, success};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub fn router() -> Router<crate::state::AppState> {
    Router::new().route("/", post(index_handler))
}

async fn index_handler(payload: Result<Json<User>, JsonRejection>) -> Json<Value> {
    match payload {
        Ok(payload) => success(payload.0),
        Err(err) => param_error(&err.to_string()),
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u8,
}
