use crate::{
    bindings::iced::app::widgets::ImageNode,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};

/// Builds an `iced::Image` from its node.
pub fn build<'a, Theme, Renderer>(node: ImageNode) -> iced::Element<'a, Message, Theme, Renderer>
where
    Theme: WrapperTheme + 'a,
    Renderer: WrapperRenderer + 'a,
{
    let handle = iced::advanced::image::Handle::from_path(node.handle);
    let mut image = iced::widget::Image::new(handle);

    if let Some(w) = node.width {
        image = image.width(w);
    }
    if let Some(h) = node.height {
        image = image.height(h);
    }
    if let Some(expand) = node.expand {
        image = image.expand(expand);
    }
    if let Some(fit) = node.content_fit {
        image = image.content_fit(fit.into());
    }
    if let Some(method) = node.filter_method {
        image = image.filter_method(method.into());
    }
    if let Some(rotation) = node.rotation {
        image = image.rotation(iced::Rotation::from(rotation));
    }
    if let Some(opacity) = node.opacity {
        image = image.opacity(opacity);
    }
    if let Some(scale) = node.scale {
        image = image.scale(scale);
    }
    image.into()
}
