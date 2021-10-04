rustc --print target-list

#rustup target add x86_64-unknown-linux-musl
#cargo build --release --target x86_64-unknown-linux-musl
cargo install cross
cross build --release --target x86_64-unknown-linux-musl
#cross build --release --target x86_64-unknown-linux-gnu


docker build -t gorpher/echo-service-rust:x86_64-unknown-linux-musl .

docker tag gorpher/echo-service-rust:x86_64-unknown-linux-musl gorpher/echo-service-rust:latest