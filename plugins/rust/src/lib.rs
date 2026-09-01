use igloo_guest::{
    Element,
    widgets::{button, checkbox, column, text},
};

#[derive(Debug, Clone)]
pub enum Message {
    Clicked,
    Toggled(bool),
}

#[allow(missing_debug_implementations)]
pub struct App {
    count: u32,
    checked: bool,
}

impl igloo_guest::Application for App {
    type Message = Message;

    fn new() -> Self {
        App {
            count: 0,
            checked: false,
        }
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
            .spacing(10)
            .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Clicked => self.count += 1,
            Message::Toggled(value) => self.checked = value,
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
