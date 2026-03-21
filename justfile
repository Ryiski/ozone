dev: 
    watchexec -d 1s -e rs,kdl -r cargo run -- -c ./config/config.kdl -v
build:
    cargo build -r && cp ./target/release/ozone .
lint:
  cargo fmt --check
  cargo clippy --workspace --tests
