use std::borrow::Borrow;

use iced_core::{Length, Padding, Pixels, text};

use crate::Element;
use crate::bindings::iced::app::pick_list::PickList as WitPickList;

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
        Element::new(move |realize| {
            let mapper = realize.string_mapper(pick_list.on_select);
            let raw = WitPickList::new(
                &pick_list.str_options,
                pick_list.selected.as_deref(),
                mapper,
            );
            if let Some(placeholder) = pick_list.placeholder {
                raw.placeholder(&placeholder);
            }
            if let Some(width) = pick_list.width {
                raw.width(width.into());
            }
            if let Some(padding) = pick_list.padding {
                raw.padding(padding.into());
            }
            if let Some(size) = pick_list.text_size {
                raw.text_size(size);
            }
            if let Some(lh) = pick_list.text_line_height {
                raw.text_line_height(lh.into());
            }
            if let Some(shaping) = pick_list.text_shaping {
                raw.text_shaping(shaping.into());
            }
            if let Some(msg) = pick_list.on_open {
                raw.on_open(realize.fixed(msg));
            }
            if let Some(msg) = pick_list.on_close {
                raw.on_close(realize.fixed(msg));
            }
            WitPickList::into_element(raw)
        })
    }
}
