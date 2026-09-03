//! Guest-authored stream subscriptions with stable identities.
use futures::StreamExt;
enum EventSource<M> {
    Stream {
        id: u64,
        start: Box<dyn Fn() -> futures::stream::BoxStream<'static, M> + Send + Sync>,
    },
    Timer {
        duration: std::time::Duration,
        emit: Box<dyn Fn(u64) -> M + Send + Sync>,
    },
}

/// Event sources returned by an application on each subscription pass.
pub struct Subscription<M> {
    sources: Vec<EventSource<M>>,
}

impl<M: Send + 'static> Subscription<M> {
    pub fn none() -> Self {
        Self {
            sources: Vec::new(),
        }
    }

    /// Starts a stream when iced first observes this ID; change the ID to restart it.
    ///
    /// ```
    /// use igloo_native::Subscription;
    /// let state = std::sync::Arc::new(1);
    /// let subscription = Subscription::run(1, move || futures::stream::iter([*state]));
    /// ```
    ///
    pub fn run<S>(id: u64, factory: impl Fn() -> S + Send + Sync + 'static) -> Self
    where
        S: futures::Stream<Item = M> + Send + 'static,
    {
        Self {
            sources: vec![EventSource::Stream {
                id,
                start: Box::new(move || factory().boxed()),
            }],
        }
    }

    pub fn batch(subscriptions: impl IntoIterator<Item = Self>) -> Self {
        Self {
            sources: subscriptions
                .into_iter()
                .flat_map(|subscription| subscription.sources)
                .collect(),
        }
    }

    pub fn map<B: Send + 'static>(
        self,
        mapper: impl Fn(M) -> B + Send + Sync + 'static,
    ) -> Subscription<B> {
        let mapper = std::sync::Arc::new(mapper);
        Subscription {
            sources: self
                .sources
                .into_iter()
                .map(|source| {
                    let mapper = mapper.clone();
                    match source {
                        EventSource::Stream { id, start } => EventSource::Stream {
                            id,
                            start: Box::new(move || {
                                let mapper = mapper.clone();
                                start().map(move |message| mapper(message)).boxed()
                            }),
                        },
                        EventSource::Timer { duration, emit } => EventSource::Timer {
                            duration,
                            emit: Box::new(move |tick| mapper(emit(tick))),
                        },
                    }
                })
                .collect(),
        }
    }

    pub(crate) fn erase(self) -> Sources {
        Sources {
            sources: self
                .sources
                .into_iter()
                .map(|source| match source {
                    EventSource::Stream { id, start } => Source::Stream(StreamSource {
                        id,
                        start: stabby::boxed::Box::new(move || {
                            crate::wire_task::box_stream(start().map(crate::messages::Message::new))
                        })
                        .into(),
                    }),
                    EventSource::Timer { duration, emit } => Source::Timer(Timer {
                        seconds: duration.as_secs(),
                        nanos: duration.subsec_nanos(),
                        emit: stabby::boxed::Box::new(move |tick| {
                            crate::messages::Message::new(emit(tick))
                        })
                        .into(),
                    }),
                })
                .collect(),
        }
    }
}

impl Subscription<u64> {
    /// Emits epoch milliseconds at each tick of the host's iced timer.
    pub fn every(duration: std::time::Duration) -> Self {
        assert!(!duration.is_zero(), "timer duration must be positive");
        Self {
            sources: vec![EventSource::Timer {
                duration,
                emit: Box::new(|tick| tick),
            }],
        }
    }
}

impl<M: Send + 'static> Default for Subscription<M> {
    fn default() -> Self {
        Self::none()
    }
}
impl<M> std::fmt::Debug for Subscription<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Subscription")
            .field("sources", &self.sources.len())
            .finish()
    }
}
/// A thread-safe stream factory crossing the native boundary.
pub type Factory = stabby::dynptr!(
    stabby::boxed::Box<dyn stabby::closure::Call0<crate::helpers::BoxStream> + Send + Sync>
);
/// An ABI stream recipe.
#[stabby::stabby]
pub struct StreamSource {
    pub id: u64,
    pub start: Factory,
}
/// A timer callback whose captures can be shared with the host executor.
pub type Tick = stabby::dynptr!(
    stabby::boxed::Box<dyn stabby::closure::Call1<crate::messages::Message, u64> + Send + Sync>
);
/// A request to use the host's iced timer subscription.
#[stabby::stabby]
pub struct Timer {
    pub seconds: u64,
    pub nanos: u32,
    pub emit: Tick,
}
/// A single event source crossing the native boundary.
#[stabby::stabby]
#[repr(C, u8)]
pub enum Source {
    Stream(StreamSource),
    Timer(Timer),
}
/// ABI subscription recipes.
#[stabby::stabby]
pub struct Sources {
    pub sources: stabby::vec::Vec<Source>,
}
#[cfg(feature = "host")]
struct RecipeData {
    plugin: u64,
    id: u64,
    start: Factory,
}
#[cfg(feature = "host")]
impl std::hash::Hash for RecipeData {
    fn hash<H: std::hash::Hasher>(&self, h: &mut H) {
        self.plugin.hash(h);
        self.id.hash(h);
    }
}
#[cfg(feature = "host")]
impl Sources {
    pub(crate) fn into_iced(self, plugin: u64) -> iced::Subscription<crate::messages::Message> {
        fn start(
            data: &RecipeData,
        ) -> futures::stream::BoxStream<'static, crate::messages::Message> {
            use stabby::closure::Call0Dyn;
            crate::wire_task::unbox(data.start.call()).boxed()
        }
        let mut ids = std::collections::HashSet::new();
        iced::Subscription::batch(self.sources.into_iter().map(|source| match source {
            Source::Stream(source) => {
                assert!(ids.insert(source.id), "duplicate native subscription ID");
                iced::Subscription::run_with(
                    RecipeData {
                        plugin,
                        id: source.id,
                        start: source.start,
                    },
                    start,
                )
            }
            Source::Timer(timer) => {
                let duration = std::time::Duration::new(timer.seconds, timer.nanos);
                let emit = std::sync::Arc::new(timer.emit);
                let recipes = iced::advanced::subscription::into_recipes(
                    iced::time::every(duration).with(plugin),
                );
                iced::Subscription::batch(recipes.into_iter().map(move |inner| {
                    iced::advanced::subscription::from_recipe(TimerRecipe {
                        inner,
                        emit: emit.clone(),
                    })
                }))
            }
        }))
    }
}

#[cfg(feature = "host")]
struct TimerRecipe {
    inner: Box<dyn iced::advanced::subscription::Recipe<Output = (u64, std::time::Instant)>>,
    emit: std::sync::Arc<Tick>,
}
#[cfg(feature = "host")]
impl iced::advanced::subscription::Recipe for TimerRecipe {
    type Output = crate::messages::Message;
    fn hash(&self, state: &mut iced::advanced::subscription::Hasher) {
        self.inner.hash(state);
    }
    fn stream(
        self: Box<Self>,
        input: iced::advanced::subscription::EventStream,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        use stabby::closure::Call1Dyn;
        let TimerRecipe { inner, emit } = *self;
        inner
            .stream(input)
            .map(move |_| {
                let millis = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("system clock before epoch")
                    .as_millis()
                    .try_into()
                    .expect("epoch milliseconds exceed u64");
                emit.call(millis)
            })
            .boxed()
    }
}

#[cfg(all(test, feature = "host"))]
mod tests {
    use super::*;
    use iced::advanced::subscription::{Hasher, into_recipes};
    use std::hash::Hasher as _;
    fn hash(sub: Subscription<u64>, plugin: u64) -> u64 {
        let recipes = into_recipes(sub.erase().into_iced(plugin));
        let mut hasher = Hasher::default();
        recipes[0].hash(&mut hasher);
        hasher.finish()
    }
    #[test]
    fn identity_ignores_factory_allocation_but_includes_plugin_and_source() {
        let make = |id, value| Subscription::run(id, move || futures::stream::iter([value]));
        assert_eq!(hash(make(1, 10), 7), hash(make(1, 20), 7));
        assert_ne!(hash(make(1, 10), 7), hash(make(2, 10), 7));
        assert_ne!(hash(make(1, 10), 7), hash(make(1, 10), 8));
    }
    #[test]
    fn factories_are_lazy_and_mapping_preserves_values() {
        use std::sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        };
        let calls = Arc::new(AtomicUsize::new(0));
        let observed = calls.clone();
        let sub = Subscription::run(1, move || {
            observed.fetch_add(1, Ordering::Relaxed);
            futures::stream::iter([4_u64])
        })
        .map(|v| v + 1);
        let recipe = into_recipes(sub.erase().into_iced(1)).pop().unwrap();
        assert_eq!(calls.load(Ordering::Relaxed), 0);
        let stream = recipe.stream(futures::stream::empty().boxed());
        assert_eq!(calls.load(Ordering::Relaxed), 1);
        let message = futures::executor::block_on(stream.into_future()).0.unwrap();
        // This recipe was erased from a u64 subscription.
        #[allow(unsafe_code)]
        let value = unsafe { message.from_raw::<u64>() };
        assert_eq!(*value, 5);
    }
    #[test]
    fn cancelling_a_stream_drops_guest_state() {
        let alive = std::sync::Arc::new(());
        let capture = alive.clone();
        let sub = Subscription::run(1, move || {
            let capture = capture.clone();
            futures::stream::unfold(capture, |capture| async move {
                futures::future::pending::<()>().await;
                Some((1_u64, capture))
            })
        });
        let recipe = into_recipes(sub.erase().into_iced(1)).pop().unwrap();
        let stream = recipe.stream(futures::stream::empty().boxed());
        assert!(std::sync::Arc::strong_count(&alive) > 1);
        drop(stream);
        assert_eq!(std::sync::Arc::strong_count(&alive), 1);
    }
}
