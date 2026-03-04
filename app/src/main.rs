use router::init_router;

#[tokio::main]
async fn main() {
    if let Err(err) = init_router().await {
        eprintln!("server startup failed: {err}");
        std::process::exit(1);
    }
}
