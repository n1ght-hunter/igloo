//! Component fixture for job ordering, retention and cancellation tests.

use igloo_guest::{
    Application, Element, Subscription, Task,
    widgets::{button, column, text},
};
use std::time::Duration;

#[derive(Debug)]
pub struct Probe {
    log: Vec<u32>,
    enabled: bool,
    revision: u64,
}
#[derive(Debug)]
pub enum Message {
    Value(u32),
    Chain,
    Refresh,
    Revise,
    Remove,
    Add,
    Pending,
    Invalid,
}

impl Application for Probe {
    type Message = Message;
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                log: Vec::new(),
                enabled: true,
                revision: 0,
            },
            Task::none(),
        )
    }
    fn view(&self) -> Element<Message> {
        column()
            .push(text(format!("{:?}", self.log)))
            .push(button(text("chain")).on_press(Message::Chain))
            .push(button(text("refresh")).on_press(Message::Refresh))
            .push(button(text("revise")).on_press(Message::Revise))
            .push(button(text("remove")).on_press(Message::Remove))
            .push(button(text("add")).on_press(Message::Add))
            .push(button(text("pending")).on_press(Message::Pending))
            .push(button(text("invalid")).on_press(Message::Invalid))
            .into()
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Value(value) => self.log.push(value),
            Message::Chain => {
                return Task::sleep(Duration::from_millis(20), 1)
                    .chain(Task::run(futures::stream::iter([2, 3]), |n| n))
                    .chain(Task::perform(async { 4 }, |n| n))
                    .map(|n| n * 2)
                    .map(|n| Message::Value(n / 2));
            }
            Message::Refresh => {}
            Message::Revise => self.revision += 1,
            Message::Remove => self.enabled = false,
            Message::Add => self.enabled = true,
            Message::Pending => {
                return Task::perform(futures::future::pending::<u32>(), Message::Value);
            }
            Message::Invalid => {
                return Task::run(futures::stream::pending::<u32>(), Message::Value);
            }
        }
        Task::none()
    }
    fn subscription(&self) -> Subscription<Message> {
        if self.enabled {
            Subscription::run("finite", || futures::stream::iter([Message::Value(100)]))
                .with_revision(self.revision)
        } else {
            Subscription::none()
        }
    }
}

igloo_guest::export_guest!(Probe);
