
default:
    @just --list igloo-py

# Generate Python bindings from WIT
gen:
    rm -rf src/wit_world
    uv run componentize-py --wit-path ../../crates/igloo/wit bindings src
    # Move componentize-py support files into igloo_py/ to keep them in sync
    rm -rf src/igloo_py/componentize_py_async_support
    mv src/componentize_py_async_support src/igloo_py/
    mv -f src/componentize_py_types.py src/igloo_py/
    mv -f src/poll_loop.py src/igloo_py/
    mv -f src/componentize_py_runtime.pyi src/igloo_py/

# Type check with pyrefly
check:
    uv run pyrefly check --project-excludes '**/wit_world/**' --project-excludes '**/poll_loop.py' --project-excludes '**/componentize_py_async_support/**' --project-excludes '**/componentize_py_types.py'

# Format with ruff
fmt:
    uv run ruff format src/

# Lint with ruff
lint:
    uv run ruff check src/
