use iced_core::{Length, Pixels};

use crate::{
    Element,
    bindings::iced::app::element::table_to_element,
    element::Widget,
};

/// A grid-like visual representation of data distributed in columns and rows.
#[derive(Debug)]
pub struct Table<Message> {
    columns: Vec<Element<Message>>,
    rows: Vec<Element<Message>>,
    width: Option<Length>,
    padding: Option<Pixels>,
    padding_x: Option<Pixels>,
    padding_y: Option<Pixels>,
    separator: Option<Pixels>,
    separator_x: Option<Pixels>,
    separator_y: Option<Pixels>,
}

impl<Message> Table<Message> {
    /// Creates a new [`Table`] with the given columns.
    pub fn new(columns: impl IntoIterator<Item = Element<Message>>) -> Self {
        Self {
            columns: columns.into_iter().collect(),
            rows: Vec::new(),
            width: None,
            padding: None,
            padding_x: None,
            padding_y: None,
            separator: None,
            separator_x: None,
            separator_y: None,
        }
    }

    /// Adds a row to the [`Table`].
    pub fn push_row(mut self, row: impl Into<Element<Message>>) -> Self {
        self.rows.push(row.into());
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn padding(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    pub fn padding_x(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding_x = Some(padding.into());
        self
    }

    pub fn padding_y(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding_y = Some(padding.into());
        self
    }

    pub fn separator(mut self, sep: impl Into<Pixels>) -> Self {
        self.separator = Some(sep.into());
        self
    }

    pub fn separator_x(mut self, sep: impl Into<Pixels>) -> Self {
        self.separator_x = Some(sep.into());
        self
    }

    pub fn separator_y(mut self, sep: impl Into<Pixels>) -> Self {
        self.separator_y = Some(sep.into());
        self
    }
}

impl<Message> Widget<Message> for Table<Message> {
    fn as_element(
        self: Box<Self>,
        create_message: &dyn crate::element::CreateMessage<Message>,
    ) -> crate::bindings::Element {
        table_to_element(crate::bindings::iced::app::table::Table {
            columns: self
                .columns
                .into_iter()
                .map(|c| c.as_element(create_message))
                .collect(),
            rows: self
                .rows
                .into_iter()
                .map(|r| r.as_element(create_message))
                .collect(),
            width: self.width.map(Into::into),
            padding: self.padding.map(Into::into),
            padding_x: self.padding_x.map(Into::into),
            padding_y: self.padding_y.map(Into::into),
            separator: self.separator.map(Into::into),
            separator_x: self.separator_x.map(Into::into),
            separator_y: self.separator_y.map(Into::into),
        })
    }
}

impl<Message: 'static> From<Table<Message>> for Element<Message> {
    fn from(value: Table<Message>) -> Self {
        Element::new(Box::new(value))
    }
}
