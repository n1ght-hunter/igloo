use std::{collections::HashMap, ops::DerefMut, path::{Path, PathBuf}, sync::Arc};

use crate::{
    bindings::App,
    widgets::{Message, ToElement, WrapperRenderer, WrapperTheme},
};
use tracing::info;
use wasmtime::{
    Config, Engine, Store,
    component::{Component, HasSelf, Linker},
};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView, p2::add_to_linker_sync};

pub struct MyState {
    wasi: WasiCtx,
    pub table: ResourceTable,
}

impl std::fmt::Debug for MyState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MyState")
            .field("wasi", &"wasi")
            .field("table", &self.table)
            .finish()
    }
}

impl WasiView for MyState {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi,
            table: &mut self.table,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    #[error("Plugin not found: {0}")]
    NotFound(String),
    #[error("Failed to update plugin: {0}")]
    UpdateFailed(String),
    #[error("Wasm error: {0}")]
    WasmError(#[from] wasmtime::Error),
}

type Result<T> = std::result::Result<T, PluginError>;

pub struct PluginManager {
    store: std::cell::RefCell<Store<MyState>>,
    engine: Engine,
    linker: Linker<MyState>,
    plugins: HashMap<String, App>,
}

impl std::fmt::Debug for PluginManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PluginManager")
            .field("plugins", &self.plugins.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl PluginManager {
    pub fn new() -> Result<Self> {
        // Construct the wasm engine with async support enabled.
        let engine = Engine::new(Config::new().wasm_component_model(true))?;

        let mut linker = Linker::new(&engine);
        App::add_to_linker::<_, HasSelf<_>>(&mut linker, |s| s)?;
        add_to_linker_sync(&mut linker)?;

        // Add capabilities (e.g. filesystem access) to the WASI preview2 context
        // here. Here only stdio is inherited, but see docs of `WasiCtxBuilder` for
        // more.
        let wasi_ctx = WasiCtxBuilder::new()
            .inherit_stderr()
            .inherit_stdout()
            .build();

        let store = Store::new(
            &engine,
            MyState {
                wasi: wasi_ctx,
                table: ResourceTable::new(),
            },
        );

        Ok(Self {
            store: std::cell::RefCell::new(store),
            engine,
            linker,
            plugins: HashMap::new(),
        })
    }

    pub fn ids(&self) -> Vec<String> {
        self.plugins.keys().cloned().collect()
    }

    /// Adds a plugin from a file.
    pub fn add_plugin_from_file(
        &mut self,
        name: impl Into<String>,
        file: impl AsRef<Path>,
    ) -> Result<()> {
        // Create our component and call our generated host function.
        self.raw_add(name.into(), |engine| {
            Ok(Component::from_file(engine, file)?)
        })
    }

    pub fn plugin_update(&mut self, id: &str, msg: Message) -> Result<()> {
        if let Some(plugin) = self.plugins.get_mut(id) {
            let mut store = self.store.borrow_mut();
            let Message { id, content } = msg;
            Ok(plugin.call_update(store.deref_mut(), id, &content)?)
        } else {
            Err(PluginError::NotFound(id.into()))
        }
    }

    pub fn plugin_view<'a, Theme, Renderer>(
        &self,
        id: &str,
    ) -> Option<iced::Element<'a, Message, Theme, Renderer>>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let plugin = self.plugins.get(id)?;

        let mut store = self.store.borrow_mut();
        let result = plugin
            .call_view(store.deref_mut())
            .inspect_err(|e| {
                tracing::error!("Failed to call view for plugin {}: {}", id, e);
            })
            .ok()?;
        let element = store
            .data_mut()
            .table
            .delete(result)
            .inspect_err(|e| {
                tracing::error!("Failed to delete element for plugin {}: {}", id, e);
            })
            .ok()?;
        Some(element.to_element(&mut store.data_mut().table))
    }

    /// Adds a plugin from pre-loaded bytes.
    pub fn add_plugin_from_bytes(&mut self, name: &str, bytes: &[u8]) -> Result<()> {
        let component = Component::from_binary(&self.engine, bytes)?;
        let app = App::instantiate(self.store.get_mut(), &component, &self.linker)?;
        if self.plugins.insert(name.to_string(), app).is_some() {
            info!("Replaced existing plugin: {}", name);
        }
        Ok(())
    }

    /// Adds a pre-compiled plugin component.
    pub fn add_compiled_plugin(&mut self, plugin: CompiledPlugin) -> Result<()> {
        let app = App::instantiate(self.store.get_mut(), &plugin.component, &self.linker)?;
        if self.plugins.insert(plugin.name.clone(), app).is_some() {
            info!("Replaced existing plugin: {}", plugin.name);
        }
        Ok(())
    }

    /// Returns a clone of the engine for use in async plugin compilation.
    pub fn engine(&self) -> Engine {
        self.engine.clone()
    }

    #[doc(hidden)]
    pub fn raw_add<F: FnOnce(&Engine) -> Result<Component>>(
        &mut self,
        id: String,
        component: F,
    ) -> Result<()> {
        let component = component(&self.engine)?;
        let app = App::instantiate(self.store.get_mut(), &component, &self.linker)?;
        if self.plugins.insert(id.clone(), app).is_some() {
            info!("Replaced existing plugin: {}", id);
        }

        Ok(())
    }
}

/// A compiled plugin ready to be instantiated.
#[derive(Clone)]
pub struct CompiledPlugin {
    pub name: String,
    pub component: Arc<Component>,
}

impl std::fmt::Debug for CompiledPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompiledPlugin")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

/// Asynchronously load and compile a plugin from a file.
/// File reading and compilation are done in parallel background tasks.
pub async fn load_and_compile_plugin_async(
    engine: Engine,
    name: String,
    path: PathBuf,
) -> std::result::Result<CompiledPlugin, String> {
    let bytes = tokio::fs::read(&path)
        .await
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

    // Compile in a blocking task to not block the async runtime
    let component = tokio::task::spawn_blocking(move || Component::from_binary(&engine, &bytes))
        .await
        .map_err(|e| format!("Task join error for {}: {}", name, e))?
        .map_err(|e| format!("Compilation error for {}: {}", name, e))?;

    Ok(CompiledPlugin {
        name,
        component: Arc::new(component),
    })
}
