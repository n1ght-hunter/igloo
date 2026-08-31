use crate::{
    bindings::iced::app::{
        image,
        length::Length,
        shared::{ContentFit, Element, FilterMethod, Rotation},
    },
    plugin_manager::MyState,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};
use wasmtime::component::Resource;

#[derive(Debug)]
pub struct ImageResource {
    pub handle: String,
    pub width: Option<Length>,
    pub height: Option<Length>,
    pub expand: Option<bool>,
    pub content_fit: Option<ContentFit>,
    pub filter_method: Option<FilterMethod>,
    pub rotation: Option<Rotation>,
    pub opacity: Option<f32>,
    pub scale: Option<f32>,
}

impl ImageResource {
    pub fn to_iced_element<'a, Theme, Renderer>(self) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let handle = iced::advanced::image::Handle::from_path(self.handle);
        let mut image = iced::widget::Image::new(handle);

        if let Some(w) = self.width {
            image = image.width(w);
        }
        if let Some(h) = self.height {
            image = image.height(h);
        }
        if let Some(expand) = self.expand {
            image = image.expand(expand);
        }
        if let Some(fit) = self.content_fit {
            image = image.content_fit(fit.into());
        }
        if let Some(method) = self.filter_method {
            image = image.filter_method(method.into());
        }
        if let Some(rotation) = self.rotation {
            image = image.rotation(iced::Rotation::from(rotation));
        }
        if let Some(opacity) = self.opacity {
            image = image.opacity(opacity);
        }
        if let Some(scale) = self.scale {
            image = image.scale(scale);
        }
        image.into()
    }
}

impl image::HostImage for MyState {
    fn new(&mut self, handle: String) -> Resource<image::Image> {
        self.table
            .push(ImageResource {
                handle,
                width: None,
                height: None,
                expand: None,
                content_fit: None,
                filter_method: None,
                rotation: None,
                opacity: None,
                scale: None,
            })
            .unwrap()
    }

    fn width(&mut self, self_: Resource<image::Image>, w: Length) {
        self.table.get_mut(&self_).unwrap().width = Some(w);
    }

    fn height(&mut self, self_: Resource<image::Image>, h: Length) {
        self.table.get_mut(&self_).unwrap().height = Some(h);
    }

    fn expand(&mut self, self_: Resource<image::Image>, expand: bool) {
        self.table.get_mut(&self_).unwrap().expand = Some(expand);
    }

    fn content_fit(&mut self, self_: Resource<image::Image>, fit: ContentFit) {
        self.table.get_mut(&self_).unwrap().content_fit = Some(fit);
    }

    fn filter_method(&mut self, self_: Resource<image::Image>, method: FilterMethod) {
        self.table.get_mut(&self_).unwrap().filter_method = Some(method);
    }

    fn rotation(&mut self, self_: Resource<image::Image>, r: Rotation) {
        self.table.get_mut(&self_).unwrap().rotation = Some(r);
    }

    fn opacity(&mut self, self_: Resource<image::Image>, o: f32) {
        self.table.get_mut(&self_).unwrap().opacity = Some(o);
    }

    fn scale(&mut self, self_: Resource<image::Image>, s: f32) {
        self.table.get_mut(&self_).unwrap().scale = Some(s);
    }

    fn into_element(&mut self, self_: Resource<image::Image>) -> Resource<Element> {
        let image = self.table.delete(self_).unwrap();
        self.table.push(super::Element::Image(image)).unwrap()
    }

    fn drop(&mut self, rep: Resource<image::Image>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
