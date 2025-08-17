use crate::{
    Element,
    bindings::iced::app::element::pane_grid_to_element,
    element::Widget,
};

/// A layout that can dynamically split its content into panes.
#[derive(Debug)]
pub struct PaneGrid<Message> {
    children: Vec<Element<Message>>,
}

impl<Message> PaneGrid<Message> {
    /// Creates a new [`PaneGrid`] with the given children.
    pub fn new(children: impl IntoIterator<Item = Element<Message>>) -> Self {
        let children = children.into_iter().collect();
        Self { children }
    }
}

impl<Message> Widget<Message> for PaneGrid<Message> {
    fn as_element(
        self: Box<Self>,
        create_message: &dyn crate::element::CreateMessage<Message>,
    ) -> crate::bindings::Element {
        pane_grid_to_element(crate::bindings::iced::app::pane_grid::PaneGrid {
            children: self
                .children
                .into_iter()
                .map(|e| e.as_element(create_message))
                .collect(),
        })
    }
}

impl<Message: 'static> From<PaneGrid<Message>> for Element<Message> {
    fn from(value: PaneGrid<Message>) -> Self {
        Element::new(Box::new(value))
    }
}
