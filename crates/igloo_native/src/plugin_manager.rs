//! Host routing for native plugins, independent of the WASM manager.
use crate::{NativePlugin, NativePluginError, Permissions, abi};
use std::collections::BTreeMap;

/// A native event tagged with the originating plugin instance.
#[derive(Debug)]
pub struct Message {
    instance: u64,
    value: abi::Message,
}

/// Owns native applications and routes their views, tasks and subscriptions.
#[derive(Default)]
pub struct NativePluginManager {
    plugins: BTreeMap<String, (NativePlugin, Permissions)>,
}

impl NativePluginManager {
    pub fn new() -> Self {
        Self::default()
    }
    /// Loads or replaces a named plugin; stale events from replaced instances are ignored.
    pub fn load(
        &mut self,
        name: impl Into<String>,
        path: impl AsRef<std::path::Path>,
        permissions: Permissions,
    ) -> Result<iced::Task<Message>, NativePluginError> {
        let (plugin, task) = NativePlugin::load(path, permissions)?;
        let instance = plugin.instance;
        self.plugins.insert(name.into(), (plugin, permissions));
        Ok(task.map(move |value| Message { instance, value }))
    }
    pub fn ids(&self) -> impl Iterator<Item = &str> {
        self.plugins.keys().map(String::as_str)
    }
    pub fn remove(&mut self, name: &str) -> bool {
        self.plugins.remove(name).is_some()
    }
    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        let instance = message.instance;
        match self
            .plugins
            .values_mut()
            .find(|(p, _)| p.instance == instance)
        {
            Some((plugin, permissions)) => {
                // Message fields are private, and instance IDs are never reused.
                #[allow(unsafe_code)]
                let task = unsafe { plugin.update(message.value, *permissions) };
                task.map(move |value| Message { instance, value })
            }
            None => iced::Task::none(),
        }
    }
    pub fn view(&self, name: &str) -> Option<iced::Element<'_, Message>> {
        self.plugins.get(name).map(|(plugin, _)| {
            let instance = plugin.instance;
            plugin.view().map(move |value| Message { instance, value })
        })
    }
    pub fn subscriptions(&self) -> iced::Subscription<Message> {
        iced::Subscription::batch(self.plugins.values().map(|(plugin, _)| {
            let instance = plugin.instance;
            plugin
                .subscription()
                .with(instance)
                .map(|(instance, value)| Message { instance, value })
        }))
    }
}

impl std::fmt::Debug for NativePluginManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NativePluginManager")
            .field("plugins", &self.plugins.keys().collect::<Vec<_>>())
            .finish()
    }
}
