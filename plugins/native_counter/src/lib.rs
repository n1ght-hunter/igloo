//! A minimal native plugin: a counter with increment/decrement buttons.
//!
//! Exists to exercise the `igloo_native` guest config as a real, separate
//! cdylib — distinct from the host binary that loads it.

use igloo_native::widgets::{
    Position, button, checkbox, column, combo_box, pick_list, progress_bar, row, scrollable,
    slider, text, text_input, tooltip,
};
use igloo_native::{Application, Element, Subscription, Task};
struct Counter {
    value: i64,
    ticks: u64,
    input: String,
    selected: Option<String>,
    enabled: bool,
    progress: f32,
}
#[derive(Debug)]
enum CounterMessage {
    Increment,
    Decrement,
    Tick,
    Input(String),
    Selected(String),
    Enabled(bool),
    Progress(f32),
    Delayed,
    Scroll,
}
impl Application for Counter {
    type Message = CounterMessage;
    fn new() -> (Self, Task<CounterMessage>) {
        (
            Self {
                value: 0,
                ticks: 0,
                input: String::new(),
                selected: None,
                enabled: true,
                progress: 25.0,
            },
            Task::none(),
        )
    }
    fn update(&mut self, message: CounterMessage) -> Task<CounterMessage> {
        match message {
            CounterMessage::Increment => self.value += 1,
            CounterMessage::Decrement => self.value -= 1,
            CounterMessage::Tick => self.ticks += 1,
            CounterMessage::Input(value) => self.input = value,
            CounterMessage::Selected(value) => self.selected = Some(value),
            CounterMessage::Enabled(value) => self.enabled = value,
            CounterMessage::Progress(value) => self.progress = value,
            CounterMessage::Delayed => {
                return Task::sleep(std::time::Duration::from_millis(500), || {
                    CounterMessage::Increment
                });
            }
            CounterMessage::Scroll => {}
        }
        Task::none()
    }
    fn view(&self) -> Element<CounterMessage> {
        let child: Element<bool> = button(text("Mapped increment")).on_press(|| true).into();
        let choices = vec!["One".to_owned(), "Two".to_owned(), "Three".to_owned()];
        let controls = column()
            .spacing(10.0)
            .push(text(format!(
                "Counter: {} | Ticks: {}",
                self.value, self.ticks
            )))
            .push(
                row()
                    .spacing(8.0)
                    .push(button(text("Increment")).on_press(|| CounterMessage::Increment))
                    .push(button(text("Decrement")).on_press(|| CounterMessage::Decrement))
                    .push(child.map(|_| CounterMessage::Increment))
                    .push(tooltip(
                        button(text("Delayed +1")).on_press(|| CounterMessage::Delayed),
                        text("Runs a sleep task"),
                        Position::Top,
                    )),
            )
            .push(text_input("Type here", self.input.clone()).on_input(CounterMessage::Input))
            .push(
                pick_list(choices.clone(), self.selected.clone())
                    .on_select(CounterMessage::Selected),
            )
            .push(
                combo_box(choices, "Select a value", self.selected.clone())
                    .on_selected(CounterMessage::Selected),
            )
            .push(
                checkbox(self.enabled)
                    .label("Timer enabled")
                    .on_toggle(CounterMessage::Enabled),
            )
            .push(slider(0.0, 100.0, self.progress).on_change(CounterMessage::Progress))
            .push(progress_bar(0.0, 100.0, self.progress));
        scrollable(controls)
            .height(450.0)
            .on_scroll(|_| CounterMessage::Scroll)
            .into()
    }
    fn subscription(&self) -> Subscription<CounterMessage> {
        if self.enabled {
            igloo_native::time::every(std::time::Duration::from_secs(1))
                .map(|_| CounterMessage::Tick)
        } else {
            Subscription::none()
        }
    }
}
igloo_native::export!(Counter);
