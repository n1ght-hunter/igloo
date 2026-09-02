use std::rc::Rc;

use crate::arena::Arena;
use crate::bindings::iced::app::message_types::Viewport;
use crate::bindings::iced::app::widgets::{ExplainNode, Node, NodeId};

/// Turns a widget's deferred callbacks into opaque callback ids.
///
/// [`Element::map`] composes an adapter implementing this trait for the source
/// `Message` type on top of the target's implementation, which is what lets
/// mapping work uniformly for a fixed message and every value-carrying mapper
/// kind. The only real implementation lives in [`crate::ApplicationResource`],
/// where the concrete `Application::Message` is known.
pub(crate) trait Realize<Message> {
    fn fixed(&self, msg: Message) -> u32;
    fn bool_mapper(&self, f: Box<dyn Fn(bool) -> Message>) -> u32;
    fn f32_mapper(&self, f: Box<dyn Fn(f32) -> Message>) -> u32;
    fn f64_mapper(&self, f: Box<dyn Fn(f64) -> Message>) -> u32;
    fn u64_mapper(&self, f: Box<dyn Fn(u64) -> Message>) -> u32;
    fn string_mapper(&self, f: Box<dyn Fn(String) -> Message>) -> u32;
    fn viewport_mapper(&self, f: Box<dyn Fn(Viewport) -> Message>) -> u32;
}

/// A view element whose node arena and message callbacks are not yet built.
///
/// Building is deferred until [`crate::ApplicationResource::view`], where the
/// concrete `Application::Message` type is known and every callback — fixed or
/// value-carrying — can be turned into a real callback id with no type erasure.
/// The closure pushes this element's subtree into [`crate::arena`] in post-order
/// and returns the id of its root node.
#[allow(missing_debug_implementations)]
pub struct Element<Message> {
    build: Box<dyn FnOnce(&dyn Realize<Message>, &mut Arena) -> NodeId>,
}

impl<Message: 'static> Element<Message> {
    pub(crate) fn new(
        build: impl FnOnce(&dyn Realize<Message>, &mut Arena) -> NodeId + 'static,
    ) -> Self {
        Self {
            build: Box::new(build),
        }
    }

    pub(crate) fn build(self, realize: &dyn Realize<Message>, arena: &mut Arena) -> NodeId {
        (self.build)(realize, arena)
    }

    /// Wraps the element with a debug overlay of the given color.
    pub fn explain(self, color: impl Into<crate::bindings::iced::app::shared::Color>) -> Self {
        let color = color.into();
        Element::new(move |realize, arena| {
            let content = self.build(realize, arena);
            arena.push(Node::Explain(ExplainNode { content, color }))
        })
    }

    /// Applies `f` to the messages produced by this element's subtree.
    ///
    /// Works uniformly for the fixed message a widget sends immediately (e.g.
    /// `on_press`) and for value-carrying callbacks (`on_toggle`, `on_input`,
    /// ...), which only resolve to a message once the user interacts with the
    /// widget — both are composed, not rewritten after the fact.
    pub fn map<B: 'static>(self, f: impl Fn(Message) -> B + 'static) -> Element<B> {
        let f: Rc<dyn Fn(Message) -> B> = Rc::new(f);
        Element::new(move |realize: &dyn Realize<B>, arena: &mut Arena| {
            let adapter = MapRealize { inner: realize, f };
            self.build(&adapter, arena)
        })
    }
}

pub(crate) struct MapRealize<'a, Message, B> {
    pub(crate) inner: &'a dyn Realize<B>,
    pub(crate) f: Rc<dyn Fn(Message) -> B>,
}

impl<Message: 'static, B: 'static> Realize<Message> for MapRealize<'_, Message, B> {
    fn fixed(&self, msg: Message) -> u32 {
        self.inner.fixed((self.f)(msg))
    }

    fn bool_mapper(&self, g: Box<dyn Fn(bool) -> Message>) -> u32 {
        let f = self.f.clone();
        self.inner.bool_mapper(Box::new(move |v| f(g(v))))
    }

    fn f32_mapper(&self, g: Box<dyn Fn(f32) -> Message>) -> u32 {
        let f = self.f.clone();
        self.inner.f32_mapper(Box::new(move |v| f(g(v))))
    }

    fn f64_mapper(&self, g: Box<dyn Fn(f64) -> Message>) -> u32 {
        let f = self.f.clone();
        self.inner.f64_mapper(Box::new(move |v| f(g(v))))
    }

    fn u64_mapper(&self, g: Box<dyn Fn(u64) -> Message>) -> u32 {
        let f = self.f.clone();
        self.inner.u64_mapper(Box::new(move |v| f(g(v))))
    }

    fn string_mapper(&self, g: Box<dyn Fn(String) -> Message>) -> u32 {
        let f = self.f.clone();
        self.inner.string_mapper(Box::new(move |v| f(g(v))))
    }

    fn viewport_mapper(&self, g: Box<dyn Fn(Viewport) -> Message>) -> u32 {
        let f = self.f.clone();
        self.inner.viewport_mapper(Box::new(move |v| f(g(v))))
    }
}
