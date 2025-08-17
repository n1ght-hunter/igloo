#![allow(missing_debug_implementations)]

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
pub mod pane_grid;
pub mod pick_list;
pub mod progress_bar;
pub mod radio;
pub mod row;
pub mod rule;
pub mod slider;
pub mod svg;
pub mod table;
pub mod text;
// pub mod text_editor;
pub mod scrollable;
pub mod space;
pub mod text_input;
pub mod toggler;
pub mod tooltip;
mod utils;
pub mod vertical_slider;

use std::{borrow::Borrow, ops::RangeInclusive};

use crate::{
    Element,
    widgets::{scrollable::Scrollable, space::Space},
};
use iced_core::{Length, Pixels};

pub use checkbox::Checkbox;
pub use column::Column;
pub use combo_box::ComboBox;
pub use container::Container;
pub use float::Float;
pub use grid::Grid;
pub use image::Image;
pub use keyed::KeyedColumn;
pub use markdown::Markdown;
pub use pane_grid::PaneGrid;
pub use pick_list::PickList;
pub use progress_bar::ProgressBar;
pub use radio::Radio;
pub use row::Row;
pub use rule::Rule;
pub use slider::Slider;
pub use svg::Svg;
// pub use table::Table;
pub use text::Text;
// pub use text_editor::TextEditor;
pub use text_input::TextInput;
pub use toggler::Toggler;
pub use tooltip::Tooltip;
pub use vertical_slider::VerticalSlider;

/// Creates a [`Text`] widget with the given content.
pub fn text(text: impl Into<String>) -> Text {
    Text::new(text)
}

/// Creates an empty [`Column`].
pub fn column<Message>() -> Column<Message> {
    Column::new()
}

/// Creates an empty [`Row`].
pub fn row<Message>() -> Row<Message> {
    Row::new()
}

/// Creates a [`Container`] with the given content.
pub fn container<Message>(content: impl Into<Element<Message>>) -> Container<Message> {
    Container::new(content)
}

/// Creates a [`Tooltip`] for the given content.
pub fn tooltip<Message>(
    content: impl Into<Element<Message>>,
    tooltip: impl Into<Element<Message>>,
    position: tooltip::Position,
) -> Tooltip<Message> {
    Tooltip::new(content, tooltip, position)
}

/// Creates a [`Button`] with the given content.
pub fn button<Message: Clone>(element: impl Into<Element<Message>>) -> button::Button<Message> {
    button::Button::new(element)
}

/// Creates a [`Checkbox`] with the given label and state.
pub fn checkbox<Message>(label: impl Into<String>, is_checked: bool) -> Checkbox<Message> {
    Checkbox::new(label, is_checked)
}

/// Creates a horizontal [`Rule`] with the given thickness.
pub fn horizontal_rule(thickness: impl Into<Pixels>) -> Rule {
    Rule::horizontal(thickness)
}

/// Creates a vertical [`Rule`] with the given thickness.
pub fn vertical_rule(thickness: impl Into<Pixels>) -> Rule {
    Rule::vertical(thickness)
}

/// Creates a [`ProgressBar`] with the given range and value.
pub fn progress_bar(range: RangeInclusive<f32>, value: f32) -> ProgressBar {
    ProgressBar::new(range, value)
}

/// Creates a [`Toggler`] with the given state.
pub fn toggler<Message>(is_toggled: bool) -> Toggler<Message> {
    Toggler::new(is_toggled)
}

/// Creates a [`Radio`] with the given label and value.
pub fn radio<Message, V, F>(
    label: impl Into<String>,
    value: V,
    selected: Option<V>,
    f: F,
) -> Radio<Message>
where
    V: Eq + Copy,
    F: FnOnce(V) -> Message,
{
    Radio::new(label, value, selected, f)
}

/// Creates a [`ComboBox`] with the given options.
pub fn combo_box<T, Message>(
    options: &combo_box::State<T>,
    placeholder: String,
    selection: Option<T>,
    on_selected: impl Fn(T) -> Message + Send + Sync + 'static,
) -> ComboBox<T, Message>
where
    T: std::fmt::Display + Clone,
{
    ComboBox::new(options, placeholder, selection, on_selected)
}

/// Creates a [`Float`] with the given content.
pub fn float<Message>(content: impl Into<Element<Message>>) -> Float<Message> {
    Float::new(content)
}

/// Creates an empty [`Grid`].
pub fn grid<Message>() -> Grid<Message> {
    Grid::new()
}

/// Creates an [`Image`] from the given path.
pub fn image(path: impl Into<String>) -> Image {
    Image::new(path)
}

/// Creates an empty [`KeyedColumn`].
pub fn keyed_column<Message>() -> KeyedColumn<Message> {
    KeyedColumn::new()
}

/// Creates a [`Markdown`] widget from the given source.
pub fn markdown(content: impl Into<String>) -> Markdown {
    Markdown::new(content)
}

/// Creates an [`Overlay`] with the given elements.
/// Creates a [`PaneGrid`] with the given children.
pub fn pane_grid<Message>(
    children: impl IntoIterator<Item = Element<Message>>,
) -> PaneGrid<Message> {
    PaneGrid::new(children)
}

/// Creates a [`PickList`] with the given options.
pub fn pick_list<T, L, V, Message>(
    options: L,
    selected: Option<V>,
    on_select: impl Fn(T) -> Message + Send + Sync + 'static,
) -> PickList<T, L, V, Message>
where
    T: ToString + PartialEq + Send + Sync + Clone,
    L: Borrow<[T]> + 'static,
    V: Borrow<T> + 'static,
    Message: Clone,
{
    PickList::new(options, selected, on_select)
}

/// Creates a [`Slider`] with the given range and value.
pub fn slider<Message>(
    range: RangeInclusive<f32>,
    value: f32,
    on_change: impl Fn(f32) -> Message + Send + Sync + 'static,
) -> Slider<Message> {
    Slider::new(range, value, on_change)
}

/// Creates a [`VerticalSlider`] with the given range and value.
pub fn vertical_slider<Message>(
    range: RangeInclusive<f32>,
    value: f32,
    on_change: impl Fn(f32) -> Message + Send + Sync + 'static,
) -> VerticalSlider<Message> {
    VerticalSlider::new(range, value, on_change)
}

// /// Creates a [`Table`] with the given column headers.
// pub fn table(headers: impl IntoIterator<Item = Element>) -> Table {
//     Table::new(headers)
// }

/// Creates an [`Svg`] from the given path.
pub fn svg(path: impl Into<String>) -> Svg {
    Svg::new(path)
}

// /// Creates a [`TextEditor`] with the given content.
// pub fn text_editor(value: impl Into<String>) -> TextEditor {
//     TextEditor::new(value)
// }

/// Creates a [`TextInput`] with the given placeholder and value.
pub fn text_input<Message>(
    placeholder: impl Into<String>,
    value: impl Into<String>,
) -> TextInput<Message> {
    TextInput::new(placeholder, value)
}

/// Creates a new [`Space`] widget that fills the available
/// horizontal space.
///
/// This can be useful to separate widgets in a [`Row`].
pub fn horizontal_space() -> Space {
    Space::with_width(Length::Fill)
}

/// Creates a new [`Space`] widget that fills the available
/// vertical space.
///
/// This can be useful to separate widgets in a [`Column`].
pub fn vertical_space() -> Space {
    Space::with_height(Length::Fill)
}

pub fn scrollable<Message>(content: impl Into<Element<Message>>) -> Scrollable<Message> {
    Scrollable::new(content)
}
