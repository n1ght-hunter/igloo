use std::{collections::HashMap, ops::DerefMut, path::Path};

use crate::{
    bindings::App,
    widgets::{Message, ToElement, WrapperRenderer, WrapperTheme},
};
use tracing::info;
use wasmtime::{
    Config, Engine, Store,
    component::{Component, HasSelf, Linker},
};
use wasmtime_wasi::{
    ResourceTable,
    p2::{IoView, WasiCtx, WasiCtxBuilder, WasiView, add_to_linker_sync},
};

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

impl IoView for MyState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl WasiView for MyState {
    fn ctx(&mut self) -> &mut wasmtime_wasi::p2::WasiCtx {
        &mut self.wasi
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
