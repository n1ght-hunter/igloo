use igloo_guest::{
    Element, Task,
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
}

#[allow(missing_debug_implementations)]
pub struct App {
    count: u32,
    checked: bool,
    pings: u32,
    clipboard: String,
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
            Message::DelayedPing => Task::sleep(2000, Message::Pinged),
            Message::Pinged => {
                self.pings += 1;
                Task::none()
            }
            Message::ClipboardChanged(value) => {
                self.clipboard = value;
                Task::none()
            }
            Message::CopyClipboard => Task::write_clipboard(self.clipboard.clone()),
            Message::PasteClipboard => Task::read_clipboard(Message::ClipboardPasted),
            Message::ClipboardPasted(value) => {
                self.clipboard = value;
                Task::none()
            }
        }
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
