mod js "plugins/js/js.just"
mod py "plugins/python/py.just"
mod rust "plugins/rust/rust.just"
mod rsx "examples/rust_host/rsx.just"
mod igloo-ts "packages/igloo-ts/igloo-ts.just"
mod igloo-py "packages/igloo-py/igloo-py.just"

default:
    @just --list

setup:
    rustup target add wasm32-wasip2
    mise install

run:
    @just rsx run

# Build all plugins with caching
build-plugins:
    @mise run js:build
    @mise run python:build
    @just rust build