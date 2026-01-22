
default:
    @just --list igloo-py

# Generate Python bindings from WIT
gen:
    rm -rf src/igloo_py/generated
    uv run componentize-py --wit-path ../../wit bindings src/igloo_py/generated

# Type check with pyrefly
check:
    uv run pyrefly check --project-excludes '**/generated/**'

# Format with ruff
fmt:
    uv run ruff format src/

# Lint with ruff
lint:
    uv run ruff check src/
