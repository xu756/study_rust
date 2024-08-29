use std::net::SocketAddr;

use axum::Router;
use axum::routing::get;

mod index;

pub async fn init_router() {
    // 创建一个新的路由
    let app = Router::new()
        .route("/api/", get(root))
        .nest("/api/index", index::index_router());

    // 绑定地址并启动服务器
    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 8989));

    println!("启动web服务在 http://{}/", addr);
    println!("按下 Ctrl+C 停止服务");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn root() -> &'static str {
    "Hello, World!"
}