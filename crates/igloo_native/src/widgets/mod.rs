//! Widget node types, their host-side renderer, and an ergonomic builder API
//! for constructing a [`NodeTree`] without hand-indexing nodes.

#![allow(long_running_const_eval)]

pub mod button;
pub mod checkbox;
pub mod column;
pub mod combo_box;
pub mod container;
mod element;
pub mod explain;
pub mod float;
pub mod grid;
pub mod image;
pub mod keyed;
pub mod pick_list;
pub mod progress_bar;
pub mod radio;
pub mod row;
pub mod rule;
pub mod scrollable;
pub mod slider;
pub mod space;
pub mod svg;
pub mod text;
pub mod text_input;
pub mod toggler;
pub mod tooltip;
mod types;
pub mod vertical_slider;

pub use button::{Button, button};
pub use checkbox::{Checkbox, checkbox};
pub use column::{Column, column};
pub use combo_box::{ComboBox, combo_box};
pub use container::{Container, container};
pub use element::{Element, IntoTree};
pub use float::{Float, float};
pub use grid::{Grid, grid};
pub use image::{Image, image};
pub use keyed::{KeyedColumn, keyed_column};
pub use pick_list::{PickList, pick_list};
pub use progress_bar::{ProgressBar, progress_bar};
pub use radio::{Radio, radio};
pub use row::{Row, row};
pub use rule::{Rule, horizontal_rule, vertical_rule};
pub use scrollable::{Scrollable, scrollable};
pub use slider::{Slider, slider};
pub use space::{Space, space};
pub use svg::{Svg, svg};
pub use text::{Text, text};
pub use text_input::{TextInput, text_input};
pub use toggler::{Toggler, toggler};
pub use tooltip::{Tooltip, tooltip};
pub use types::{
    Alignment, Anchor, Color, ContentFit, Direction, FilterMethod, Horizontal, Length, LineHeight,
    Padding, Position, Rotation, Scrollbar, Shaping, TextAlignment, Translation, Vertical,
    Viewport,
};
pub use vertical_slider::{VerticalSlider, vertical_slider};

use button::ButtonNode;
use checkbox::CheckboxNode;
use column::ColumnNode;
use combo_box::ComboBoxNode;
use container::ContainerNode;
use explain::ExplainNode;
use float::FloatNode;
use grid::GridNode;
use image::ImageNode;
use keyed::KeyedColumnNode;
use pick_list::PickListNode;
use progress_bar::ProgressBarNode;
use radio::RadioNode;
use row::RowNode;
use rule::RuleNode;
use scrollable::ScrollableNode;
use slider::SliderNode;
use space::SpaceNode;
use svg::SvgNode;
use text::TextNode;
use text_input::TextInputNode;
use toggler::TogglerNode;
use tooltip::TooltipNode;
use vertical_slider::VerticalSliderNode;

/// Index into a [`NodeTree`]'s `nodes`.
pub type NodeId = u32;

/// A flattened UI tree: all nodes live in `nodes`, `root` indexes the entry point.
#[stabby::stabby]
#[derive(Debug)]
pub struct NodeTree {
    pub nodes: stabby::vec::Vec<Node>,
    pub root: NodeId,
}

/// A single renderable node in a [`NodeTree`].
#[stabby::stabby]
#[repr(C, u8)]
#[derive(Debug)]
pub enum Node {
    Text(TextNode),
    Column(ColumnNode),
    Button(ButtonNode),
    Row(RowNode),
    Container(ContainerNode),
    Space(SpaceNode),
    Rule(RuleNode),
    Svg(SvgNode),
    Image(ImageNode),
    ProgressBar(ProgressBarNode),
    Tooltip(TooltipNode),
    Float(FloatNode),
    Grid(GridNode),
    KeyedColumn(KeyedColumnNode),
    Explain(ExplainNode),
    Checkbox(CheckboxNode),
    Toggler(TogglerNode),
    Radio(RadioNode),
    TextInput(TextInputNode),
    Slider(SliderNode),
    VerticalSlider(VerticalSliderNode),
    Scrollable(ScrollableNode),
    PickList(PickListNode),
    ComboBox(ComboBoxNode),
}

/// The message type the renderer's `iced::Element` tree carries internally.
///
/// Several iced widgets require `Message: Clone` for a reason that has
/// nothing to do with cloning the closure that produces a message — see
/// `iced_widget::button::OnPress::get`, which clones the *message*, not the
/// closure. `Emitted` satisfies that bound by re-calling the stored [`Emit`]
/// on every clone; the [`Message`] it carries is never itself duplicated.
#[cfg(feature = "host")]
pub(crate) enum Emitted {
    Fixed(crate::messages::Message, std::rc::Rc<crate::Emit>),
    Bool(
        crate::messages::Message,
        std::rc::Rc<crate::messages::EmitBool>,
        bool,
    ),
    F32(
        crate::messages::Message,
        std::rc::Rc<crate::messages::EmitF32>,
        f32,
    ),
    Str(
        crate::messages::Message,
        std::rc::Rc<crate::messages::EmitString>,
        std::string::String,
    ),
    Viewport(
        crate::messages::Message,
        std::rc::Rc<crate::messages::EmitViewport>,
        Viewport,
    ),
}

#[cfg(feature = "host")]
impl Emitted {
    pub(crate) fn fixed(emit: &std::rc::Rc<crate::Emit>) -> Self {
        use stabby::closure::Call0Dyn;
        Emitted::Fixed(emit.call(), emit.clone())
    }

    pub(crate) fn bool(emit: &std::rc::Rc<crate::messages::EmitBool>, value: bool) -> Self {
        use stabby::closure::Call1Dyn;
        Emitted::Bool(emit.call(value), emit.clone(), value)
    }

    pub(crate) fn f32(emit: &std::rc::Rc<crate::messages::EmitF32>, value: f32) -> Self {
        use stabby::closure::Call1Dyn;
        Emitted::F32(emit.call(value), emit.clone(), value)
    }

    pub(crate) fn str(
        emit: &std::rc::Rc<crate::messages::EmitString>,
        value: std::string::String,
    ) -> Self {
        use stabby::closure::Call1Dyn;
        let message = emit.call(value.clone().into());
        Emitted::Str(message, emit.clone(), value)
    }

    pub(crate) fn viewport(
        emit: &std::rc::Rc<crate::messages::EmitViewport>,
        value: Viewport,
    ) -> Self {
        use stabby::closure::Call1Dyn;
        Emitted::Viewport(emit.call(value), emit.clone(), value)
    }

    fn into_message(self) -> crate::messages::Message {
        match self {
            Emitted::Fixed(message, _) => message,
            Emitted::Bool(message, ..) => message,
            Emitted::F32(message, ..) => message,
            Emitted::Str(message, ..) => message,
            Emitted::Viewport(message, ..) => message,
        }
    }
}

#[cfg(feature = "host")]
impl Clone for Emitted {
    fn clone(&self) -> Self {
        match self {
            Emitted::Fixed(_, emit) => Emitted::fixed(emit),
            Emitted::Bool(_, emit, value) => Emitted::bool(emit, *value),
            Emitted::F32(_, emit, value) => Emitted::f32(emit, *value),
            Emitted::Str(_, emit, value) => Emitted::str(emit, value.clone()),
            Emitted::Viewport(_, emit, value) => Emitted::viewport(emit, *value),
        }
    }
}

/// Owns the renderer state borrowed by combo boxes in a plugin view.
#[cfg(feature = "host")]
#[derive(Debug, Default)]
pub struct PreparedView {
    states: std::collections::HashMap<NodeId, iced::widget::combo_box::State<String>>,
}
#[cfg(feature = "host")]
impl PreparedView {
    pub fn new(tree: &NodeTree) -> Self {
        let mut prepared = Self::default();
        prepared.refresh(tree);
        prepared
    }
    /// Keeps unchanged combo box state and releases removed or changed options.
    pub fn refresh(&mut self, tree: &NodeTree) {
        let mut previous = std::mem::take(&mut self.states);
        self.states = tree
            .nodes
            .iter()
            .enumerate()
            .filter_map(|(id, node)| match node {
                Node::ComboBox(node) => {
                    let id = id as NodeId;
                    let options: Vec<String> =
                        node.options.iter().map(|v| v.as_str().to_owned()).collect();
                    let state = previous
                        .remove(&id)
                        .filter(|state| state.options() == options)
                        .unwrap_or_else(|| iced::widget::combo_box::State::new(options));
                    Some((id, state))
                }
                _ => None,
            })
            .collect();
    }
    /// Renders an ABI tree using this view's owned state.
    pub fn render<'a>(&'a self, tree: NodeTree) -> iced::Element<'a, crate::messages::Message> {
        let root = tree.root;
        let mut nodes = RenderArena {
            nodes: tree.nodes.into_iter().map(Some).collect(),
            states: &self.states,
        };
        build_node(&mut nodes, root).map(Emitted::into_message)
    }
}

#[cfg(all(test, feature = "host"))]
mod prepared_view_tests {
    use super::*;
    #[test]
    fn repeated_updates_reuse_state_and_release_removed_combo_boxes() {
        let tree = combo_box(vec!["first".to_owned()], "select", None::<String>)
            .on_selected(|value| value)
            .into_tree();
        let mut prepared = PreparedView::new(&tree);
        let allocation = prepared.states[&tree.root].options().as_ptr();
        for _ in 0..100 {
            prepared.refresh(&tree);
        }
        assert_eq!(prepared.states.len(), 1);
        assert_eq!(allocation, prepared.states[&tree.root].options().as_ptr());
        let empty: Element<String> = text("no combo").into();
        prepared.refresh(&empty.into_tree());
        assert!(prepared.states.is_empty());
    }
}
#[cfg(feature = "host")]
pub(crate) struct RenderArena<'a> {
    nodes: Vec<Option<Node>>,
    states: &'a std::collections::HashMap<NodeId, iced::widget::combo_box::State<String>>,
}

/// Builds the node at `id`, consuming it out of the arena.
///
/// An id outside `nodes` degrades to an empty space rather than panicking,
/// so a malformed tree fails softly.
#[cfg(feature = "host")]
pub(crate) fn build_node<'a>(
    nodes: &mut RenderArena<'a>,
    id: NodeId,
) -> iced::Element<'a, Emitted> {
    let Some(node) = nodes.nodes.get_mut(id as usize).and_then(Option::take) else {
        return iced::widget::Space::new().into();
    };
    match node {
        Node::Text(text) => text::build(text),
        Node::Column(column) => column::build(nodes, column),
        Node::Button(button) => button::build(nodes, button),
        Node::Row(row) => row::build(nodes, row),
        Node::Container(container) => container::build(nodes, container),
        Node::Space(space) => space::build(space),
        Node::Rule(rule) => rule::build(rule),
        Node::Svg(svg) => svg::build(svg),
        Node::Image(image) => image::build(image),
        Node::ProgressBar(bar) => progress_bar::build(bar),
        Node::Tooltip(tooltip) => tooltip::build(nodes, tooltip),
        Node::Float(float) => float::build(nodes, float),
        Node::Grid(grid) => grid::build(nodes, grid),
        Node::KeyedColumn(column) => keyed::build(nodes, column),
        Node::Explain(explain) => explain::build(nodes, explain),
        Node::Checkbox(checkbox) => checkbox::build(checkbox),
        Node::Toggler(toggler) => toggler::build(toggler),
        Node::Radio(radio) => radio::build(radio),
        Node::TextInput(input) => text_input::build(input),
        Node::Slider(slider) => slider::build(slider),
        Node::VerticalSlider(slider) => vertical_slider::build(slider),
        Node::Scrollable(scrollable) => scrollable::build(nodes, scrollable),
        Node::PickList(pick_list) => pick_list::build(pick_list),
        Node::ComboBox(combo_box) => combo_box::build(
            combo_box,
            nodes
                .states
                .get(&id)
                .expect("combo box missing from prepared view"),
        ),
    }
}
