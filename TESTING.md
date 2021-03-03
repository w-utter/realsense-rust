# Testing

Mention multi-threading nature of Rust testing, and how that's a bad thing

## If testing the lib without a device:

`RUST_TEST_THREADS=1 cargo test`

## If testing with a device connected:

`RUST_TEST_THREADS=1 cargo test --all-features`