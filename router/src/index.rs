use axum::Json;
use axum::Router;
use axum::routing::get;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use common::result::{error, success};

pub fn index_router() -> Router {
    Router::new().route("/", get(index_handler))
}


// 路由处理函数
async fn index_handler(user: Option<Json<Value>>) -> Json<Value> {
    if let Some(user) = user {
        success(user)
    } else {
        error(1001, "user not found")
    }
}


// 结构体定义
#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u8,
}

impl Default for User {
    fn default() -> Self {
        User {
            name: "default".to_string(),
            age: 18,
        }
    }
}
