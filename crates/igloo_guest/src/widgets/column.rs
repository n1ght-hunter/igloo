use std::marker::PhantomData;

use iced_core::{Length, Padding, Pixels};

use crate::Element;
use crate::bindings::iced::app::column::Column as WitColumn;

pub struct Column<Message> {
    raw: WitColumn,
    children: Vec<Element<Message>>,
    _message: PhantomData<Message>,
}

impl<Message> Default for Column<Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Message> Column<Message> {
    pub fn new() -> Self {
        Self {
            raw: WitColumn::new(),
            children: Vec::new(),
            _message: PhantomData,
        }
    }

    pub fn with_children(children: impl IntoIterator<Item = Element<Message>>) -> Self {
        children.into_iter().fold(Self::new(), Self::push)
    }

    pub fn push(mut self, child: impl Into<Element<Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn push_maybe(self, child: Option<impl Into<Element<Message>>>) -> Self {
        match child {
            Some(child) => self.push(child),
            None => self,
        }
    }

    pub fn spacing(self, amount: impl Into<Pixels>) -> Self {
        self.raw.spacing(amount.into().0);
        self
    }

    pub fn padding(self, padding: impl Into<Padding>) -> Self {
        self.raw.padding(padding.into().into());
        self
    }

    pub fn width(self, width: impl Into<Length>) -> Self {
        self.raw.width(width.into().into());
        self
    }

    pub fn height(self, height: impl Into<Length>) -> Self {
        self.raw.height(height.into().into());
        self
    }

    pub fn max_width(self, max_width: impl Into<Pixels>) -> Self {
        self.raw.max_width(max_width.into().0);
        self
    }

    pub fn align_x(self, align: impl Into<iced_core::alignment::Horizontal>) -> Self {
        self.raw.align_x(align.into().into());
        self
    }

    pub fn clip(self, clip: bool) -> Self {
        self.raw.clip(clip);
        self
    }
}

impl<Message: 'static> From<Column<Message>> for Element<Message> {
    fn from(column: Column<Message>) -> Self {
        Element::new(move |realize| {
            for child in column.children {
                column.raw.push(child.build(realize));
            }
            WitColumn::into_element(column.raw)
        })
    }
}
