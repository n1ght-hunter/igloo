use crate::{bindings::iced::app, plugin_manager::MyState};

pub mod button;
pub mod checkbox;
pub mod column;
pub mod combo_box;
pub mod container;
pub mod float;
pub mod grid;
pub mod image;
pub mod keyed;
pub mod markdown;
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
pub mod vertical_slider;

use app::widgets::Node;

/// Message wraps either an opaque callback id the guest minted ahead of time
/// (fixed callbacks like `on_press`), or a mapper callback id paired with the
/// raw value produced by the interaction (value-carrying callbacks like
/// `on_toggle`). Both id kinds are resolved back into a real
/// `Application::Message` by the guest itself in
/// [`crate::plugin_manager::PluginManager::plugin_update`].
#[derive(Debug, Clone)]
pub enum Message {
    Fixed {
        rep: u32,
    },
    Bool {
        mapper: u32,
        value: bool,
    },
    F32 {
        mapper: u32,
        value: f32,
    },
    F64 {
        mapper: u32,
        value: f64,
    },
    U64 {
        mapper: u32,
        value: u64,
    },
    String {
        mapper: u32,
        value: String,
    },
    Viewport {
        mapper: u32,
        value: app::message_types::Viewport,
    },
}

// Empty marker trait impls for the imported type-only interfaces.
impl app::shared::Host for MyState {}
impl app::callbacks::Host for MyState {}
impl app::length::Host for MyState {}
impl app::padding::Host for MyState {}
impl app::alignment::Host for MyState {}
impl app::message_types::Host for MyState {}
impl app::widgets::Host for MyState {}

pub trait WrapperTheme:
    iced::widget::text::Catalog
    + iced::widget::button::Catalog
    + iced::widget::progress_bar::Catalog
    + iced::widget::rule::Catalog
    + iced::widget::svg::Catalog
    + iced::widget::radio::Catalog
    + iced::widget::checkbox::Catalog
    + iced::widget::toggler::Catalog
    + iced::widget::text_input::Catalog
    + iced::widget::slider::Catalog
    + iced::widget::combo_box::Catalog
    + iced::widget::pick_list::Catalog
    + iced::widget::markdown::Catalog
    + iced::widget::float::Catalog
    + iced::widget::scrollable::Catalog
{
}

pub trait WrapperRenderer:
    iced::advanced::text::Renderer<Font = iced::Font>
    + iced::advanced::Renderer
    + iced::advanced::image::Renderer<Handle = iced::advanced::image::Handle>
    + iced::advanced::svg::Renderer
{
}

impl<T> WrapperTheme for T where
    T: iced::widget::text::Catalog
        + iced::widget::button::Catalog
        + iced::widget::progress_bar::Catalog
        + iced::widget::rule::Catalog
        + iced::widget::svg::Catalog
        + iced::widget::radio::Catalog
        + iced::widget::checkbox::Catalog
        + iced::widget::toggler::Catalog
        + iced::widget::text_input::Catalog
        + iced::widget::slider::Catalog
        + iced::widget::combo_box::Catalog
        + iced::widget::pick_list::Catalog
        + iced::widget::markdown::Catalog
        + iced::widget::float::Catalog
        + iced::widget::scrollable::Catalog
{
}

impl<T> WrapperRenderer for T where
    T: iced::advanced::text::Renderer<Font = iced::Font>
        + iced::advanced::Renderer
        + iced::advanced::image::Renderer<Handle = iced::advanced::image::Handle>
        + iced::advanced::svg::Renderer
{
}

/// Recursively builds an `iced::Element` from the node at `id` in `tree`.
///
/// Each parent resolves its children first, then hands the built elements to the
/// matching widget builder. Child ids that fall outside `tree.nodes` are treated
/// as an empty space so a malformed tree degrades instead of panicking.
pub fn build_element<'a, Theme, Renderer>(
    tree: &mut Vec<Option<Node>>,
    id: u32,
) -> iced::Element<'a, Message, Theme, Renderer>
where
    Theme: WrapperTheme + 'a,
    Renderer: WrapperRenderer + 'a,
{
    let Some(node) = tree.get_mut(id as usize).and_then(Option::take) else {
        return iced::widget::Space::new().into();
    };

    let build_child_elements = |children: Vec<u32>,
                                tree: &mut Vec<Option<Node>>|
     -> Vec<iced::Element<'a, Message, Theme, Renderer>> {
        children
            .into_iter()
            .map(|c| build_element(tree, c))
            .collect()
    };

    match node {
        Node::Text(n) => text::build(n),
        Node::Column(mut n) => {
            let children = std::mem::take(&mut n.children);
            column::build(n, build_child_elements(children, tree))
        }
        Node::Row(mut n) => {
            let children = std::mem::take(&mut n.children);
            row::build(n, build_child_elements(children, tree))
        }
        Node::Container(n) => container::build(n, build_element(tree, n.content)),
        Node::Tooltip(n) => tooltip::build(
            n,
            build_element(tree, n.content),
            build_element(tree, n.tooltip),
        ),
        Node::Button(n) => button::build(n, build_element(tree, n.content)),
        Node::Rule(n) => rule::build(n),
        Node::Space(n) => space::build(n),
        Node::Svg(n) => svg::build(n),
        Node::Image(n) => image::build(n),
        Node::ProgressBar(n) => progress_bar::build(n),
        Node::Radio(n) => radio::build(n),
        Node::Checkbox(n) => checkbox::build(n),
        Node::Toggler(n) => toggler::build(n),
        Node::TextInput(n) => text_input::build(n),
        Node::Slider(n) => slider::build(n),
        Node::VerticalSlider(n) => vertical_slider::build(n),
        Node::ComboBox(n) => combo_box::build(n),
        Node::PickList(n) => pick_list::build(n),
        Node::Markdown(n) => markdown::build(n),
        Node::Float(n) => float::build(n, build_element(tree, n.content)),
        Node::Grid(mut n) => {
            let children = std::mem::take(&mut n.elements);
            grid::build(n, build_child_elements(children, tree))
        }
        Node::KeyedColumn(mut n) => {
            let children = std::mem::take(&mut n.children);
            keyed::build(n, build_child_elements(children, tree))
        }
        Node::Scrollable(n) => scrollable::build(n, build_element(tree, n.content)),
        Node::Explain(n) => build_element(tree, n.content).explain(iced::Color::from(n.color)),
    }
}
