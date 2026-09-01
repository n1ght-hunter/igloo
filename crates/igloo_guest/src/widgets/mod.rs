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

pub use checkbox::Checkbox;
pub use column::Column;
pub use combo_box::ComboBox;
pub use container::Container;
pub use float::Float;
pub use grid::Grid;
pub use image::Image;
pub use keyed::KeyedColumn;
pub use markdown::Markdown;
pub use pick_list::PickList;
pub use progress_bar::ProgressBar;
pub use radio::Radio;
pub use row::Row;
pub use rule::Rule;
pub use scrollable::Scrollable;
pub use slider::Slider;
pub use space::Space;
pub use svg::Svg;
pub use text::Text;
pub use text_input::TextInput;
pub use toggler::Toggler;
pub use tooltip::Tooltip;
pub use vertical_slider::VerticalSlider;

use std::ops::RangeInclusive;

use crate::Element;

/// Creates a [`Text`] widget with the given content.
pub fn text(text: impl Into<String>) -> Text {
    Text::new(text)
}

/// Creates an empty [`Column`].
pub fn column<Message>() -> Column<Message> {
    Column::new()
}

/// Creates a [`Button`] with the given content.
pub fn button<Message: 'static>(element: impl Into<Element<Message>>) -> button::Button<Message> {
    button::Button::new(element)
}

/// Creates an empty [`Row`].
pub fn row<Message>() -> Row<Message> {
    Row::new()
}

/// Creates a [`Container`] with the given content.
pub fn container<Message: 'static>(content: impl Into<Element<Message>>) -> Container<Message> {
    Container::new(content)
}

/// Displays floating content on top of the application.
pub fn float<Message: 'static>(content: impl Into<Element<Message>>) -> Float<Message> {
    Float::new(content)
}

/// Creates an empty [`Grid`].
pub fn grid<Message>() -> Grid<Message> {
    Grid::new()
}

/// Creates an empty [`KeyedColumn`].
pub fn keyed_column<Message>() -> KeyedColumn<Message> {
    KeyedColumn::new()
}

/// Creates a [`Scrollable`] wrapping the given content.
pub fn scrollable<Message: 'static>(content: impl Into<Element<Message>>) -> Scrollable<Message> {
    Scrollable::new(content)
}

/// Creates a [`Tooltip`] for the given content.
pub fn tooltip<Message>(
    content: impl Into<Element<Message>>,
    tooltip: impl Into<Element<Message>>,
    position: tooltip::Position,
) -> Tooltip<Message> {
    Tooltip::new(content, tooltip, position)
}

/// Creates a horizontal [`Rule`] with the given thickness.
pub fn horizontal_rule(thickness: impl Into<iced_core::Pixels>) -> Rule {
    Rule::horizontal(thickness)
}

/// Creates a vertical [`Rule`] with the given thickness.
pub fn vertical_rule(thickness: impl Into<iced_core::Pixels>) -> Rule {
    Rule::vertical(thickness)
}

/// Creates an amount of empty [`Space`].
pub fn space() -> Space {
    Space::new()
}

/// Creates a new [`Svg`] from the given path.
pub fn svg(path: impl Into<String>) -> Svg {
    Svg::new(path)
}

/// Creates a new [`Image`] with the given path.
pub fn image(handle: impl Into<String>) -> Image {
    Image::new(handle)
}

/// Creates a new [`ProgressBar`] with the given range and value.
pub fn progress_bar(range: RangeInclusive<f32>, value: f32) -> ProgressBar {
    ProgressBar::new(range, value)
}

/// Creates a new [`Radio`] button with the given label and value.
pub fn radio<Message: 'static, F, V>(
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

/// Creates a new [`Checkbox`] with the given checked state.
pub fn checkbox<Message: 'static>(is_checked: bool) -> Checkbox<Message> {
    Checkbox::new(is_checked)
}

/// Creates a new [`Toggler`] with the given state.
pub fn toggler<Message: 'static>(is_toggled: bool) -> Toggler<Message> {
    Toggler::new(is_toggled)
}

/// Creates a new [`TextInput`] with the given placeholder and value.
pub fn text_input<Message: 'static>(
    placeholder: impl Into<String>,
    value: impl Into<String>,
) -> TextInput<Message> {
    TextInput::new(placeholder, value)
}

/// Creates a new [`Slider`] for the given range and value.
pub fn slider<Message: 'static>(
    range: RangeInclusive<f32>,
    value: f32,
    on_change: impl Fn(f32) -> Message + 'static,
) -> Slider<Message> {
    Slider::new(range, value, on_change)
}

/// Creates a new [`VerticalSlider`] for the given range and value.
pub fn vertical_slider<Message: 'static>(
    range: RangeInclusive<f32>,
    value: f32,
    on_change: impl Fn(f32) -> Message + 'static,
) -> VerticalSlider<Message> {
    VerticalSlider::new(range, value, on_change)
}

/// Creates a new [`ComboBox`] with the given options.
pub fn combo_box<T, Message: 'static>(
    options: &combo_box::State<T>,
    placeholder: String,
    selection: Option<T>,
    on_selected: impl Fn(T) -> Message + 'static,
) -> ComboBox<T, Message>
where
    T: std::fmt::Display + Clone + 'static,
{
    ComboBox::new(options, placeholder, selection, on_selected)
}

/// Creates a new [`PickList`] with the given options.
pub fn pick_list<T, L, V, Message: 'static>(
    options: L,
    selected: Option<V>,
    on_select: impl Fn(T) -> Message + 'static,
) -> PickList<Message>
where
    T: ToString + PartialEq + Clone + 'static,
    L: std::borrow::Borrow<[T]>,
    V: std::borrow::Borrow<T>,
{
    PickList::new(options, selected, on_select)
}

/// Creates a [`Markdown`] widget from the given source, mapping clicked link
/// URLs to a message through `on_link_click`.
pub fn markdown<Message: 'static>(
    content: impl Into<String>,
    on_link_click: impl Fn(String) -> Message + 'static,
) -> Markdown<Message> {
    Markdown::new(content, on_link_click)
}
