use crate::common::result::{success, Response};
use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
pub async fn init_router() {
    // 初始化路由
    let app = Router::new().route("/", get(index_handler));

    // 绑定地址并启动服务器
    let addr = SocketAddr::from(([0, 0, 0, 0], 8989));
    println!("Server is running on http://{}/", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 结构体定义
#[derive(Serialize)]
struct User {
    name: String,
    age: u8,
}

// 路由处理函数
async fn index_handler() -> Json<Response<User>> {
    let user = User {
        name: "Jack".to_string(),
        age: 18,
    };
    success(user)
}
