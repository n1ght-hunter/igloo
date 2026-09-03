use std::time::Duration;

use igloo_guest::{
    Element, Subscription, Task, time,
    widgets::{button, checkbox, column, text, text_input},
};

#[derive(Debug, Clone)]
pub enum Message {
    Clicked,
    Toggled(bool),
    DelayedPing,
    Pinged,
    ClipboardChanged(String),
    CopyClipboard,
    PasteClipboard,
    ClipboardPasted(String),
    Tick(u64),
    RunPerform,
    Performed(u64),
    RunStream,
    Streamed(u64),
}

#[allow(missing_debug_implementations)]
pub struct App {
    count: u32,
    checked: bool,
    pings: u32,
    clipboard: String,
    ticks: u32,
    performed: Option<u64>,
    stream_sum: u64,
}

impl igloo_guest::Application for App {
    type Message = Message;

    fn new() -> (Self, Task<Message>) {
        (
            App {
                count: 0,
                checked: false,
                pings: 0,
                clipboard: String::new(),
                ticks: 0,
                performed: None,
                stream_sum: 0,
            },
            Task::none(),
        )
    }

    fn view(&self) -> Element<Message> {
        column()
            .push(text(format!("Count: {}", self.count)))
            .push(button(text("Click me")).on_press(Message::Clicked))
            .push(
                checkbox(self.checked)
                    .label("Toggle me")
                    .on_toggle(Message::Toggled),
            )
            .push(inner_checkbox(self.checked).map(InnerMessage::into_message))
            .push(text(format!("Pings: {}", self.pings)))
            .push(text(format!("Ticks: {}", self.ticks)))
            .push(text(match self.performed {
                Some(value) => format!("Performed: {value}"),
                None => "Performed: -".to_string(),
            }))
            .push(button(text("Perform")).on_press(Message::RunPerform))
            .push(text(format!("Stream sum: {}", self.stream_sum)))
            .push(button(text("Run stream")).on_press(Message::RunStream))
            .push(button(text("Ping in 2s")).on_press(Message::DelayedPing))
            .push(text_input("Clipboard text", &self.clipboard).on_input(Message::ClipboardChanged))
            .push(button(text("Copy")).on_press(Message::CopyClipboard))
            .push(button(text("Paste")).on_press(Message::PasteClipboard))
            .spacing(10)
            .into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Clicked => {
                self.count += 1;
                Task::none()
            }
            Message::Toggled(value) => {
                self.checked = value;
                Task::none()
            }
            Message::DelayedPing => Task::sleep(Duration::from_secs(2), Message::Pinged),
            Message::Pinged => {
                self.pings += 1;
                Task::none()
            }
            Message::ClipboardChanged(value) => {
                self.clipboard = value;
                Task::none()
            }
            Message::CopyClipboard => Task::write_clipboard(self.clipboard.clone()),
            Message::PasteClipboard => Task::read_clipboard(|result| {
                Message::ClipboardPasted(result.ok().flatten().unwrap_or_default())
            }),
            Message::ClipboardPasted(value) => {
                self.clipboard = value;
                Task::none()
            }
            Message::Tick(_) => {
                self.ticks += 1;
                Task::none()
            }
            Message::RunPerform => Task::perform(async { 6 * 7 }, Message::Performed),
            Message::Performed(value) => {
                self.performed = Some(value);
                Task::none()
            }
            Message::RunStream => Task::run(futures::stream::iter(1..=5u64), Message::Streamed),
            Message::Streamed(value) => {
                self.stream_sum += value;
                Task::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_secs(1)).map(Message::Tick)
    }
}

#[derive(Debug, Clone)]
enum InnerMessage {
    Toggled(bool),
}

impl InnerMessage {
    fn into_message(self) -> Message {
        match self {
            InnerMessage::Toggled(value) => Message::Toggled(value),
        }
    }
}

fn inner_checkbox(checked: bool) -> Element<InnerMessage> {
    checkbox(checked)
        .label("Mapped toggle")
        .on_toggle(InnerMessage::Toggled)
        .into()
}

igloo_guest::export_guest!(App);
