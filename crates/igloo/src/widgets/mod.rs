
use crate::{
    bindings::iced::app::{
        self, button::Button, checkbox::Checkbox as CheckboxWidget, column::Column,
        combo_box::ComboBox as ComboBoxWidget, container::Container, float::Float as FloatWidget,
        grid::Grid as GridWidget, image::Image as ImageWidget,
        keyed::KeyedColumn as KeyedColumnWidget, markdown::Markdown as MarkdownWidget,
        pane_grid::PaneGrid as PaneGridWidget, pick_list::PickList as PickListWidget,
        progress_bar::ProgressBar as ProgressBarWidget, radio::Radio as RadioWidget, row::Row,
        rule::Rule as RuleWidget, scrollable::Scrollable, slider::Slider as SliderWidget,
        space::Space, svg::Svg as SvgWidget, table::Table as TableWidget, text::Text,
        text_input::TextInput as TextInputWidget, toggler::Toggler as TogglerWidget,
        tooltip::Tooltip, vertical_slider::VerticalSlider as VerticalSliderWidget,
    },
    plugin_manager::MyState,
};

#[derive(Debug, Clone)]
pub struct Message {
    pub id: crate::bindings::MessageId,
    pub content: crate::bindings::Message,
}

impl From<crate::bindings::MessageId> for Message {
    fn from(value: crate::bindings::MessageId) -> Self {
        Self {
            id: value,
            content: crate::bindings::Message::Empty,
        }
    }
}

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
pub mod vertical_slider;

impl app::text::Host for MyState {}
impl app::alignment::Host for MyState {}
impl app::length::Host for MyState {}
impl app::padding::Host for MyState {}
impl app::column::Host for MyState {}
impl app::row::Host for MyState {}
impl app::container::Host for MyState {}
impl app::tooltip::Host for MyState {}
impl app::shared::Host for MyState {}
impl app::button::Host for MyState {}
impl app::rule::Host for MyState {}
impl app::checkbox::Host for MyState {}
impl app::progress_bar::Host for MyState {}
impl app::toggler::Host for MyState {}
impl app::radio::Host for MyState {}
impl app::table::Host for MyState {}
impl app::text_input::Host for MyState {}
// impl app::text_editor::Host for MyState {}
impl app::pick_list::Host for MyState {}
impl app::combo_box::Host for MyState {}
impl app::float::Host for MyState {}
impl app::grid::Host for MyState {}
impl app::image::Host for MyState {}
impl app::keyed::Host for MyState {}
impl app::markdown::Host for MyState {}
impl app::pane_grid::Host for MyState {}
impl app::slider::Host for MyState {}
impl app::vertical_slider::Host for MyState {}
impl app::svg::Host for MyState {}
impl app::message_types::Host for MyState {}
impl app::space::Host for MyState {}
impl app::scrollable::Host for MyState {}
impl app::message::Host for MyState {
    fn clone_message(&mut self, message: crate::bindings::MessageId) -> crate::bindings::MessageId {
        message
    }
}
impl app::shared::HostElement for MyState {
    fn drop(&mut self, rep: wasmtime::component::Resource<Element>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum Element {
    Text(Text),
    Column(Column),
    Row(Row),
    Container(Container),
    Tooltip(Tooltip),
    Button(Button),
    Rule(RuleWidget),
    Checkbox(CheckboxWidget),
    ComboBox(ComboBoxWidget),
    Float(FloatWidget),
    Grid(GridWidget),
    Image(ImageWidget),
    KeyedColumn(KeyedColumnWidget),
    Markdown(MarkdownWidget),
    PaneGrid(PaneGridWidget),
    PickList(PickListWidget),
    ProgressBar(ProgressBarWidget),
    Toggler(TogglerWidget),
    Radio(RadioWidget),
    Table(TableWidget),
    // TextEditor(TextEditorWidget),
    TextInput(TextInputWidget),
    Slider(SliderWidget),
    VerticalSlider(VerticalSliderWidget),
    Svg(SvgWidget),
    Space(Space),
    Scrollable(Scrollable),
    Explain(Box<Element>, app::shared::Color),
}

impl app::element::Host for MyState {
    fn explain(
        &mut self,
        element: wasmtime::component::Resource<app::element::Element>,
        color: app::element::Color,
    ) -> wasmtime::component::Resource<app::element::Element> {
        let element = self.table.delete(element).unwrap();
        self.table.push(Element::Explain(Box::new(element), color)).unwrap()
    }

    fn text_to_element(&mut self, text: Text) -> wasmtime::component::Resource<Element> {
        self.table.push(Element::Text(text)).unwrap()
    }

    fn column_to_element(&mut self, column: Column) -> wasmtime::component::Resource<Element> {
        self.table.push(Element::Column(column)).unwrap()
    }

    fn row_to_element(&mut self, row: Row) -> wasmtime::component::Resource<Element> {
        self.table.push(Element::Row(row)).unwrap()
    }

    fn container_to_element(
        &mut self,
        container: Container,
    ) -> wasmtime::component::Resource<Element> {
        self.table.push(Element::Container(container)).unwrap()
    }

    fn tooltip_to_element(&mut self, tooltip: Tooltip) -> wasmtime::component::Resource<Element> {
        self.table.push(Element::Tooltip(tooltip)).unwrap()
    }

    fn button_to_element(&mut self, button: Button) -> wasmtime::component::Resource<Element> {
        self.table.push(Element::Button(button)).unwrap()
    }

    fn rule_to_element(&mut self, rule: RuleWidget) -> wasmtime::component::Resource<Element> {
        self.table.push(Element::Rule(rule)).unwrap()
    }

    fn checkbox_to_element(
        &mut self,
        checkbox: CheckboxWidget,
    ) -> wasmtime::component::Resource<Element> {
        self.table.push(Element::Checkbox(checkbox)).unwrap()
    }

    fn combo_box_to_element(
        &mut self,
        combo_box: ComboBoxWidget,
    ) -> wasmtime::component::Resource<Element> {
        self.table.push(Element::ComboBox(combo_box)).unwrap()
    }

    fn float_to_element(&mut self, float: FloatWidget) -> wasmtime::component::Resource<Element> {
        self.table.push(Element::Float(float)).unwrap()
    }

    fn grid_to_element(&mut self, grid: GridWidget) -> wasmtime::component::Resource<Element> {
        self.table.push(Element::Grid(grid)).unwrap()
    }

    fn image_to_element(&mut self, image: ImageWidget) -> wasmtime::component::Resource<Element> {
        self.table.push(Element::Image(image)).unwrap()
    }

    fn keyed_column_to_element(
        &mut self,
        column: KeyedColumnWidget,
    ) -> wasmtime::component::Resource<Element> {
        self.table.push(Element::KeyedColumn(column)).unwrap()
    }

    fn markdown_to_element(
        &mut self,
        markdown: MarkdownWidget,
    ) -> wasmtime::component::Resource<Element> {
        self.table.push(Element::Markdown(markdown)).unwrap()
    }

    fn pane_grid_to_element(
        &mut self,
        grid: PaneGridWidget,
    ) -> wasmtime::component::Resource<Element> {
        self.table.push(Element::PaneGrid(grid)).unwrap()
    }

    fn progress_bar_to_element(
        &mut self,
        progress_bar: ProgressBarWidget,
    ) -> wasmtime::component::Resource<Element> {
        self.table.push(Element::ProgressBar(progress_bar)).unwrap()
    }

    fn toggler_to_element(
        &mut self,
        toggler: TogglerWidget,
    ) -> wasmtime::component::Resource<Element> {
        self.table.push(Element::Toggler(toggler)).unwrap()
    }

    fn radio_to_element(&mut self, radio: RadioWidget) -> wasmtime::component::Resource<Element> {
        self.table.push(Element::Radio(radio)).unwrap()
    }

    fn pick_list_to_element(
        &mut self,
        pick_list: PickListWidget,
    ) -> wasmtime::component::Resource<Element> {
        self.table.push(Element::PickList(pick_list)).unwrap()
    }

    fn slider_to_element(
        &mut self,
        slider: SliderWidget,
    ) -> wasmtime::component::Resource<Element> {
        self.table.push(Element::Slider(slider)).unwrap()
    }

    fn vertical_slider_to_element(
        &mut self,
        slider: VerticalSliderWidget,
    ) -> wasmtime::component::Resource<Element> {
        self.table.push(Element::VerticalSlider(slider)).unwrap()
    }

    fn svg_to_element(&mut self, svg: SvgWidget) -> wasmtime::component::Resource<Element> {
        self.table.push(Element::Svg(svg)).unwrap()
    }

    fn table_to_element(&mut self, table: TableWidget) -> wasmtime::component::Resource<Element> {
        self.table.push(Element::Table(table)).unwrap()
    }

    // fn text_editor_to_element(
    //     &mut self,
    //     editor: TextEditorWidget,
    // ) -> wasmtime::component::Resource<Element> {
    //     self.table.push(Element::TextEditor(editor)).unwrap()
    // }

    fn text_input_to_element(
        &mut self,
        text_input: TextInputWidget,
    ) -> wasmtime::component::Resource<Element> {
        self.table.push(Element::TextInput(text_input)).unwrap()
    }

    fn space_to_element(
        &mut self,
        space: app::element::Space,
    ) -> wasmtime::component::Resource<app::element::Element> {
        self.table.push(Element::Space(space)).unwrap()
    }

    fn scrollable_to_element(
        &mut self,
        scrollable: app::element::Scrollable,
    ) -> wasmtime::component::Resource<app::element::Element> {
        self.table.push(Element::Scrollable(scrollable)).unwrap()
    }
}

pub trait WrapperTheme:
    iced::widget::text::Catalog
    + iced::widget::button::Catalog
    + iced::widget::container::Catalog
    + iced::widget::rule::Catalog
    + iced::widget::checkbox::Catalog
    + iced::widget::progress_bar::Catalog
    + iced::widget::toggler::Catalog
    + iced::widget::radio::Catalog
    + iced::widget::pick_list::Catalog
    + iced::widget::combo_box::Catalog
    + iced::widget::float::Catalog
    + iced::widget::pane_grid::Catalog
    + iced::widget::markdown::Catalog
    + iced::widget::table::Catalog
    // + iced::widget::text_editor::Catalog
    + iced::widget::text_input::Catalog
    + iced::widget::slider::Catalog
    + iced::widget::vertical_slider::Catalog
    + iced::widget::svg::Catalog
{
}
pub trait WrapperRenderer:
    iced::advanced::text::Renderer<Font = iced::Font>
    + iced::advanced::image::Renderer<Handle = iced::advanced::image::Handle>
    + iced::advanced::svg::Renderer
    + iced::advanced::Renderer
{
}

impl<T> WrapperTheme for T where
    T: iced::widget::text::Catalog
        + iced::widget::button::Catalog
        + iced::widget::container::Catalog
        + iced::widget::rule::Catalog
        + iced::widget::checkbox::Catalog
        + iced::widget::progress_bar::Catalog
        + iced::widget::toggler::Catalog
        + iced::widget::radio::Catalog
        + iced::widget::pick_list::Catalog
        + iced::widget::combo_box::Catalog
        + iced::widget::float::Catalog
        + iced::widget::pane_grid::Catalog
        + iced::widget::markdown::Catalog
        + iced::widget::table::Catalog
        // + iced::widget::text_editor::Catalog
        + iced::widget::text_input::Catalog
        + iced::widget::slider::Catalog
        + iced::widget::vertical_slider::Catalog
        + iced::widget::svg::Catalog
{
}

impl<T> WrapperRenderer for T where
    T: iced::advanced::text::Renderer<Font = iced::Font>
        + iced::advanced::image::Renderer<Handle = iced::advanced::image::Handle>
        + iced::advanced::svg::Renderer
        + iced::advanced::Renderer
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
            Element::Text(text) => text.into(),
            Element::Column(column) => column.to_element(resource_table),
            Element::Row(row) => row.to_element(resource_table),
            Element::Container(container) => container.to_element(resource_table),
            Element::Tooltip(tooltip) => tooltip.to_element(resource_table),
            Element::Button(button) => button.to_element(resource_table),
            Element::Rule(rule) => rule.to_element(resource_table),
            Element::Checkbox(checkbox) => checkbox.to_element(resource_table),
            Element::ProgressBar(progress_bar) => progress_bar.to_element(resource_table),
            Element::Toggler(toggler) => toggler.to_element(resource_table),
            Element::Radio(radio) => radio.to_element(resource_table),
            Element::ComboBox(combo_box) => combo_box.to_element(resource_table),
            Element::Float(float) => float.to_element(resource_table),
            Element::Grid(grid) => grid.to_element(resource_table),
            Element::Image(image) => image.to_element(resource_table),
            Element::KeyedColumn(column) => column.to_element(resource_table),
            Element::Markdown(markdown) => markdown.to_element(resource_table),
            Element::PaneGrid(pane_grid) => pane_grid.to_element(resource_table),
            Element::PickList(pick_list) => pick_list.to_element(resource_table),
            Element::Slider(slider) => slider.to_element(resource_table),
            Element::VerticalSlider(slider) => slider.to_element(resource_table),
            Element::Svg(svg) => svg.to_element(resource_table),
            Element::Table(table) => table.to_element(resource_table),
            // Element::TextEditor(editor) => editor.to_element(resource_table),
            Element::TextInput(text_input) => text_input.to_element(resource_table),
            Element::Explain(element, color) => element.to_element(resource_table).explain(color),
            Element::Space(space) => space.to_element(resource_table),
            Element::Scrollable(scrollable) => scrollable.to_element(resource_table),
        }
    }
}
