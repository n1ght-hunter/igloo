
use iced_core::{
    Length, Padding,
    text::{self, LineHeight},
};

use crate::{
    Element,
    bindings::iced::app::element::combo_box_to_element,
    element::Widget,
};

#[derive(Debug, Clone)]
pub struct State<T> {
    options: Vec<T>,
    str_options: Vec<String>,
}

impl<T: std::fmt::Display + Clone> State<T> {
    pub fn new(options: Vec<T>) -> Self {
        Self {
            str_options: options.iter().map(|o| o.to_string()).collect(),
            options,
        }
    }

    fn str_options(&self) -> &Vec<String> {
        &self.str_options
    }

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
    selection: Option<T>,
    on_selected: Box<dyn Fn(T) -> Message>,
    on_input: Option<Box<dyn Fn(String) -> Message>>,
    on_option_hovered: Option<Box<dyn Fn(T) -> Message>>,
    on_open: Option<Message>,
    on_close: Option<Message>,
    padding: Option<Padding>,
    size: Option<f32>,
    line_height: Option<LineHeight>,
    width: Option<Length>,
}

impl<T, Message> ComboBox<T, Message>
where
    T: std::fmt::Display + Clone,
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
            selection,
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

impl<T, Message: Clone + 'static> Widget<Message> for ComboBox<T, Message>
where
    T: std::fmt::Display + Clone + 'static,
{
    fn as_element(
        self: Box<Self>,
        create_message: &dyn crate::element::CreateMessage<Message>,
    ) -> crate::bindings::Element {
        let on_selected = self.on_selected;
        let options = self.options;
        let options2 = options.clone();
        combo_box_to_element(&crate::bindings::iced::app::combo_box::ComboBox {
            options: options.str_options().to_owned(),
            placeholder: self.placeholder.to_string(),
            selected: self.selection.map(|s| s.to_string()),
            on_selected: create_message.add_message_func(Box::new(move |v| {
                if let crate::bindings::Message::StringType(value) = v {
                    Some((on_selected)(options.from_str(&value)?))
                } else {
                    None
                }
            })),
            on_input: self.on_input.map(|f| {
                create_message.add_message_func(Box::new(move |msg| {
                    if let crate::bindings::Message::StringType(value) = msg {
                        Some(f(value))
                    } else {
                        None
                    }
                }))
            }),
            on_option_hovered: self.on_option_hovered.map(|f| {
                create_message.add_message_func(Box::new(move |msg| {
                    if let crate::bindings::Message::StringType(value) = msg {
                        Some(f(options2.from_str(&value)?))
                    } else {
                        None
                    }
                }))
            }),
            on_open: self
                .on_open
                .as_ref()
                .map(|f| create_message.add_message((*f).clone())),
            on_close: self
                .on_close
                .as_ref()
                .map(|f| create_message.add_message((*f).clone())),
            padding: self.padding.map(Into::into),
            size: self.size.map(Into::into),
            line_height: self.line_height.map(Into::into),
            width: self.width.map(Into::into),
        })
    }
}

impl<T: std::fmt::Display + Clone + 'static, Message: Clone + 'static> From<ComboBox<T, Message>>
    for Element<Message>
{
    fn from(combo_box: ComboBox<T, Message>) -> Self {
        Element::new(Box::new(combo_box))
    }
}
