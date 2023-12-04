mod router;
mod study;
use router::init_router;

use std::io::{Read, Write};
#[tokio::main]
async fn main() {
    init_router().await;
}
