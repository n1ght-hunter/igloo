

build:build-guest


build-guest:
    cargo build -p rust_guest --target wasm32-wasip2 -r


run: build run-host

run-host:
    cargo run -p rust_host