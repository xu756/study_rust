use std::fs::File;

// indexRouter
use crate::common::result::{success, Response};
use axum::{routing::post, Json, Router};

use serde::{Deserialize, Serialize};

pub fn index_router() -> Router {
    Router::new().route("/", post(index_handler))
}

// 路由处理函数
async fn index_handler(Json(params): Json<User>) -> Json<Response<User>> {
    let user = User {
        name: params.name,
        age: 18,
    };
    success(user)
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

// 测试
#[tokio::test]
async fn test_index_handler() {}
