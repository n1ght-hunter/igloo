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

// Empty marker trait impls for interfaces with no host functions
impl app::shared::Host for MyState {}
impl app::button::Host for MyState {}
impl app::column::Host for MyState {}
impl app::row::Host for MyState {}
impl app::container::Host for MyState {}
impl app::tooltip::Host for MyState {}
impl app::text::Host for MyState {}
impl app::length::Host for MyState {}
impl app::padding::Host for MyState {}
impl app::alignment::Host for MyState {}
impl app::message_types::Host for MyState {}
impl app::callbacks::Host for MyState {}
impl app::rule::Host for MyState {}
impl app::space::Host for MyState {}
impl app::svg::Host for MyState {}
impl app::image::Host for MyState {}
impl app::progress_bar::Host for MyState {}
impl app::radio::Host for MyState {}
impl app::checkbox::Host for MyState {}
impl app::toggler::Host for MyState {}
impl app::text_input::Host for MyState {}
impl app::slider::Host for MyState {}
impl app::vertical_slider::Host for MyState {}
impl app::combo_box::Host for MyState {}
impl app::pick_list::Host for MyState {}
impl app::markdown::Host for MyState {}
impl app::float::Host for MyState {}
impl app::grid::Host for MyState {}
impl app::keyed::Host for MyState {}
impl app::scrollable::Host for MyState {}

impl app::shared::HostElement for MyState {
    fn drop(&mut self, rep: wasmtime::component::Resource<Element>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }

    fn noop(&mut self, _rep: wasmtime::component::Resource<Element>) {
        // Dummy function required for jco componentize to generate the Element class.
        // See: https://github.com/bytecodealliance/ComponentizeJS/issues/221
    }

    fn explain(
        &mut self,
        self_: wasmtime::component::Resource<Element>,
        color: app::shared::Color,
    ) -> wasmtime::component::Resource<Element> {
        let inner = self.table.delete(self_).unwrap();
        self.table
            .push(Element::Explain(Box::new(inner), color))
            .unwrap()
    }
}

/// The host-side element enum. Each variant holds a resource struct
/// that stores the builder state accumulated from guest widget calls.
#[derive(Debug)]
pub enum Element {
    Text(text::TextResource),
    Column(column::ColumnResource),
    Row(row::RowResource),
    Container(container::ContainerResource),
    Tooltip(tooltip::TooltipResource),
    Button(button::ButtonResource),
    Rule(rule::RuleResource),
    Space(space::SpaceResource),
    Svg(svg::SvgResource),
    Image(image::ImageResource),
    ProgressBar(progress_bar::ProgressBarResource),
    Radio(radio::RadioResource),
    Checkbox(checkbox::CheckboxResource),
    Toggler(toggler::TogglerResource),
    TextInput(text_input::TextInputResource),
    Slider(slider::SliderResource),
    VerticalSlider(vertical_slider::VerticalSliderResource),
    ComboBox(combo_box::ComboBoxResource),
    PickList(pick_list::PickListResource),
    Markdown(markdown::MarkdownResource),
    Float(float::FloatResource),
    Grid(grid::GridResource),
    Keyed(keyed::KeyedColumnResource),
    Scrollable(scrollable::ScrollableResource),
    Explain(Box<Element>, app::shared::Color),
}

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

pub trait ToElement {
    fn to_element<'a, Theme, Renderer>(
        self,
        resource_table: &mut wasmtime::component::ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a;
}

impl ToElement for Element {
    fn to_element<'a, Theme, Renderer>(
        self,
        resource_table: &mut wasmtime::component::ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        match self {
            Element::Text(txt) => txt.to_iced_element(),
            Element::Column(col) => col.to_iced_element(resource_table),
            Element::Row(row) => row.to_iced_element(resource_table),
            Element::Container(container) => container.to_iced_element(resource_table),
            Element::Tooltip(tooltip) => tooltip.to_iced_element(resource_table),
            Element::Button(btn) => btn.to_iced_element(resource_table),
            Element::Rule(rule) => rule.to_iced_element(),
            Element::Space(space) => space.to_iced_element(),
            Element::Svg(svg) => svg.to_iced_element(),
            Element::Image(image) => image.to_iced_element(),
            Element::ProgressBar(bar) => bar.to_iced_element(),
            Element::Radio(radio) => radio.to_iced_element(),
            Element::Checkbox(checkbox) => checkbox.to_iced_element(),
            Element::Toggler(toggler) => toggler.to_iced_element(),
            Element::TextInput(text_input) => text_input.to_iced_element(),
            Element::Slider(slider) => slider.to_iced_element(),
            Element::VerticalSlider(slider) => slider.to_iced_element(),
            Element::ComboBox(combo_box) => combo_box.to_iced_element(),
            Element::PickList(pick_list) => pick_list.to_iced_element(),
            Element::Markdown(markdown) => markdown.to_iced_element(),
            Element::Float(float) => float.to_iced_element(resource_table),
            Element::Grid(grid) => grid.to_iced_element(resource_table),
            Element::Keyed(keyed) => keyed.to_iced_element(resource_table),
            Element::Scrollable(scrollable) => scrollable.to_iced_element(resource_table),
            Element::Explain(element, color) => element.to_element(resource_table).explain(color),
        }
    }
}
