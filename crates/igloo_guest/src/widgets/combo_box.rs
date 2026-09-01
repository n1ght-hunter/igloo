use iced_core::{Length, Padding, text};

use crate::Element;
use crate::bindings::iced::app::combo_box::ComboBox as WitComboBox;

#[derive(Debug, Clone)]
pub struct State<T> {
    options: Vec<T>,
    str_options: Vec<String>,
}

impl<T: std::fmt::Display + Clone> State<T> {
    pub fn new(options: Vec<T>) -> Self {
        Self {
            str_options: options
                .iter()
                .map(std::string::ToString::to_string)
                .collect(),
            options,
        }
    }

    fn str_options(&self) -> &Vec<String> {
        &self.str_options
    }

    // Parses a selection string back into T using a per-instance options
    // table, unlike std::str::FromStr::from_str which is a free function.
    #[allow(clippy::wrong_self_convention)]
    fn from_str(&self, input: &str) -> Option<T> {
        self.str_options
            .iter()
            .position(|s| s == input)
            .map(|index| self.options[index].clone())
    }
}

/// A widget allowing selection from a list of options.
pub struct ComboBox<T, Message> {
    options: State<T>,
    placeholder: String,
    selected: Option<String>,
    on_selected: Box<dyn Fn(T) -> Message>,
    on_input: Option<Box<dyn Fn(String) -> Message>>,
    on_option_hovered: Option<Box<dyn Fn(T) -> Message>>,
    on_open: Option<Message>,
    on_close: Option<Message>,
    padding: Option<Padding>,
    size: Option<f32>,
    line_height: Option<text::LineHeight>,
    width: Option<Length>,
}

impl<T, Message> ComboBox<T, Message>
where
    T: std::fmt::Display + Clone + 'static,
    Message: 'static,
{
    /// Creates a new [`ComboBox`] with the given options.
    pub fn new(
        options: &State<T>,
        placeholder: String,
        selection: Option<T>,
        on_selected: impl Fn(T) -> Message + 'static,
    ) -> Self {
        Self {
            options: options.clone(),
            placeholder,
            selected: selection.map(|s| s.to_string()),
            on_selected: Box::new(on_selected),
            on_input: None,
            on_option_hovered: None,
            on_open: None,
            on_close: None,
            padding: None,
            size: None,
            line_height: None,
            width: None,
        }
    }

    pub fn on_input(mut self, message: impl Fn(String) -> Message + 'static) -> Self {
        self.on_input = Some(Box::new(message));
        self
    }

    pub fn on_option_hovered(mut self, message: impl Fn(T) -> Message + 'static) -> Self {
        self.on_option_hovered = Some(Box::new(message));
        self
    }

    pub fn on_open(mut self, message: Message) -> Self {
        self.on_open = Some(message);
        self
    }

    pub fn on_close(mut self, message: Message) -> Self {
        self.on_close = Some(message);
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn line_height(mut self, line_height: impl Into<text::LineHeight>) -> Self {
        self.line_height = Some(line_height.into());
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }
}

impl<T: std::fmt::Display + Clone + 'static, Message: 'static> From<ComboBox<T, Message>>
    for Element<Message>
{
    fn from(combo_box: ComboBox<T, Message>) -> Self {
        Element::new(move |realize| {
            let lookup = combo_box.options.clone();
            let on_selected = combo_box.on_selected;
            let on_selected_mapper = realize.string_mapper(Box::new(move |value| {
                let selected = lookup
                    .from_str(&value)
                    .expect("combo box produced an unknown option");
                on_selected(selected)
            }));

            let raw = WitComboBox::new(
                combo_box.options.str_options(),
                &combo_box.placeholder,
                combo_box.selected.as_deref(),
                on_selected_mapper,
            );

            if let Some(on_input) = combo_box.on_input {
                raw.on_input(realize.string_mapper(on_input));
            }
            if let Some(on_option_hovered) = combo_box.on_option_hovered {
                let lookup = combo_box.options.clone();
                let mapper = realize.string_mapper(Box::new(move |value| {
                    let hovered = lookup
                        .from_str(&value)
                        .expect("combo box produced an unknown option");
                    on_option_hovered(hovered)
                }));
                raw.on_option_hovered(mapper);
            }
            if let Some(msg) = combo_box.on_open {
                raw.on_open(realize.fixed(msg));
            }
            if let Some(msg) = combo_box.on_close {
                raw.on_close(realize.fixed(msg));
            }
            if let Some(padding) = combo_box.padding {
                raw.padding(padding.into());
            }
            if let Some(size) = combo_box.size {
                raw.size(size);
            }
            if let Some(line_height) = combo_box.line_height {
                raw.line_height(line_height.into());
            }
            if let Some(width) = combo_box.width {
                raw.width(width.into());
            }

            WitComboBox::into_element(raw)
        })
    }
}
