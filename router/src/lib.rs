use axum::Router;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

mod routes;
mod state;

pub async fn init_router() -> Result<(), Box<dyn std::error::Error>> {
    let db = common::db::connect::connect_database(&config::CFG.database.link).await?;
    let state = state::AppState::new(db);

    let api_prefix = config::CFG.server.api_prefix.as_str();
    let app = Router::new()
        .route("/", axum::routing::get(root))
        .nest(api_prefix, routes::api_router(state))
        .fallback(not_found)
        .layer(CorsLayer::permissive());

    let addr: SocketAddr = config::CFG.server.address.parse()?;
    println!("启动 web 11服务: http://{addr}");
    println!("API 前缀: {api_prefix}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn root() -> &'static str {
    "Rust API Server is running"
}

async fn not_found() -> &'static str {
    "404 not found"
}
