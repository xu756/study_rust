use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;


pub fn init_router() {
    let app = route("/", post(handler)); // http://127.0.0.1:3000

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // run it with hyper on localhost:3000
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}



// 处理器
async fn handler() -> &'static str {
    "Hello, World!"
}