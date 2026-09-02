use std::{
    collections::HashMap,
    ops::DerefMut,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{
    bindings::{App, exports::iced::app::app_instance::MessageValue, iced::app::widgets::Node},
    widgets::{Message, WrapperRenderer, WrapperTheme, build_element},
};
use tracing::info;
use wasmtime::{
    Config, Engine, Store,
    component::{Component, HasSelf, Linker},
};
use wasmtime_wasi::{
    ResourceTable, WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView, p2::add_to_linker_sync,
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

/// A loaded plugin: the bindgen world handle plus the `application` resource
/// created for it at instantiation time. The resource is the guest's own state
/// and outlives every `view`/`update` call — it is only dropped when the plugin
/// is replaced or the manager itself goes away.
struct Plugin {
    app: App,
    instance: wasmtime::component::ResourceAny,
}

pub struct PluginManager {
    store: std::cell::RefCell<Store<MyState>>,
    engine: Engine,
    linker: Linker<MyState>,
    plugins: HashMap<String, Plugin>,
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

    /// Instantiates a component and constructs its `application` resource, which
    /// holds the plugin's state for as long as the plugin lives.
    fn instantiate(&mut self, component: &Component) -> Result<(Plugin, iced::Task<Message>)> {
        let app = App::instantiate(self.store.get_mut(), component, &self.linker)?;
        let application = app.iced_app_app_instance().application();
        let instance = application.call_constructor(self.store.get_mut())?;
        let boot = application.call_boot(self.store.get_mut(), instance)?;
        let boot = self
            .store
            .get_mut()
            .data_mut()
            .table
            .delete(boot)
            .map_err(wasmtime::Error::from)?
            .0;
        Ok((Plugin { app, instance }, boot))
    }

    /// Adds a plugin from a file.
    pub fn add_plugin_from_file(
        &mut self,
        name: impl Into<String>,
        file: impl AsRef<Path>,
    ) -> Result<()> {
        self.raw_add(name.into(), |engine| {
            Ok(Component::from_file(engine, file)?)
        })
    }

    /// Sends a message to a plugin's update function.
    ///
    /// Every `Message` variant already carries a plain callback id the guest
    /// minted ahead of time, plus the raw value for mapper variants. The guest
    /// resolves the id into a real `Application::Message` itself, so this just
    /// forwards to the matching `update-*` entry point.
    pub fn plugin_update(&mut self, id: &str, msg: Message) -> Result<iced::Task<Message>> {
        let Some(plugin) = self.plugins.get_mut(id) else {
            return Err(PluginError::NotFound(id.into()));
        };
        let (callback_id, value) = match msg {
            Message::Fixed { rep } => (rep, MessageValue::Fixed),
            Message::Bool { mapper, value } => (mapper, MessageValue::BoolValue(value)),
            Message::F32 { mapper, value } => (mapper, MessageValue::F32Value(value)),
            Message::F64 { mapper, value } => (mapper, MessageValue::F64Value(value)),
            Message::U64 { mapper, value } => (mapper, MessageValue::U64Value(value)),
            Message::String { mapper, value } => (mapper, MessageValue::StringValue(value)),
            Message::Viewport { mapper, value } => (mapper, MessageValue::ViewportValue(value)),
        };
        let mut store = self.store.borrow_mut();
        let app = plugin.app.iced_app_app_instance().application();
        let task = app.call_update(store.deref_mut(), plugin.instance, callback_id, &value)?;
        let task = store
            .data_mut()
            .table
            .delete(task)
            .map_err(wasmtime::Error::from)?
            .0;
        Ok(task)
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
        let tree = plugin
            .app
            .iced_app_app_instance()
            .application()
            .call_view(store.deref_mut(), plugin.instance)
            .inspect_err(|e| {
                tracing::error!("Failed to call view for plugin {}: {}", id, e);
            })
            .ok()?;
        let root = tree.root;
        let mut nodes: Vec<Option<Node>> = tree.nodes.into_iter().map(Some).collect();
        Some(build_element(&mut nodes, root))
    }

    /// Adds a plugin from pre-loaded bytes.
    pub fn add_plugin_from_bytes(
        &mut self,
        name: &str,
        bytes: &[u8],
    ) -> Result<iced::Task<Message>> {
        let component = Component::from_binary(&self.engine, bytes)?;
        let (plugin, boot) = self.instantiate(&component)?;
        if self.plugins.insert(name.to_string(), plugin).is_some() {
            info!("Replaced existing plugin: {}", name);
        }
        Ok(boot)
    }

    /// Adds a pre-compiled plugin component, returning its boot task.
    pub fn add_compiled_plugin(&mut self, plugin: CompiledPlugin) -> Result<iced::Task<Message>> {
        let (instance, boot) = self.instantiate(&plugin.component)?;
        if self.plugins.insert(plugin.name.clone(), instance).is_some() {
            info!("Replaced existing plugin: {}", plugin.name);
        }
        Ok(boot)
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
        let (plugin, _boot) = self.instantiate(&component)?;
        if self.plugins.insert(id.clone(), plugin).is_some() {
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

    let component = tokio::task::spawn_blocking(move || Component::from_binary(&engine, &bytes))
        .await
        .map_err(|e| format!("Task join error for {}: {}", name, e))?
        .map_err(|e| format!("Compilation error for {}: {}", name, e))?;

    Ok(CompiledPlugin {
        name,
        component: Arc::new(component),
    })
}
