//! Host timer subscriptions. Timestamps are epoch milliseconds.

use crate::{Subscription, Task, bindings::iced::app::capabilities};
use futures::StreamExt;
use std::time::Duration;

pub fn every(interval: Duration) -> Subscription<u64> {
    let key = format!("timer:{}:{}", interval.as_secs(), interval.subsec_nanos());
    Subscription::from_task(
        key,
        Task::new(
            move || match capabilities::timer(crate::duration(interval)) {
                Ok(reader) => reader.into_stream().map(Ok).boxed_local(),
                Err(error) => futures::stream::once(async move { Err(crate::task::failed(error)) })
                    .boxed_local(),
            },
        ),
    )
}
