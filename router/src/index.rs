use axum::extract::rejection::JsonRejection;
use axum::routing::get;
use axum::Json;
use axum::Router;
use common::result::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub fn index_router() -> Router {
    Router::new().route("/", get(index_handler))
}


// 路由处理函数
async fn index_handler(payload: Result<Json<User>, JsonRejection>) -> Json<Value> {
    match payload {
        Ok(payload) => {
            // 提取成功
            success(payload.0)
        }
        Err(err) => {
            error(err.to_string().as_str())
        }
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
