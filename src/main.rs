#![allow(dead_code)]
#![allow(unused_variables)]
mod router;
mod study;
use router::init_router;

#[tokio::main]
#[allow(dead_code)]
async fn main() {
    init_router().await;
}
