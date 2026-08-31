
default:
    @just --list igloo-ts

build:
    pnpm exec rolldown -c rolldown.config.mjs

gen:
    rm -rf ./src/generated
    jco guest-types ../../crates/igloo/wit --out-dir ./src/generated/
