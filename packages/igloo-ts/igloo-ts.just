
default:
    @just --list igloo-ts

build:
    mise exec -- bun build ./src/index.ts --outdir ./dist --external='iced:app'

gen:
    mise exec -- bunx jco guest-types ../../wit --out-dir ./src/generated/
