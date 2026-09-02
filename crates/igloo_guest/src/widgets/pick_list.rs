use std::borrow::Borrow;

use iced_core::{Length, Padding, Pixels, text};

use crate::Element;
use crate::bindings::iced::app::widgets::{Node, PickListNode};

/// A widget for selecting a value from a set of options.
pub struct PickList<Message> {
    str_options: Vec<String>,
    selected: Option<String>,
    on_select: Box<dyn Fn(String) -> Message>,
    placeholder: Option<String>,
    width: Option<Length>,
    padding: Option<Padding>,
    text_size: Option<f32>,
    text_line_height: Option<text::LineHeight>,
    text_shaping: Option<text::Shaping>,
    on_open: Option<Message>,
    on_close: Option<Message>,
}

impl<Message: 'static> PickList<Message> {
    /// Creates a new [`PickList`] with the given options.
    pub fn new<T, L, V>(
        options: L,
        selected: Option<V>,
        on_select: impl Fn(T) -> Message + 'static,
    ) -> Self
    where
        T: ToString + PartialEq + Clone + 'static,
        L: Borrow<[T]>,
        V: Borrow<T>,
    {
        let str_options: Vec<String> = options
            .borrow()
            .iter()
            .map(std::string::ToString::to_string)
            .collect();

        let options: Vec<T> = options.borrow().to_vec();
        let selected = selected.map(|s| s.borrow().to_string());

        let on_select = Box::new(move |value: String| {
            let selected = options
                .iter()
                .find(|o| o.to_string() == value)
                .expect("pick list produced an unknown option")
                .clone();
            on_select(selected)
        });

        Self {
            str_options,
            selected,
            on_select,
            placeholder: None,
            width: None,
            padding: None,
            text_size: None,
            text_line_height: None,
            text_shaping: None,
            on_open: None,
            on_close: None,
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
        self.text_size = Some(size.into().0);
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

impl<Message: 'static> From<PickList<Message>> for Element<Message> {
    fn from(pick_list: PickList<Message>) -> Self {
        Element::new(move |realize, arena| {
            let node = PickListNode {
                options: pick_list.str_options,
                selected: pick_list.selected,
                on_select: realize.string_mapper(pick_list.on_select),
                placeholder: pick_list.placeholder,
                width: pick_list.width.map(Into::into),
                padding: pick_list.padding.map(Into::into),
                text_size: pick_list.text_size,
                text_line_height: pick_list.text_line_height.map(Into::into),
                text_shaping: pick_list.text_shaping.map(Into::into),
                on_open: pick_list.on_open.map(|msg| realize.fixed(msg)),
                on_close: pick_list.on_close.map(|msg| realize.fixed(msg)),
            };
            arena.push(Node::PickList(node))
        })
    }
}
