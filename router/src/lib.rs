use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;

mod index;

pub async fn init_router() {
    // 创建一个新的路由
    let app = Router::new()
        .route("/api/", get(root))
        .nest("/api/index", index::index_router());
    let app = app.fallback(not_found);
    let addr: SocketAddr = config::CFG.server.address.parse().unwrap();
    println!("启动web服务在 http://{}/", addr);
    println!("按下 Ctrl+C 停止服务");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

// not_found
async fn not_found() -> &'static str {
    "404"
}