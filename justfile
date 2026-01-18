mod js "plugins/js/js.just"
mod rsx "examples/rust_host/rsx.just"
mod igloo-ts "packages/igloo-ts/igloo-ts.just"

default:
    @just --list


setup:
    rustup target add wasm32-wasip2



run:
    @just rsx run

gen:
    just igloo-ts gen
    just js gen