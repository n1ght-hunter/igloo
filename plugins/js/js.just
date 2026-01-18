
default:
    @just --list js

bundle:
    bun build ./src/index.ts --outdir ./dist --external='iced:app'

componentize:
    jco componentize ./dist/index.js \
        --wit ../../wit/ \
        --world-name app \
        --out js-app.wasm \
        --disable all \
        --enable stdio \
        --debug-starlingmonkey-build

build: bundle componentize