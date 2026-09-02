
<p align="center">
    <img src="assets/igloo.png" alt="Igloo Logo" width="120" style="border-radius: 15px;" />
</p>


# Igloo: WebAssembly Plugin System for Iced

Igloo is a modular plugin system for GUI applications, built with Rust, Iced, and the WebAssembly Component Model. It enables dynamic loading of UI components as secure, sandboxed WebAssembly plugins, making it easy to extend and customize desktop applications.

## Overview


Igloo lets you:
- Build desktop GUIs with plugins written in Rust or other WASM-compatible languages
- Use WIT (WebAssembly Interface Types) for type-safe host/guest communication
- Run plugins in a secure, isolated environment

- **Host Application**: An Iced-based GUI application that manages and renders WebAssembly plugins
- **Guest Plugins**: WebAssembly components that define UI elements and behavior
- **WIT (WebAssembly Interface Types)**: For type-safe communication between host and guest
- **Component Model**: For secure, sandboxed plugin execution

## Features

- **Type-Safe Plugin Communication**: WIT-based interfaces for host/guest messaging
- **Rich UI Components**: Buttons, text, containers, layouts, and more
- **Message Passing**: Bidirectional communication between host and plugins
- **Resource Management**: Efficient handling of UI elements across WASM boundary
- **Plugin Isolation**: Secure sandboxed execution of plugin code
- **Multi-Language Support**: Rust, TypeScript/JavaScript, and Python

## Todo
- Add iced canvas support
- Add cwasm plugin support and precompilation guide
- Auto cache plugins by compiling then storing compiled version and using hash to check for changes
- Figure out how to hook in iced tasks and subscriptions for async operations

## Getting Started

### Prerequisites

- Rust and rustup installed
- [mise](https://mise.jdx.dev/) installed (provides all other tooling and task running)


### Setup

1. Clone the repository:
```bash
git clone https://github.com/n1ght-hunter/igloo.git
cd igloo
```

2. Install required Rust target and tools:
```bash
mise run setup
# or manually:
rustup target add wasm32-wasip2
mise install
```

3. Build plugins and run the example:
```bash
# Build all plugins (Rust, TypeScript, Python) and run
mise run build-plugins
mise run run

# Or build individual plugins:
mise run //plugins/rust:build     # Build Rust plugin
mise run //plugins/js:build       # Build TypeScript/JavaScript plugin
mise run //plugins/python:build   # Build Python plugin
```

Run `mise tasks ls --all` to see every available task.

### Language SDKs

Igloo supports plugins written in multiple languages:

| Language | SDK | Status |
|----------|-----|--------|
| Rust | `igloo_guest` | Handwritten |
| TypeScript/JavaScript | `igloo-ts` | Generated with Claude code, manually tested |
| Python | `igloo_py` | Generated with Claude code, works but not checked much |

### Creating a Rust Plugin

1. Create a new Rust library with `crate-type = ["cdylib"]`
2. Add `igloo_guest` as a dependency
3. Implement the `Application` trait:

```rust
use igloo_guest::{
    Element, Task,
    widgets::{button, column, text},
};

#[derive(Debug, Clone)]
pub enum MyMessage {
    ButtonPressed,
    // ... other messages
}

pub struct MyPlugin {
    counter: u32,
}

impl igloo_guest::Application for MyPlugin {
    type Message = MyMessage;

    /// Build the initial state and the task to run on startup.
    fn new() -> (Self, Task<MyMessage>) {
        (Self { counter: 0 }, Task::none())
    }

    fn view(&self) -> Element<MyMessage> {
        column()
            .push(text(format!("Count: {}", self.counter)))
            .push(button(text("Click me")).on_press(MyMessage::ButtonPressed))
            .into()
    }

    fn update(&mut self, message: MyMessage) -> Task<MyMessage> {
        match message {
            MyMessage::ButtonPressed => {
                self.counter += 1;
                Task::none()
            }
        }
    }
}

// Export the plugin
igloo_guest::export_guest!(MyPlugin);
```

4. Compile to WASM:
```bash
cargo build --target wasm32-wasip2 --release
```

### Creating a TypeScript Plugin

```typescript
import { App, Text, Column, Button, MessageManager, ElementLike } from "igloo-ts";

type State = { count: number };
type Msg = "increment" | "decrement";

class CounterApp implements App<State, Msg> {
  init(): State {
    return { count: 0 };
  }

  update(state: State, msg: Msg): State {
    switch (msg) {
      case "increment": return { count: state.count + 1 };
      case "decrement": return { count: state.count - 1 };
    }
  }

  view(state: State, messages: MessageManager<Msg>): ElementLike {
    return Column.new()
      .push(Text.new(`Count: ${state.count}`))
      .push(Button.new(Text.new("+")).onPress(messages, () => "increment"))
      .push(Button.new(Text.new("-")).onPress(messages, () => "decrement"));
  }
}

export const { update, view } = createApp(new CounterApp());
```

### Creating a Python Plugin

```python
from igloo_py import App, igloo_app, Text, Column, Button, MessageManager, ElementLike

@igloo_app
class CounterApp(App[str]):
    def __init__(self):
        self.count = 0

    def update(self, msg: str) -> None:
        if msg == "increment":
            self.count += 1
        elif msg == "decrement":
            self.count -= 1

    def view(self, messages: MessageManager[str]) -> ElementLike:
        return Column.new().push(
            Text.new(f"Count: {self.count}")
        ).push(
            Button.new(Text.new("+")).on_press(messages, lambda: "increment")
        ).push(
            Button.new(Text.new("-")).on_press(messages, lambda: "decrement")
        )

```

### Loading Plugins in Host

```rust
use igloo::plugin_manager::PluginManager;

let mut plugin_manager = PluginManager::new()?;
plugin_manager.add_plugin_from_file("my-plugin", "path/to/plugin.wasm")?;

// In your update loop: returns an `iced::Task` the plugin asked to run.
let task = plugin_manager.plugin_update("my-plugin", message)?;

// In your view:
let plugin_view = plugin_manager.plugin_view("my-plugin")?;
```

## Development

### Building

- Build all components: `cargo build`
- Build for WASM target: `cargo build --target wasm32-wasip2`
- Run example: `mise run run`
- Generate bindings: `mise run //packages/igloo-ts:gen` and `mise run //packages/igloo-py:gen`

Warning: Loading plugins can be slow during development due to Wasmtime compilation. 
Consider adding this to your `Cargo.toml` for faster builds or running in release mode:

```toml
# Optimize wasmtime/cranelift in dev builds for faster WASM compilation
[profile.dev.package.wasmtime]
opt-level = 3

[profile.dev.package.cranelift-codegen]
opt-level = 3

[profile.dev.package.regalloc2]
opt-level = 3
```
or precompile your plugins using `wasmtime compile` like so:
```bash
wasmtime compile --target wasm32-wasip2 path/to/plugin.wasm -o path/to/plugin.cwasm
```

## Acknowledgments

- [Iced](https://github.com/iced-rs/iced) - GUI framework
- [Wasmtime](https://github.com/bytecodealliance/wasmtime) - WebAssembly runtime
- [wit-bindgen](https://github.com/bytecodealliance/wit-bindgen) - Interface generation
- WebAssembly Component Model specification

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.