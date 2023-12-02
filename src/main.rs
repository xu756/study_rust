
// main.rs
mod router;

#[tokio::main]
async fn main() {
    router::init_router().await;
}

// router/route.rs