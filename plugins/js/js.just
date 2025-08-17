

default:
    @just --list js


build:
    bun build ./src/index.ts --outdir ./dist --external='iced:app'
    jco componentize ./dist/index.js \
        --wit ../../wit/ \
        --world-name app \
        --out js-app.wasm \
        --disable all  

gen:
    jco guest-types ../../wit \
        --out-dir ./src/generated/ \