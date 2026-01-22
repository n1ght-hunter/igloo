mod js "plugins/js/js.just"
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

# Build JS plugin with caching
build-js:
    mise run js:build

# Build Python plugin with caching
build-py:
    mise run python:build