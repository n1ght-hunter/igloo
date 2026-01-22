
default:
    @just --list rust

build:
    cargo build -p rust_guest --target wasm32-wasip2 -r
