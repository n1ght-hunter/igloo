//! Host-side backing for the `iced:app/task` interface.
//!
//! Each guest call maps straight onto the matching `iced::Task` constructor or
//! combinator. The task is stashed in the [`ResourceTable`] like any widget
//! resource and taken back out by
//! [`crate::plugin_manager::PluginManager::plugin_update`] once the guest hands
//! the handle back.

use std::time::Duration;

use wasmtime::component::Resource;

use crate::{
    bindings::iced::app::task::{self, HostTask as HostTaskTrait},
    plugin_manager::MyState,
    widgets::Message,
};

/// A pending `iced::Task` a guest built during `update` or `boot`.
#[derive(Debug)]
pub struct HostTask(pub iced::Task<Message>);

impl task::Host for MyState {}

impl HostTaskTrait for MyState {
    fn none(&mut self) -> Resource<HostTask> {
        self.table.push(HostTask(iced::Task::none())).unwrap()
    }

    fn done(&mut self, message: u32) -> Resource<HostTask> {
        self.table
            .push(HostTask(iced::Task::done(Message::Fixed { rep: message })))
            .unwrap()
    }

    /// Not wired up yet: reading the guest-facing `FutureReader` needs the store's
    /// concurrent event loop plus a wake path back into iced. Until that lands
    /// this behaves like [`iced::Task::none`], and the future/stream is dropped.
    fn perform(
        &mut self,
        _value: wasmtime::component::FutureReader<task::MessageValue>,
        _on_complete: task::CallbackId,
    ) -> Resource<HostTask> {
        self.table.push(HostTask(iced::Task::none())).unwrap()
    }

    /// Not wired up yet, for the same reason as [`Self::perform`]; the stream is
    /// dropped and this behaves like [`iced::Task::none`].
    fn run(
        &mut self,
        _values: wasmtime::component::StreamReader<task::MessageValue>,
        _on_complete: task::CallbackId,
    ) -> Resource<HostTask> {
        self.table.push(HostTask(iced::Task::none())).unwrap()
    }

    fn batch(&mut self, tasks: Vec<Resource<HostTask>>) -> Resource<HostTask> {
        let tasks: Vec<iced::Task<Message>> = tasks
            .into_iter()
            .map(|handle| self.table.delete(handle).unwrap().0)
            .collect();
        self.table.push(HostTask(iced::Task::batch(tasks))).unwrap()
    }

    fn sleep(&mut self, millis: u64, on_complete: u32) -> Resource<HostTask> {
        let task = iced::Task::future(async move {
            tokio::time::sleep(Duration::from_millis(millis)).await;
        })
        .map(move |()| Message::Fixed { rep: on_complete });
        self.table.push(HostTask(task)).unwrap()
    }

    fn read_clipboard(&mut self, on_complete: u32) -> Resource<HostTask> {
        let task = iced::clipboard::read().map(move |contents| Message::String {
            mapper: on_complete,
            value: contents.unwrap_or_default(),
        });
        self.table.push(HostTask(task)).unwrap()
    }

    fn write_clipboard(&mut self, contents: String) -> Resource<HostTask> {
        self.table
            .push(HostTask(iced::clipboard::write(contents)))
            .unwrap()
    }

    fn exit(&mut self) -> Resource<HostTask> {
        self.table.push(HostTask(iced::exit())).unwrap()
    }

    fn chain(&mut self, self_: Resource<HostTask>, next: Resource<HostTask>) -> Resource<HostTask> {
        let first = self.table.delete(self_).unwrap().0;
        let next = self.table.delete(next).unwrap().0;
        self.table.push(HostTask(first.chain(next))).unwrap()
    }

    fn drop(&mut self, rep: Resource<HostTask>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
