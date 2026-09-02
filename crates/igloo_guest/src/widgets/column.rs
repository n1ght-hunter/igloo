use std::marker::PhantomData;

use iced_core::{Length, Padding, Pixels};

use crate::Element;
use crate::bindings::iced::app::widgets::{ColumnNode, Node};

pub struct Column<Message> {
    children: Vec<Element<Message>>,
    spacing: Option<f32>,
    padding: Option<Padding>,
    width: Option<Length>,
    height: Option<Length>,
    max_width: Option<f32>,
    align_x: Option<iced_core::alignment::Horizontal>,
    clip: Option<bool>,
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
            children: Vec::new(),
            spacing: None,
            padding: None,
            width: None,
            height: None,
            max_width: None,
            align_x: None,
            clip: None,
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

    pub fn spacing(mut self, amount: impl Into<Pixels>) -> Self {
        self.spacing = Some(amount.into().0);
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
        self.max_width = Some(max_width.into().0);
        self
    }

    pub fn align_x(mut self, align: impl Into<iced_core::alignment::Horizontal>) -> Self {
        self.align_x = Some(align.into());
        self
    }

    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = Some(clip);
        self
    }
}

impl<Message: 'static> From<Column<Message>> for Element<Message> {
    fn from(column: Column<Message>) -> Self {
        Element::new(move |realize, arena| {
            let children = column
                .children
                .into_iter()
                .map(|child| child.build(realize, arena))
                .collect();
            let node = ColumnNode {
                children,
                spacing: column.spacing,
                padding: column.padding.map(Into::into),
                width: column.width.map(Into::into),
                height: column.height.map(Into::into),
                max_width: column.max_width,
                align_x: column.align_x.map(Into::into),
                clip: column.clip,
            };
            arena.push(Node::Column(node))
        })
    }
}
