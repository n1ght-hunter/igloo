//! A scrollable container for a single child node.

use stabby::option::Option as Opt;

use crate::messages::EmitViewport;
use crate::widgets::element::{Arena, Element, Realize};
use crate::widgets::types::{Anchor, Direction, Length, Scrollbar, Viewport};
use crate::widgets::{Node, NodeId};

/// A scrollable container for a single child node.
#[stabby::stabby]
pub struct ScrollableNode {
    pub content: NodeId,
    pub width: Opt<Length>,
    pub height: Opt<Length>,
    pub direction: Opt<Direction>,
    pub on_scroll: Opt<EmitViewport>,
}

impl std::fmt::Debug for ScrollableNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScrollableNode")
            .field("content", &self.content)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("direction", &self.direction)
            .field("on_scroll", &self.on_scroll.as_ref().map(|_| "<opaque>"))
            .finish()
    }
}

#[cfg(feature = "host")]
pub(crate) fn build<'a>(
    nodes: &mut crate::widgets::RenderArena<'a>,
    node: ScrollableNode,
) -> iced::Element<'a, crate::widgets::Emitted> {
    let content = crate::widgets::build_node(nodes, node.content);
    let mut widget = iced::widget::Scrollable::new(content);

    if let Some(width) = Option::<Length>::from(node.width) {
        widget = widget.width(iced::Length::from(width));
    }
    if let Some(height) = Option::<Length>::from(node.height) {
        widget = widget.height(iced::Length::from(height));
    }
    if let Some(direction) = Option::<Direction>::from(node.direction) {
        widget = widget.direction(iced::widget::scrollable::Direction::from(direction));
    }
    if let Some(emit) = Option::from(node.on_scroll) {
        let emit = std::rc::Rc::new(emit);
        widget = widget.on_scroll(move |viewport| {
            crate::widgets::Emitted::viewport(&emit, Viewport::from(viewport))
        });
    }

    widget.into()
}

/// A builder for a [`ScrollableNode`].
pub struct Scrollable<Message> {
    content: Element<Message>,
    width: Option<Length>,
    height: Option<Length>,
    direction: Option<Direction>,
    on_scroll: Option<std::boxed::Box<dyn Fn(Viewport) -> Message>>,
}

impl<Message> Scrollable<Message> {
    /// Creates a scrollable wrapping `content`.
    pub fn new(content: impl Into<Element<Message>>) -> Self {
        Self {
            content: content.into(),
            width: None,
            height: None,
            direction: None,
            on_scroll: None,
        }
    }

    /// Creates a scrollable wrapping `content` with the given [`Direction`].
    pub fn with_direction(content: impl Into<Element<Message>>, direction: Direction) -> Self {
        let mut this = Self::new(content);
        this.direction = Some(direction);
        this
    }

    /// Makes the scrollable scroll horizontally, with default scrollbar settings.
    pub fn horizontal(self) -> Self {
        self.direction(Direction::Horizontal(Scrollbar::default()))
    }

    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = Some(direction);
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

    /// Sets the message emitted, with the current [`Viewport`], on every scroll.
    pub fn on_scroll(mut self, emit: impl Fn(Viewport) -> Message + 'static) -> Self {
        self.on_scroll = Some(std::boxed::Box::new(emit));
        self
    }

    /// Anchors the vertical direction to the top.
    pub fn anchor_top(self) -> Self {
        self.anchor_y(Anchor::Start)
    }

    /// Anchors the vertical direction to the bottom.
    pub fn anchor_bottom(self) -> Self {
        self.anchor_y(Anchor::End)
    }

    /// Anchors the horizontal direction to the left.
    pub fn anchor_left(self) -> Self {
        self.anchor_x(Anchor::Start)
    }

    /// Anchors the horizontal direction to the right.
    pub fn anchor_right(self) -> Self {
        self.anchor_x(Anchor::End)
    }

    /// Sets the [`Anchor`] of the horizontal direction, if applicable.
    pub fn anchor_x(mut self, anchor: Anchor) -> Self {
        let direction = self
            .direction
            .get_or_insert_with(|| Direction::Vertical(Scrollbar::default()));
        match direction {
            Direction::Horizontal(scrollbar) | Direction::Both(_, scrollbar) => {
                scrollbar.anchor = Some(anchor).into();
            }
            Direction::Vertical(_) => {}
        }
        self
    }

    /// Sets the [`Anchor`] of the vertical direction, if applicable.
    pub fn anchor_y(mut self, anchor: Anchor) -> Self {
        let direction = self
            .direction
            .get_or_insert_with(|| Direction::Vertical(Scrollbar::default()));
        match direction {
            Direction::Vertical(scrollbar) | Direction::Both(scrollbar, _) => {
                scrollbar.anchor = Some(anchor).into();
            }
            Direction::Horizontal(_) => {}
        }
        self
    }

    /// Embeds the scrollbar into the scrollable, using `spacing` between it and the content.
    pub fn spacing(mut self, spacing: f32) -> Self {
        let direction = self
            .direction
            .get_or_insert_with(|| Direction::Vertical(Scrollbar::default()));
        match direction {
            Direction::Horizontal(scrollbar) | Direction::Vertical(scrollbar) => {
                scrollbar.spacing = Some(spacing).into();
            }
            Direction::Both(..) => {}
        }
        self
    }
}

impl<Message> std::fmt::Debug for Scrollable<Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scrollable")
            .field("content", &self.content)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("direction", &self.direction)
            .field("on_scroll", &self.on_scroll.as_ref().map(|_| "<opaque>"))
            .finish()
    }
}

impl<Message: 'static> From<Scrollable<Message>> for Element<Message> {
    fn from(scrollable: Scrollable<Message>) -> Self {
        Element::new(move |realize: &dyn Realize<Message>, arena: &mut Arena| {
            let content = scrollable.content.build(realize, arena);
            let on_scroll = scrollable
                .on_scroll
                .map(|emit| realize.viewport_mapper(emit));
            arena.push(Node::Scrollable(ScrollableNode {
                content,
                width: scrollable.width.into(),
                height: scrollable.height.into(),
                direction: scrollable.direction.into(),
                on_scroll: on_scroll.into(),
            }))
        })
    }
}

/// Creates a [`Scrollable`] wrapping `content`.
pub fn scrollable<Message>(content: impl Into<Element<Message>>) -> Scrollable<Message> {
    Scrollable::new(content)
}
