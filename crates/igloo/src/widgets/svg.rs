use crate::{
    bindings::iced::app::widgets::SvgNode,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};

/// Builds an `iced::Svg` from its node.
pub fn build<'a, Theme, Renderer>(node: SvgNode) -> iced::Element<'a, Message, Theme, Renderer>
where
    Theme: WrapperTheme + 'a,
    Renderer: WrapperRenderer + 'a,
{
    let handle = iced::advanced::svg::Handle::from_path(node.path);
    iced::widget::Svg::new(handle).into()
}
