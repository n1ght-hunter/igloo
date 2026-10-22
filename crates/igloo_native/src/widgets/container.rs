//! A box that contains another element.

use stabby::option::Option as Opt;

use crate::widgets::element::{Arena, Element, Realize};
use crate::widgets::types::{Horizontal, Length, Padding, Vertical};
use crate::widgets::{Node, NodeId};

/// A box that contains another element.
#[stabby::stabby]
#[derive(Debug)]
pub struct ContainerNode {
    pub content: NodeId,
    pub padding: Opt<Padding>,
    pub width: Opt<Length>,
    pub height: Opt<Length>,
    pub max_width: Opt<f32>,
    pub max_height: Opt<f32>,
    pub center_x: Opt<Length>,
    pub center_y: Opt<Length>,
    pub center: Opt<Length>,
    pub align_left: Opt<Length>,
    pub align_right: Opt<Length>,
    pub align_top: Opt<Length>,
    pub align_bottom: Opt<Length>,
    pub align_x: Opt<Horizontal>,
    pub align_y: Opt<Vertical>,
    pub clip: Opt<bool>,
}

#[cfg(feature = "host")]
pub(crate) fn build<'a>(
    nodes: &mut crate::widgets::RenderArena<'a>,
    container: ContainerNode,
) -> impl iced::Widget<crate::widgets::Emitted> + 'a {
    let content = crate::widgets::build_node(nodes, container.content);
    let mut widget = iced::widget::Container::new(content);
    if let Some(padding) = Option::<Padding>::from(container.padding) {
        widget = widget.padding(iced::Padding::from(padding));
    }
    let mut width: Option<iced::Length> = Option::<Length>::from(container.width).map(Into::into);
    if let Some(max_width) = Option::<f32>::from(container.max_width) {
        width = Some(width.unwrap_or(iced::Length::Shrink).max(max_width));
    }
    if let Some(width) = width {
        widget = widget.width(width);
    }
    let mut height: Option<iced::Length> = Option::<Length>::from(container.height).map(Into::into);
    if let Some(max_height) = Option::<f32>::from(container.max_height) {
        height = Some(height.unwrap_or(iced::Length::Shrink).max(max_height));
    }
    if let Some(height) = height {
        widget = widget.height(height);
    }
    if let Some(width) = Option::<Length>::from(container.center_x) {
        widget = widget.center_x(iced::Length::from(width));
    }
    if let Some(height) = Option::<Length>::from(container.center_y) {
        widget = widget.center_y(iced::Length::from(height));
    }
    if let Some(length) = Option::<Length>::from(container.center) {
        widget = widget.center(iced::Length::from(length));
    }
    if let Some(width) = Option::<Length>::from(container.align_left) {
        widget = widget.align_left(iced::Length::from(width));
    }
    if let Some(width) = Option::<Length>::from(container.align_right) {
        widget = widget.align_right(iced::Length::from(width));
    }
    if let Some(height) = Option::<Length>::from(container.align_top) {
        widget = widget.align_top(iced::Length::from(height));
    }
    if let Some(height) = Option::<Length>::from(container.align_bottom) {
        widget = widget.align_bottom(iced::Length::from(height));
    }
    if let Some(align_x) = Option::<Horizontal>::from(container.align_x) {
        widget = widget.align_x(iced::alignment::Horizontal::from(align_x));
    }
    if let Some(align_y) = Option::<Vertical>::from(container.align_y) {
        widget = widget.align_y(iced::alignment::Vertical::from(align_y));
    }
    if let Some(clip) = Option::<bool>::from(container.clip) {
        widget = widget.clip(clip);
    }
    widget
}

/// A builder for a [`ContainerNode`].
pub struct Container<Message> {
    content: Element<Message>,
    padding: Option<Padding>,
    width: Option<Length>,
    height: Option<Length>,
    max_width: Option<f32>,
    max_height: Option<f32>,
    center_x: Option<Length>,
    center_y: Option<Length>,
    center: Option<Length>,
    align_left: Option<Length>,
    align_right: Option<Length>,
    align_top: Option<Length>,
    align_bottom: Option<Length>,
    align_x: Option<Horizontal>,
    align_y: Option<Vertical>,
    clip: Option<bool>,
}

impl<Message> Container<Message> {
    /// Creates a container wrapping `content`.
    pub fn new(content: impl Into<Element<Message>>) -> Self {
        Self {
            content: content.into(),
            padding: None,
            width: None,
            height: None,
            max_width: None,
            max_height: None,
            center_x: None,
            center_y: None,
            center: None,
            align_left: None,
            align_right: None,
            align_top: None,
            align_bottom: None,
            align_x: None,
            align_y: None,
            clip: None,
        }
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

    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = Some(max_width);
        self
    }

    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = Some(max_height);
        self
    }

    /// Sets the width and centers the content horizontally.
    pub fn center_x(mut self, width: impl Into<Length>) -> Self {
        self.center_x = Some(width.into());
        self
    }

    /// Sets the height and centers the content vertically.
    pub fn center_y(mut self, height: impl Into<Length>) -> Self {
        self.center_y = Some(height.into());
        self
    }

    /// Centers the content in both axes.
    pub fn center(mut self, length: impl Into<Length>) -> Self {
        self.center = Some(length.into());
        self
    }

    pub fn align_left(mut self, width: impl Into<Length>) -> Self {
        self.align_left = Some(width.into());
        self
    }

    pub fn align_right(mut self, width: impl Into<Length>) -> Self {
        self.align_right = Some(width.into());
        self
    }

    pub fn align_top(mut self, height: impl Into<Length>) -> Self {
        self.align_top = Some(height.into());
        self
    }

    pub fn align_bottom(mut self, height: impl Into<Length>) -> Self {
        self.align_bottom = Some(height.into());
        self
    }

    pub fn align_x(mut self, align_x: Horizontal) -> Self {
        self.align_x = Some(align_x);
        self
    }

    pub fn align_y(mut self, align_y: Vertical) -> Self {
        self.align_y = Some(align_y);
        self
    }

    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = Some(clip);
        self
    }
}

impl<Message> std::fmt::Debug for Container<Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Container")
            .field("content", &self.content)
            .field("padding", &self.padding)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("max_width", &self.max_width)
            .field("max_height", &self.max_height)
            .field("center_x", &self.center_x)
            .field("center_y", &self.center_y)
            .field("center", &self.center)
            .field("align_left", &self.align_left)
            .field("align_right", &self.align_right)
            .field("align_top", &self.align_top)
            .field("align_bottom", &self.align_bottom)
            .field("align_x", &self.align_x)
            .field("align_y", &self.align_y)
            .field("clip", &self.clip)
            .finish()
    }
}

impl<Message: 'static> From<Container<Message>> for Element<Message> {
    fn from(container: Container<Message>) -> Self {
        Element::new(move |realize: &dyn Realize<Message>, arena: &mut Arena| {
            let content = container.content.build(realize, arena);
            arena.push(Node::Container(ContainerNode {
                content,
                padding: container.padding.into(),
                width: container.width.into(),
                height: container.height.into(),
                max_width: container.max_width.into(),
                max_height: container.max_height.into(),
                center_x: container.center_x.into(),
                center_y: container.center_y.into(),
                center: container.center.into(),
                align_left: container.align_left.into(),
                align_right: container.align_right.into(),
                align_top: container.align_top.into(),
                align_bottom: container.align_bottom.into(),
                align_x: container.align_x.into(),
                align_y: container.align_y.into(),
                clip: container.clip.into(),
            }))
        })
    }
}

/// Creates a [`Container`] wrapping `content`.
pub fn container<Message>(content: impl Into<Element<Message>>) -> Container<Message> {
    Container::new(content)
}
