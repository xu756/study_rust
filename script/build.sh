
cargo build --release


cross build --release --target aarch64-unknown-linux-gnu

kickpi scp ./target/aarch64-unknown-linux-gnu/release/app
