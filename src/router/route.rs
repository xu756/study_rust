use axum::{
    routing::get,
    Router,
    Json,
};
use serde::{Serialize};

// 结构体定义
#[derive(Serialize)]
struct User {
    name: String,
    age: u8,
}
// 初始化路由
pub async fn init_router() {
    let app = Router::new().
        route("/", get(handler));

    // 使用 hyper 在 localhost:8989 上运行服务器
    axum::Server::bind(&"0.0.0.0:8989".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


#[derive(Serialize)]
pub struct Response<T> {
    err_code: u32,
    err_msg: String,
    data: T,
}

pub fn success<T>(data: T) -> Json<Response<T>> {
    let res = Response {
        err_code: 0,
        err_msg: "".to_string(),
        data,
    };
    Json(res)
}


async fn handler() -> Json<Response<User>> {
    let user = User {
        name: "Jack".to_string(),
        age: 18,
    };
    success(user)
}


