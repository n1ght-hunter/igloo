//! The builder substrate: a not-yet-built `Element<Message>` pushed into an [`Arena`].

use crate::messages::{EmitBool, EmitF32, EmitString, EmitViewport};
use crate::widgets::types::Viewport;
use crate::widgets::{Node, NodeId, NodeTree};
use crate::{Emit, messages::Message as WireMessage};

/// Accumulates nodes in post-order; [`Arena::push`] returns the new node's id.
pub(crate) struct Arena {
    nodes: std::vec::Vec<Node>,
}

impl Arena {
    fn new() -> Self {
        Self {
            nodes: std::vec::Vec::new(),
        }
    }

    /// Appends `node` and returns its id.
    pub(crate) fn push(&mut self, node: Node) -> NodeId {
        let id = self.nodes.len() as NodeId;
        self.nodes.push(node);
        id
    }
}

/// Turns a widget's deferred message-producing closures into the ABI's opaque,
/// erased callbacks ([`Emit`] and friends).
///
/// [`Element::map`] composes an adapter implementing this trait for the source
/// `Message` type on top of the target's implementation, which is what lets
/// mapping work uniformly for a fixed message and every value-carrying
/// callback kind. The only real implementation is [`RootRealize`], built once
/// [`Element::into_tree`] knows the concrete plugin `Message` type.
pub(crate) trait Realize<Message> {
    fn fixed(&self, f: std::boxed::Box<dyn Fn() -> Message>) -> Emit;
    fn bool_mapper(&self, f: std::boxed::Box<dyn Fn(bool) -> Message>) -> EmitBool;
    fn f32_mapper(&self, f: std::boxed::Box<dyn Fn(f32) -> Message>) -> EmitF32;
    fn string_mapper(
        &self,
        f: std::boxed::Box<dyn Fn(std::string::String) -> Message>,
    ) -> EmitString;
    fn viewport_mapper(&self, f: std::boxed::Box<dyn Fn(Viewport) -> Message>) -> EmitViewport;
}

/// The [`Realize`] implementation used at the root of a view tree, where the
/// concrete plugin `Message` is known and every callback is erased into the
/// ABI's opaque [`crate::messages::Message`] via [`crate::messages::Message::new`].
struct RootRealize<Message> {
    _marker: std::marker::PhantomData<fn() -> Message>,
}

impl<Message: Send + 'static> Realize<Message> for RootRealize<Message> {
    fn fixed(&self, f: std::boxed::Box<dyn Fn() -> Message>) -> Emit {
        stabby::boxed::Box::new(move || WireMessage::new(f())).into()
    }

    fn bool_mapper(&self, f: std::boxed::Box<dyn Fn(bool) -> Message>) -> EmitBool {
        stabby::boxed::Box::new(move |value| WireMessage::new(f(value))).into()
    }

    fn f32_mapper(&self, f: std::boxed::Box<dyn Fn(f32) -> Message>) -> EmitF32 {
        stabby::boxed::Box::new(move |value| WireMessage::new(f(value))).into()
    }

    fn string_mapper(
        &self,
        f: std::boxed::Box<dyn Fn(std::string::String) -> Message>,
    ) -> EmitString {
        stabby::boxed::Box::new(move |value: stabby::string::String| {
            WireMessage::new(f(std::string::String::from(value)))
        })
        .into()
    }

    fn viewport_mapper(&self, f: std::boxed::Box<dyn Fn(Viewport) -> Message>) -> EmitViewport {
        stabby::boxed::Box::new(move |value| WireMessage::new(f(value))).into()
    }
}

/// A view element whose node arena and message callbacks are not yet built.
///
/// Building is deferred until [`Element::into_tree`], where the concrete
/// plugin `Message` type is known and every callback — fixed or
/// value-carrying — can be erased into the ABI's opaque [`crate::messages::Message`]
/// with no manual boxing on the plugin's part. Every widget's `From` impl
/// builds its children first, then itself, so a [`NodeId`] always flows from
/// exactly one child build into exactly one parent — the tree a builder
/// produces cannot become the kind of DAG that broke the old hand-indexed
/// arena.
pub struct Element<Message> {
    build: std::boxed::Box<dyn FnOnce(&dyn Realize<Message>, &mut Arena) -> NodeId>,
}

impl<Message: 'static> Element<Message> {
    pub(crate) fn new(
        build: impl FnOnce(&dyn Realize<Message>, &mut Arena) -> NodeId + 'static,
    ) -> Self {
        Self {
            build: std::boxed::Box::new(build),
        }
    }

    pub(crate) fn build(self, realize: &dyn Realize<Message>, arena: &mut Arena) -> NodeId {
        (self.build)(realize, arena)
    }

    /// Applies `f` to the messages produced by this element's subtree.
    ///
    /// Works uniformly for the fixed message a widget sends immediately (e.g.
    /// `on_press`) and for value-carrying callbacks (`on_toggle`, `on_input`,
    /// ...), which only resolve to a message once the user interacts with the
    /// widget — both are composed, not rewritten after the fact.
    pub fn map<B: 'static>(self, f: impl Fn(Message) -> B + 'static) -> Element<B> {
        let f = std::rc::Rc::new(f);
        Element::new(move |realize: &dyn Realize<B>, arena: &mut Arena| {
            let adapter = MapRealize { inner: realize, f };
            self.build(&adapter, arena)
        })
    }
}

impl<Message: Send + 'static> Element<Message> {
    /// Builds this element into a standalone [`NodeTree`], rooted at itself.
    pub fn into_tree(self) -> NodeTree {
        let mut arena = Arena::new();
        let realize = RootRealize::<Message> {
            _marker: std::marker::PhantomData,
        };
        let root = self.build(&realize, &mut arena);
        NodeTree {
            nodes: arena.nodes.into_iter().collect(),
            root,
        }
    }
}

impl<Message> std::fmt::Debug for Element<Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Element").finish_non_exhaustive()
    }
}

struct MapRealize<'a, Message, B> {
    inner: &'a dyn Realize<B>,
    f: std::rc::Rc<dyn Fn(Message) -> B>,
}

impl<Message: 'static, B: 'static> Realize<Message> for MapRealize<'_, Message, B> {
    fn fixed(&self, g: std::boxed::Box<dyn Fn() -> Message>) -> Emit {
        let f = self.f.clone();
        self.inner.fixed(std::boxed::Box::new(move || f(g())))
    }

    fn bool_mapper(&self, g: std::boxed::Box<dyn Fn(bool) -> Message>) -> EmitBool {
        let f = self.f.clone();
        self.inner
            .bool_mapper(std::boxed::Box::new(move |v| f(g(v))))
    }

    fn f32_mapper(&self, g: std::boxed::Box<dyn Fn(f32) -> Message>) -> EmitF32 {
        let f = self.f.clone();
        self.inner
            .f32_mapper(std::boxed::Box::new(move |v| f(g(v))))
    }

    fn string_mapper(
        &self,
        g: std::boxed::Box<dyn Fn(std::string::String) -> Message>,
    ) -> EmitString {
        let f = self.f.clone();
        self.inner
            .string_mapper(std::boxed::Box::new(move |v| f(g(v))))
    }

    fn viewport_mapper(&self, g: std::boxed::Box<dyn Fn(Viewport) -> Message>) -> EmitViewport {
        let f = self.f.clone();
        self.inner
            .viewport_mapper(std::boxed::Box::new(move |v| f(g(v))))
    }
}

/// Builds any widget builder straight into a standalone [`NodeTree`], so a
/// plugin's `view` never has to name [`Element`] itself.
pub trait IntoTree<Message> {
    fn into_tree(self) -> NodeTree;
}

impl<Message: Send + 'static, T: Into<Element<Message>>> IntoTree<Message> for T {
    fn into_tree(self) -> NodeTree {
        self.into().into_tree()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widgets::{button, checkbox, column, slider, text, text_input};
    use stabby::closure::{Call0Dyn, Call1Dyn};

    #[test]
    fn nested_mapping_reaches_fixed_and_value_callbacks_without_clone_messages() {
        let fixed: Element<u32> = button(text("mapped")).on_press(|| 3).into();
        let boolean: Element<bool> = checkbox(false).on_toggle(|v| v).into();
        let float: Element<f32> = slider(0.0, 10.0, 0.0).on_change(|v| v).into();
        let string: Element<String> = text_input("", "").on_input(|v| v).into();
        let tree = column()
            .push(fixed.map(|v| v + 1).map(|v| format!("fixed:{v}")))
            .push(boolean.map(|v| format!("bool:{v}")))
            .push(float.map(|v| format!("float:{v}")))
            .push(string.map(|v| format!("str:{v}")))
            .into_tree();
        let mut values = Vec::new();
        for node in tree.nodes {
            let message = match node {
                Node::Button(node) => Option::<Emit>::from(node.on_press).unwrap().call(),
                Node::Checkbox(node) => {
                    Option::<EmitBool>::from(node.on_toggle).unwrap().call(true)
                }
                Node::Slider(node) => node.on_change.call(2.5),
                Node::TextInput(node) => Option::<EmitString>::from(node.on_input)
                    .unwrap()
                    .call("hello".into()),
                _ => continue,
            };
            // All root callbacks above produce String.
            #[allow(unsafe_code)]
            let value = unsafe { message.from_raw::<String>() };
            values.push(*value);
        }
        assert_eq!(values, ["fixed:4", "bool:true", "float:2.5", "str:hello"]);
    }

    #[test]
    fn widget_callbacks_can_capture_local_rc_state() {
        let state = std::rc::Rc::new(std::cell::Cell::new(1));
        let capture = state.clone();
        let _: Element<u32> = button(text("local")).on_press(move || capture.get()).into();
        assert_eq!(state.get(), 1);
    }
}
