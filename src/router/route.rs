use axum::Router;
use std::net::SocketAddr;

use super::index::index_router;

pub async fn init_router() {
    // 创建一个新的路由
    let app = Router::new()
        .nest("/api/index", index_router())
        .nest("/home", index_router());

    // 绑定地址并启动服务器
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 8989));

    println!("启动web服务在 http://{}/", addr);
    println!("按下 Ctrl+C 停止服务");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
