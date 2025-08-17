mod js "plugins/js/js.just"
mod rsx "examples/rust_host/rsx.just"

default:
    @just --list


setup:
    rustup target add wasm32-wasip2



run:
    @just rsx run

gen:
    just js gen