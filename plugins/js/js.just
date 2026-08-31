
default:
    @just --list js

bundle:
    pnpm exec rolldown -c rolldown.config.mjs

check:
    pnpm exec tsc --noEmit

componentize:
    jco componentize ./dist/index.js \
        --wit ../../crates/igloo/wit/ \
        --world-name app \
        --out js-app.wasm \
        --disable all \
        --enable stdio \
        --debug-starlingmonkey-build

build: bundle componentize