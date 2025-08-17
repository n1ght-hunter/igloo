use std::borrow::Borrow;

use iced_core::{Length, Padding, Pixels, text};

use crate::{
    Element,
    bindings::iced::app::{self, element::pick_list_to_element},
    element::Widget,
};

/// A widget for selecting a value from a set of options.
pub struct PickList<T, L, V, Message>
where
    T: ToString + PartialEq + Clone,
    L: Borrow<[T]>,
    V: Borrow<T>,
{
    options: L,
    on_select: Box<dyn Fn(T) -> Message + Send + Sync>,
    on_open: Option<Message>,
    on_close: Option<Message>,
    placeholder: Option<String>,
    selected: Option<V>,
    width: Option<Length>,
    padding: Option<Padding>,
    text_size: Option<Pixels>,
    text_line_height: Option<text::LineHeight>,
    text_shaping: Option<text::Shaping>,
}

impl<T, L, V, Message> PickList<T, L, V, Message>
where
    T: ToString + PartialEq + Clone,
    L: Borrow<[T]>,
    V: Borrow<T>,
    Message: Clone,
{
    /// Creates a new [`PickList`] with the given options.
    pub fn new(
        options: L,
        selected: Option<V>,
        on_select: impl Fn(T) -> Message + Send + Sync + 'static,
    ) -> Self {
        Self {
            options,
            on_select: Box::new(on_select),
            selected,
            on_open: None,
            on_close: None,
            placeholder: None,
            width: None,
            padding: None,
            text_size: None,
            text_line_height: None,
            text_shaping: None,
        }
    }

    /// Sets the placeholder text of the [`PickList`].
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    pub fn text_size(mut self, size: impl Into<Pixels>) -> Self {
        self.text_size = Some(size.into());
        self
    }

    pub fn text_line_height(mut self, line_height: impl Into<text::LineHeight>) -> Self {
        self.text_line_height = Some(line_height.into());
        self
    }

    pub fn text_shaping(mut self, shaping: impl Into<text::Shaping>) -> Self {
        self.text_shaping = Some(shaping.into());
        self
    }

    /// Sets the message produced when the [`PickList`] is opened.
    pub fn on_open(mut self, message: Message) -> Self {
        self.on_open = Some(message);
        self
    }

    /// Sets the message produced when the [`PickList`] is closed.
    pub fn on_close(mut self, message: Message) -> Self {
        self.on_close = Some(message);
        self
    }
}

impl<T, L, V, Message> Widget<Message> for PickList<T, L, V, Message>
where
    T: ToString + PartialEq + Clone + Send + Sync + 'static,
    L: Borrow<[T]>,
    V: Borrow<T>,
    Message: Clone + 'static,
{
    fn as_element(
        self: Box<Self>,
        create_message: &dyn crate::element::CreateMessage<Message>,
    ) -> crate::bindings::Element {
        let str_options: Vec<String> = self
            .options
            .borrow()
            .into_iter()
            .map(|o| o.to_string())
            .collect();

        let options: &[T] = self.options.borrow();
        let options: Vec<T> = options.to_vec();
        let on_select = self.on_select;

        pick_list_to_element(&app::element::PickList {
            options: str_options,
            selected: self.selected.map(|s| s.borrow().to_string()),
            on_select: create_message.add_message_func(Box::new(move |v| {
                if let crate::bindings::Message::StringType(value) = v {
                    let v = options.iter().find(|o| o.to_string() == value)?;
                    Some((on_select)(v.clone()))
                } else {
                    None
                }
            })),
            on_open: self.on_open.map(|m| create_message.add_message(m)),
            on_close: self.on_close.map(|m| create_message.add_message(m)),
            placeholder: self.placeholder.clone(),
            width: self.width.map(Into::into),
            padding: self.padding.map(Into::into),
            text_size: self.text_size.map(Into::into),
            text_line_height: self.text_line_height.map(Into::into),
            text_shaping: self.text_shaping.map(Into::into),
        })
    }
}

impl<T, L, V, Message> From<PickList<T, L, V, Message>> for Element<Message>
where
    T: ToString + PartialEq + Clone + Send + Sync+ 'static,
    L: Borrow<[T]> + 'static,
    V: Borrow<T> + 'static,
    Message: Clone + 'static,
{
    fn from(value: PickList<T, L, V, Message>) -> Self {
        Element::new(Box::new(value))
    }
}
