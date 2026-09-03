//! Component workers and the lifecycle of their guest-owned jobs.

use crate::{
    bindings::{
        App,
        exports::iced::app::app_instance::{CallbackEvent, Event, JobError, MessageValue},
        iced::app::{
            capabilities::{CapabilityError, KeyEvent},
            widgets::ViewTree,
        },
    },
    widgets::{Message, WrapperRenderer, WrapperTheme, build_element},
};
use futures::{
    FutureExt, StreamExt,
    channel::{
        mpsc::{UnboundedReceiver, UnboundedSender},
        oneshot,
    },
};
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
    thread::JoinHandle,
};
use wasmtime::{
    AsContextMut, Config, Engine, Store, StoreContextMut,
    component::{
        Accessor, Component, FutureConsumer, HasSelf, Linker, ResourceAny, Source, StreamConsumer,
        StreamResult,
    },
};
use wasmtime_wasi::{
    ResourceTable, WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView, p2::add_to_linker_async,
};

pub(crate) type Keys = Arc<Mutex<Vec<(KeyEvent, futures::channel::mpsc::Sender<String>)>>>;

pub struct MyState {
    wasi: WasiCtx,
    pub table: ResourceTable,
    pub(crate) effects: UnboundedSender<Effect>,
    pub(crate) keys: Keys,
}
impl std::fmt::Debug for MyState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MyState").finish_non_exhaustive()
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
    #[error("Plugin worker thread is gone")]
    WorkerGone,
}
type Result<T> = std::result::Result<T, PluginError>;
type WakeMessage = (String, Message);

#[derive(Debug)]
pub(crate) enum Effect {
    Read(oneshot::Sender<std::result::Result<Option<String>, CapabilityError>>),
    Write(
        String,
        oneshot::Sender<std::result::Result<(), CapabilityError>>,
    ),
    Exit(oneshot::Sender<std::result::Result<(), CapabilityError>>),
}

/// An effect is consumed once, even if iced clones its delivery message.
#[derive(Debug, Clone)]
pub struct SharedEffect(Arc<Mutex<Option<Effect>>>);
impl SharedEffect {
    fn run(self) -> iced::Task<Message> {
        match self.0.lock().unwrap().take() {
            Some(Effect::Read(reply)) => {
                if reply.is_canceled() {
                    return iced::Task::none();
                }
                let mut reply = Some(reply);
                iced::clipboard::read_text()
                    .map(move |result| {
                        let value = result
                            .map(|text| Some((*text).clone()))
                            .map_err(|error| CapabilityError::Failed(format!("{error:?}")));
                        if let Some(reply) = reply.take() {
                            let _ = reply.send(value);
                        }
                    })
                    .discard()
            }
            Some(Effect::Write(contents, reply)) => {
                if reply.is_canceled() {
                    return iced::Task::none();
                }
                let mut reply = Some(reply);
                iced::clipboard::write(contents)
                    .map(move |result| {
                        if let Some(reply) = reply.take() {
                            let _ = reply
                                .send(result.map_err(|error| {
                                    CapabilityError::Failed(format!("{error:?}"))
                                }));
                        }
                    })
                    .discard()
            }
            Some(Effect::Exit(reply)) => {
                if reply.is_canceled() {
                    return iced::Task::none();
                }
                let _ = reply.send(Ok(()));
                iced::exit()
            }
            None => iced::Task::none(),
        }
    }
}

enum Request {
    View {
        reply: std::sync::mpsc::Sender<Option<ViewTree>>,
    },
    Update {
        event: Event,
        reply: std::sync::mpsc::Sender<Result<iced::Task<Message>>>,
    },
    Shutdown,
}
struct PluginHandle {
    req: UnboundedSender<Request>,
    worker: Option<JoinHandle<()>>,
    generation: u64,
}

/// Each component owns a concurrent worker; UI methods send it application calls.
pub struct PluginManager {
    engine: Engine,
    linker: Linker<MyState>,
    plugins: HashMap<String, PluginHandle>,
    ids: Vec<String>,
    wake_tx: UnboundedSender<WakeMessage>,
    wake_rx: Arc<Mutex<Option<UnboundedReceiver<WakeMessage>>>>,
    keys: Keys,
    next_generation: u64,
}
impl std::fmt::Debug for PluginManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PluginManager")
            .field("ids", &self.ids)
            .finish_non_exhaustive()
    }
}
impl PluginManager {
    pub fn new() -> Result<Self> {
        let mut config = Config::new();
        config
            .wasm_component_model(true)
            .wasm_component_model_async(true);
        let engine = Engine::new(&config)?;
        let mut linker = Linker::new(&engine);
        App::add_to_linker::<_, HasSelf<_>>(&mut linker, |s| s)?;
        add_to_linker_async(&mut linker)?;
        let (wake_tx, wake_rx) = futures::channel::mpsc::unbounded();
        Ok(Self {
            engine,
            linker,
            plugins: HashMap::new(),
            ids: Vec::new(),
            wake_tx,
            wake_rx: Arc::new(Mutex::new(Some(wake_rx))),
            keys: Arc::new(Mutex::new(Vec::new())),
            next_generation: 0,
        })
    }
    pub fn ids(&self) -> Vec<String> {
        self.ids.clone()
    }
    pub fn plugin_update(&mut self, id: &str, message: Message) -> Result<iced::Task<Message>> {
        let event = match message {
            Message::Effect(effect) => return Ok(effect.run()),
            Message::Job {
                generation,
                id: token,
            } => {
                if self
                    .plugins
                    .get(id)
                    .is_none_or(|plugin| plugin.generation != generation)
                {
                    return Ok(iced::Task::none());
                }
                Event::Message(token)
            }
            message => {
                let (id, value) = match message {
                    Message::Fixed { rep } => (rep, MessageValue::Fixed),
                    Message::Bool { mapper, value } => (mapper, MessageValue::BoolValue(value)),
                    Message::F32 { mapper, value } => (mapper, MessageValue::F32Value(value)),
                    Message::F64 { mapper, value } => (mapper, MessageValue::F64Value(value)),
                    Message::U64 { mapper, value } => (mapper, MessageValue::U64Value(value)),
                    Message::String { mapper, value } => (mapper, MessageValue::StringValue(value)),
                    Message::Viewport { mapper, value } => {
                        (mapper, MessageValue::ViewportValue(value))
                    }
                    _ => unreachable!(),
                };
                Event::Callback(CallbackEvent { id, value })
            }
        };
        let (tx, rx) = std::sync::mpsc::channel();
        self.send(id, Request::Update { event, reply: tx })?;
        rx.recv().map_err(|_| PluginError::WorkerGone)?
    }
    pub fn plugin_view<'a, Theme, Renderer>(
        &self,
        id: &str,
    ) -> Option<iced::Element<'a, Message, Theme, Renderer>>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a + 'static,
    {
        let (tx, rx) = std::sync::mpsc::channel();
        self.plugins
            .get(id)?
            .req
            .unbounded_send(Request::View { reply: tx })
            .ok()?;
        let tree = rx.recv().ok()??;
        let root = tree.root;
        let mut nodes = tree.nodes.into_iter().map(Some).collect();
        Some(build_element(&mut nodes, root))
    }

    /// Adds a plugin from a file, discarding its boot task.
    pub fn add_plugin_from_file(
        &mut self,
        name: impl Into<String>,
        file: impl AsRef<Path>,
    ) -> Result<()> {
        let component = Component::from_file(&self.engine, file)?;
        self.add(name.into(), Arc::new(component)).map(drop)
    }

    /// Adds a plugin from pre-loaded bytes, returning its boot task.
    pub fn add_plugin_from_bytes(
        &mut self,
        name: &str,
        bytes: &[u8],
    ) -> Result<iced::Task<Message>> {
        let component = Component::from_binary(&self.engine, bytes)?;
        self.add(name.to_string(), Arc::new(component))
    }

    /// Adds a pre-compiled plugin component, returning its boot task.
    pub fn add_compiled_plugin(&mut self, plugin: CompiledPlugin) -> Result<iced::Task<Message>> {
        self.add(plugin.name, plugin.component)
    }

    /// Returns a clone of the engine for use in async plugin compilation.
    pub fn engine(&self) -> Engine {
        self.engine.clone()
    }

    pub fn wake_subscription(&self) -> iced::Subscription<WakeMessage> {
        iced::advanced::subscription::from_recipe(WakeRecipe {
            rx: self.wake_rx.clone(),
        })
    }
    /// Forwards keyboard input to capability readers with bounded delivery.
    pub fn subscriptions(&self) -> iced::Subscription<WakeMessage> {
        iced::advanced::subscription::from_recipe(KeyRecipe {
            keys: self.keys.clone(),
        })
    }
    #[doc(hidden)]
    pub fn raw_add<F: FnOnce(&Engine) -> Result<Component>>(
        &mut self,
        id: String,
        component: F,
    ) -> Result<()> {
        let component = component(&self.engine)?;
        self.add(id, Arc::new(component)).map(drop)
    }

    fn add(&mut self, name: String, component: Arc<Component>) -> Result<iced::Task<Message>> {
        let (req_tx, req_rx) = futures::channel::mpsc::unbounded();
        let (boot_tx, boot_rx) = std::sync::mpsc::channel();

        let engine = self.engine.clone();
        let linker = self.linker.clone();
        let wake_tx = self.wake_tx.clone();
        let keys = self.keys.clone();
        let generation = self.next_generation;
        self.next_generation = self
            .next_generation
            .checked_add(1)
            .expect("plugin identity exhausted");
        let worker_name = name.clone();
        let worker = std::thread::Builder::new()
            .name(format!("igloo-plugin-{name}"))
            .spawn(move || {
                run_plugin_worker(Worker {
                    engine,
                    linker,
                    component,
                    name: worker_name,
                    requests: req_rx,
                    wake: wake_tx,
                    keys,
                    generation,
                    boot_reply: boot_tx,
                });
            })
            .map_err(|e| PluginError::UpdateFailed(e.to_string()))?;

        let boot = boot_rx.recv().map_err(|_| PluginError::WorkerGone)??;

        if let Some(mut old) = self.plugins.remove(&name) {
            let _ = old.req.unbounded_send(Request::Shutdown);
            if let Some(worker) = old.worker.take() {
                let _ = worker.join();
            }
        }
        self.plugins.insert(
            name.clone(),
            PluginHandle {
                req: req_tx,
                worker: Some(worker),
                generation,
            },
        );
        if !self.ids.contains(&name) {
            self.ids.push(name);
        }
        Ok(boot)
    }

    fn send(&self, id: &str, req: Request) -> Result<()> {
        self.plugins
            .get(id)
            .ok_or_else(|| PluginError::NotFound(id.to_string()))?
            .req
            .unbounded_send(req)
            .map_err(|_| PluginError::WorkerGone)
    }
}

impl Drop for PluginManager {
    fn drop(&mut self) {
        for handle in self.plugins.values_mut() {
            let _ = handle.req.unbounded_send(Request::Shutdown);
            if let Some(worker) = handle.worker.take() {
                let _ = worker.join();
            }
        }
    }
}
/// The [`Recipe`](iced::advanced::subscription::Recipe) behind
/// [`PluginManager::wake_subscription`]. It owns the receiver end of the wake
/// channel; iced runs it once and the stream lives for the app's lifetime.
struct WakeRecipe {
    rx: Arc<Mutex<Option<UnboundedReceiver<WakeMessage>>>>,
}

impl iced::advanced::subscription::Recipe for WakeRecipe {
    type Output = WakeMessage;

    fn hash(&self, state: &mut iced::advanced::subscription::Hasher) {
        use std::hash::Hash;
        std::any::TypeId::of::<Self>().hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: iced::advanced::subscription::EventStream,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        match self.rx.lock().unwrap().take() {
            Some(rx) => rx.boxed(),
            None => futures::stream::pending().boxed(),
        }
    }
}

struct KeyRecipe {
    keys: Keys,
}
impl iced::advanced::subscription::Recipe for KeyRecipe {
    type Output = WakeMessage;
    fn hash(&self, state: &mut iced::advanced::subscription::Hasher) {
        use std::hash::Hash;
        std::any::TypeId::of::<Self>().hash(state);
        Arc::as_ptr(&self.keys).hash(state);
    }
    fn stream(
        self: Box<Self>,
        input: iced::advanced::subscription::EventStream,
    ) -> futures::stream::BoxStream<'static, WakeMessage> {
        input
            .filter_map(move |event| {
                if let iced::advanced::subscription::Event::Interaction {
                    event: iced::Event::Keyboard(event),
                    ..
                } = event
                {
                    let event = match event {
                        iced::keyboard::Event::KeyPressed { key, .. } => Some((true, key)),
                        iced::keyboard::Event::KeyReleased { key, .. } => Some((false, key)),
                        iced::keyboard::Event::ModifiersChanged(_) => None,
                    };
                    if let Some((pressed, key)) = event {
                        let value = match key {
                            iced::keyboard::Key::Character(c) => c.to_string(),
                            key => format!("{key:?}"),
                        };
                        self.keys.lock().unwrap().retain_mut(|(kind, sender)| {
                            if sender.is_closed() {
                                return false;
                            }
                            if matches!(kind, KeyEvent::Press) == pressed {
                                let _ = sender.try_send(value.clone());
                            }
                            true
                        });
                    }
                }
                futures::future::ready(None)
            })
            .boxed()
    }
}

struct Messages {
    plugin: String,
    generation: u64,
    wake: UnboundedSender<WakeMessage>,
}
impl StreamConsumer<MyState> for Messages {
    type Item = u64;
    fn poll_consume(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        mut store: StoreContextMut<'_, MyState>,
        mut source: Source<'_, u64>,
        finish: bool,
    ) -> Poll<wasmtime::Result<StreamResult>> {
        let mut values = Vec::with_capacity(1);
        source.read(store.as_context_mut(), &mut values)?;
        if values.is_empty() {
            return if finish {
                Poll::Ready(Ok(StreamResult::Cancelled))
            } else {
                Poll::Pending
            };
        }
        for id in values {
            if self
                .wake
                .unbounded_send((
                    self.plugin.clone(),
                    Message::Job {
                        generation: self.generation,
                        id,
                    },
                ))
                .is_err()
            {
                return Poll::Ready(Ok(StreamResult::Dropped));
            }
        }
        Poll::Ready(Ok(StreamResult::Completed))
    }
}
struct Completion {
    id: u64,
    completed: UnboundedSender<(u64, std::result::Result<(), JobError>)>,
}
impl FutureConsumer<MyState> for Completion {
    type Item = std::result::Result<(), JobError>;
    fn poll_consume(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        mut store: StoreContextMut<'_, MyState>,
        mut source: Source<'_, Self::Item>,
        finish: bool,
    ) -> Poll<wasmtime::Result<()>> {
        let mut values = Vec::with_capacity(1);
        source.read(store.as_context_mut(), &mut values)?;
        if let Some(result) = values.pop() {
            let _ = self.completed.unbounded_send((self.id, result));
            return Poll::Ready(Ok(()));
        }
        if finish {
            Poll::Ready(Ok(()))
        } else {
            Poll::Pending
        }
    }
}

fn drop_resource(acc: &Accessor<MyState>, resource: ResourceAny) -> wasmtime::Result<()> {
    // Canonical resource destructors cannot suspend. Wasmtime's async store
    // still requires its fiber entrypoint when invoked from a concurrent worker.
    acc.with(|mut store| {
        resource
            .resource_drop_async(&mut store)
            .now_or_never()
            .ok_or_else(|| wasmtime::Error::msg("resource destructor suspended"))?
    })
}

struct Jobs {
    name: String,
    generation: u64,
    next: u64,
    live: HashMap<u64, ResourceAny>,
    subscriptions: HashMap<String, (u64, Option<u64>)>,
    wake: UnboundedSender<WakeMessage>,
    completed: UnboundedSender<(u64, std::result::Result<(), JobError>)>,
}
impl Jobs {
    async fn start(&mut self, acc: &Accessor<MyState>, app: &App, job: ResourceAny) -> Result<u64> {
        let id = self.next;
        self.next = self.next.checked_add(1).expect("job identity exhausted");
        let execution = app
            .iced_app_app_instance()
            .job()
            .call_start(acc, job)
            .await?
            .map_err(|error| PluginError::UpdateFailed(format!("job start: {error:?}")))?;
        self.live.insert(id, job);
        acc.with(|mut store| {
            execution.messages.pipe(
                &mut store,
                Messages {
                    plugin: self.name.clone(),
                    generation: self.generation,
                    wake: self.wake.clone(),
                },
            )?;
            execution.completion.pipe(
                &mut store,
                Completion {
                    id,
                    completed: self.completed.clone(),
                },
            )
        })?;
        Ok(id)
    }
    async fn discard(&self, acc: &Accessor<MyState>, app: &App, job: ResourceAny) -> Result<()> {
        app.iced_app_app_instance()
            .job()
            .call_cancel(acc, job)
            .await?;
        drop_resource(acc, job)?;
        Ok(())
    }
    async fn cancel(&mut self, acc: &Accessor<MyState>, app: &App, id: u64) -> Result<()> {
        if let Some(job) = self.live.remove(&id) {
            self.discard(acc, app, job).await?;
        }
        Ok(())
    }
    fn complete(&mut self, acc: &Accessor<MyState>, id: u64) -> Result<()> {
        if let Some(job) = self.live.remove(&id) {
            for (_, active) in self.subscriptions.values_mut() {
                if *active == Some(id) {
                    *active = None;
                }
            }
            drop_resource(acc, job)?;
        }
        Ok(())
    }
    async fn reconcile(
        &mut self,
        acc: &Accessor<MyState>,
        app: &App,
        instance: ResourceAny,
    ) -> Result<()> {
        let declared = app
            .iced_app_app_instance()
            .application()
            .call_subscription(acc, instance)
            .await?;
        let mut keys = HashSet::new();
        if declared.iter().any(|entry| !keys.insert(entry.key.clone())) {
            for entry in declared {
                self.discard(acc, app, entry.job).await?;
            }
            return Err(PluginError::UpdateFailed(
                "duplicate subscription key".into(),
            ));
        }
        let removed: Vec<_> = self
            .subscriptions
            .keys()
            .filter(|key| !keys.contains(*key))
            .cloned()
            .collect();
        for key in removed {
            if let Some((_, Some(id))) = self.subscriptions.remove(&key) {
                self.cancel(acc, app, id).await?;
            }
        }
        for entry in declared {
            if self
                .subscriptions
                .get(&entry.key)
                .is_some_and(|(revision, _)| *revision == entry.revision)
            {
                self.discard(acc, app, entry.job).await?;
                continue;
            }
            if let Some((_, Some(id))) = self.subscriptions.remove(&entry.key) {
                self.cancel(acc, app, id).await?;
            }
            let id = self.start(acc, app, entry.job).await?;
            self.subscriptions
                .insert(entry.key, (entry.revision, Some(id)));
        }
        Ok(())
    }
    async fn shutdown(&mut self, acc: &Accessor<MyState>, app: &App) {
        let ids: Vec<_> = self.live.keys().copied().collect();
        for id in ids {
            if let Err(error) = self.cancel(acc, app, id).await {
                tracing::error!("job shutdown: {error}");
            }
        }
    }
}

struct Worker {
    engine: Engine,
    linker: Linker<MyState>,
    component: Arc<Component>,
    name: String,
    requests: UnboundedReceiver<Request>,
    wake: UnboundedSender<WakeMessage>,
    keys: Keys,
    generation: u64,
    boot_reply: std::sync::mpsc::Sender<Result<iced::Task<Message>>>,
}

fn run_plugin_worker(worker: Worker) {
    let Worker {
        engine,
        linker,
        component,
        name,
        mut requests,
        wake,
        keys,
        generation,
        boot_reply,
    } = worker;
    let runtime = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime,
        Err(error) => {
            let _ = boot_reply.send(Err(PluginError::UpdateFailed(error.to_string())));
            return;
        }
    };
    runtime.block_on(async move {
        let (effects, mut effects_rx) = futures::channel::mpsc::unbounded();
        let wasi = WasiCtxBuilder::new().inherit_stderr().inherit_stdout().build();
        let mut store = Store::new(&engine, MyState { wasi, table: ResourceTable::new(), effects, keys });
        let app = match App::instantiate_async(&mut store, &component, &linker).await {
            Ok(app) => app,
            Err(error) => { let _ = boot_reply.send(Err(error.context("instantiate component").into())); return; }
        };
        let mut boot_reply = Some(boot_reply);
        let result = store.run_concurrent(async |acc| {
            let application = app.iced_app_app_instance().application();
            let instance = application.call_constructor(acc).await?;
            let (completed, mut completed_rx) = futures::channel::mpsc::unbounded();
            let mut jobs = Jobs { name: name.clone(), generation, next: 0, live: HashMap::new(), subscriptions: HashMap::new(), wake: wake.clone(), completed };
            let result: Result<()> = async {
                if let Some(job) = application.call_boot(acc, instance).await? { jobs.start(acc, &app, job).await?; }
                jobs.reconcile(acc, &app, instance).await?;
                if let Some(reply) = boot_reply.take() { let _ = reply.send(Ok(iced::Task::none())); }
                loop {
                    futures::select! {
                        request = requests.next() => match request {
                            Some(Request::View { reply }) => {
                                let tree = application.call_view(acc, instance).await;
                                let failed = tree.is_err();
                                let _ = reply.send(tree.ok());
                                if failed { return Err(PluginError::UpdateFailed("view trapped".into())); }
                            }
                            Some(Request::Update { event, reply }) => {
                                let result: Result<iced::Task<Message>> = async {
                                    if let Some(job) = application.call_update(acc, instance, event).await? { jobs.start(acc, &app, job).await?; }
                                    jobs.reconcile(acc, &app, instance).await?;
                                    Ok(iced::Task::none())
                                }.await;
                                let failed = result.is_err();
                                let _ = reply.send(result);
                                if failed { return Err(PluginError::UpdateFailed("update failed".into())); }
                            }
                            _ => break,
                        },
                        effect = effects_rx.next() => if let Some(effect) = effect {
                            let _ = wake.unbounded_send((name.clone(), Message::Effect(SharedEffect(Arc::new(Mutex::new(Some(effect)))))));
                        },
                        finished = completed_rx.next() => if let Some((id, result)) = finished {
                            if let Err(error) = result { tracing::warn!("job in {name} stopped: {error:?}"); }
                            jobs.complete(acc, id)?;
                        },
                    }
                }
                Ok(())
            }.await;
            jobs.shutdown(acc, &app).await;
            drop_resource(acc, instance)?;
            result.map_err(|error| wasmtime::Error::msg(error.to_string()))
        }).await;
        let result = result.and_then(|result| result);
        if let Some(reply) = boot_reply.take() {
            let error = result.err().map(PluginError::WasmError).unwrap_or(PluginError::WorkerGone);
            let _ = reply.send(Err(error));
        } else if let Err(error) = result { tracing::error!("plugin {name}: {error}"); }
    });
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "build the WASM component with mise run //plugins/rust:build first"]
    async fn component_delivers_timer_while_idle_and_shuts_down() {
        let mut manager = PluginManager::new().unwrap();
        manager
            .add_plugin_from_file(
                "probe",
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../../target/wasm32-wasip2/release/rust_guest.wasm"),
            )
            .unwrap();
        let mut events = manager.wake_rx.lock().unwrap().take().unwrap();
        for _ in 0..2 {
            let (plugin, message) =
                tokio::time::timeout(std::time::Duration::from_secs(4), events.next())
                    .await
                    .expect("idle timer must progress")
                    .unwrap();
            assert_eq!(plugin, "probe");
            assert!(matches!(message, Message::Job { .. }));
            let _ = manager.plugin_update(&plugin, message).unwrap();
        }
        drop(manager);
    }
    fn view(manager: &PluginManager) -> ViewTree {
        let (reply, receiver) = std::sync::mpsc::channel();
        manager.send("probe", Request::View { reply }).unwrap();
        receiver.recv().unwrap().unwrap()
    }

    fn click(manager: &mut PluginManager, label: &str) {
        use crate::bindings::iced::app::widgets::Node;
        let tree = view(manager);
        let callback = tree
            .nodes
            .iter()
            .find_map(|node| {
                let Node::Button(button) = node else {
                    return None;
                };
                let Node::Text(text) = &tree.nodes[button.content as usize] else {
                    return None;
                };
                (text.content == label).then_some(button.on_press.unwrap())
            })
            .expect("fixture button");
        let _ = manager
            .plugin_update("probe", Message::Fixed { rep: callback })
            .unwrap();
    }

    fn log(manager: &PluginManager) -> String {
        use crate::bindings::iced::app::widgets::Node;
        let tree = view(manager);
        let Node::Text(text) = &tree.nodes[0] else {
            panic!("fixture log");
        };
        text.content.clone()
    }

    async fn deliver(manager: &mut PluginManager, events: &mut UnboundedReceiver<WakeMessage>) {
        let (plugin, message) =
            tokio::time::timeout(std::time::Duration::from_secs(3), events.next())
                .await
                .expect("job must progress while idle")
                .unwrap();
        let _ = manager.plugin_update(&plugin, message).unwrap();
    }

    #[tokio::test]
    #[ignore = "build with mise run //plugins/rust:build-async-probe first"]
    async fn component_chain_backpressure_and_subscription_lifetimes() {
        let mut manager = PluginManager::new().unwrap();
        manager
            .add_plugin_from_file(
                "probe",
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../../target/wasm32-wasip2/release/examples/async_probe.wasm"),
            )
            .unwrap();
        let mut events = manager.wake_rx.lock().unwrap().take().unwrap();
        deliver(&mut manager, &mut events).await;
        assert_eq!(log(&manager), "[100]");
        click(&mut manager, "refresh");
        assert!(
            tokio::time::timeout(std::time::Duration::from_millis(50), events.next())
                .await
                .is_err(),
            "finished identity must not restart"
        );
        click(&mut manager, "chain");
        let (_, first) = tokio::time::timeout(std::time::Duration::from_secs(3), events.next())
            .await
            .unwrap()
            .unwrap();
        assert!(
            tokio::time::timeout(std::time::Duration::from_millis(50), events.next())
                .await
                .is_err(),
            "one outstanding message per producer"
        );
        let _ = manager.plugin_update("probe", first).unwrap();
        for _ in 0..3 {
            deliver(&mut manager, &mut events).await;
        }
        assert_eq!(log(&manager), "[100, 1, 2, 3, 4]");
        click(&mut manager, "revise");
        deliver(&mut manager, &mut events).await;
        click(&mut manager, "remove");
        click(&mut manager, "add");
        deliver(&mut manager, &mut events).await;
        assert_eq!(log(&manager), "[100, 1, 2, 3, 4, 100, 100]");
        click(&mut manager, "pending");
        drop(manager);
    }

    #[tokio::test]
    #[ignore = "build with mise run //plugins/rust:build-async-probe first"]
    async fn component_cancellation_reclaims_queued_tokens_and_replacement_isolated() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../target/wasm32-wasip2/release/examples/async_probe.wasm");
        let mut manager = PluginManager::new().unwrap();
        manager.add_plugin_from_file("probe", &path).unwrap();
        let mut events = manager.wake_rx.lock().unwrap().take().unwrap();
        let (_, stale) = tokio::time::timeout(std::time::Duration::from_secs(3), events.next())
            .await
            .unwrap()
            .unwrap();
        click(&mut manager, "remove");
        let _ = manager.plugin_update("probe", stale.clone()).unwrap();
        assert_eq!(log(&manager), "[]");
        manager.add_plugin_from_file("probe", &path).unwrap();
        let _ = manager.plugin_update("probe", stale).unwrap();
        assert_eq!(log(&manager), "[]");
        deliver(&mut manager, &mut events).await;
        assert_eq!(log(&manager), "[100]");
    }
}
