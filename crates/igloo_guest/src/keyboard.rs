//! Keyboard streams supplied by the host.

use crate::{
    Subscription, Task,
    bindings::iced::app::capabilities::{self, KeyEvent},
};
use futures::StreamExt;

pub fn on_key_press() -> Subscription<String> {
    keys("keyboard:press", KeyEvent::Press)
}
pub fn on_key_release() -> Subscription<String> {
    keys("keyboard:release", KeyEvent::Release)
}

fn keys(key: &str, event: KeyEvent) -> Subscription<String> {
    Subscription::from_task(
        key,
        Task::new(move || match capabilities::keyboard(event) {
            Ok(reader) => reader.into_stream().map(Ok).boxed_local(),
            Err(error) => {
                futures::stream::once(async move { Err(crate::task::failed(error)) }).boxed_local()
            }
        }),
    )
}
