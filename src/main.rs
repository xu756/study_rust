#![allow(dead_code)]
#![allow(unused_variables)]
use study_rust::router;

use router::route::init_router;

#[tokio::main]
async fn main() {
    init_router().await;
}
