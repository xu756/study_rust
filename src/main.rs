#![allow(dead_code)]
#![allow(unused_variables)]

use router::route::init_router;
use study_rust::router;

#[tokio::main]
async fn main() {
    init_router().await;
}
