//! Keyed guest jobs whose lifetime follows application state.

use crate::task::Task;
use futures::{Stream, StreamExt};

pub(crate) struct Entry<M> {
    pub key: String,
    pub revision: u64,
    pub task: Task<M>,
}

/// Declares jobs retained by key and revision across application updates.
pub struct Subscription<M> {
    pub(crate) entries: Vec<Entry<M>>,
}

impl<M> std::fmt::Debug for Subscription<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Subscription")
            .field("entries", &self.entries.len())
            .finish()
    }
}

impl<M: 'static> Subscription<M> {
    pub fn none() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    pub fn run<S: Stream<Item = M> + 'static>(
        key: impl Into<String>,
        make: impl FnOnce() -> S + 'static,
    ) -> Self {
        Self::from_task(key, Task::new(move || make().map(Ok).boxed_local()))
    }
    pub(crate) fn from_task(key: impl Into<String>, task: Task<M>) -> Self {
        Self {
            entries: vec![Entry {
                key: key.into(),
                revision: 0,
                task,
            }],
        }
    }
    pub fn batch(subscriptions: impl IntoIterator<Item = Self>) -> Self {
        Self {
            entries: subscriptions.into_iter().flat_map(|s| s.entries).collect(),
        }
    }
    /// Changes every entry's revision to restart its retained producer.
    pub fn with_revision(mut self, revision: u64) -> Self {
        self.entries
            .iter_mut()
            .for_each(|entry| entry.revision = revision);
        self
    }
    pub fn map<B: 'static>(self, map: impl Fn(M) -> B + 'static) -> Subscription<B> {
        let map = std::rc::Rc::new(map);
        Subscription {
            entries: self
                .entries
                .into_iter()
                .map(|entry| {
                    let map = map.clone();
                    Entry {
                        key: entry.key,
                        revision: entry.revision,
                        task: entry.task.map(move |value| map(value)),
                    }
                })
                .collect(),
        }
    }
}
