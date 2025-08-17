use super::Message;
use crate::{
    bindings::iced::app::image::Image,
    widgets::{ToElement, WrapperRenderer, WrapperTheme},
};

impl ToElement for Image {
    fn to_element<'a, Theme, Renderer>(
        self,
        _resource_table: &mut wasmtime::component::ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let Image {
            handle,
            width,
            height,
            expand,
            content_fit,
            filter_method,
            rotation,
            opacity,
            scale,
        } = self;

        let handle = iced::advanced::image::Handle::from_path(handle);

        let mut image = iced::widget::Image::new(handle);

        if let Some(width) = width {
            image = image.width(width);
        }

        if let Some(height) = height {
            image = image.height(height);
        }

        if let Some(expand) = expand {
            image = image.expand(expand);
        }

        if let Some(content_fit) = content_fit {
            image = image.content_fit(content_fit.into());
        }

        if let Some(filter_method) = filter_method {
            image = image.filter_method(filter_method.into());
        }

        if let Some(rotation) = rotation {
            image = image.rotation(rotation);
        }

        if let Some(opacity) = opacity {
            image = image.opacity(opacity);
        }

        if let Some(scale) = scale {
            image = image.scale(scale);
        }

        image.into()
    }
}
