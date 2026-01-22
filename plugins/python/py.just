
default:
    @just --list py

# Componentize Python to WASM
componentize:
    uv run componentize-py \
        --wit-path ../../wit \
        --world app \
        componentize \
        -p src \
        -p ../../packages/igloo-py/src \
        -o python-app.wasm \
        task_manager

build: componentize
