use iced_core::{ContentFit, Length, Rotation, image::FilterMethod};


use crate::Element;
use crate::bindings::iced::app::element::image_to_element;
use crate::element::Widget;

type Handle = String;

/// A frame that displays an image.
#[derive(Debug)]
pub struct Image {
    handle: Handle,
    width: Option<Length>,
    height: Option<Length>,
    expand: Option<bool>,
    content_fit: Option<ContentFit>,
    filter_method: Option<FilterMethod>,
    rotation: Option<Rotation>,
    opacity: Option<f32>,
    scale: Option<f32>,
}

impl Image {
    /// Creates a new [`Image`] with the given path.
    pub fn new(handle: impl Into<Handle>) -> Self {
        Self {
            handle: handle.into(),
            width: None,
            height: None,
            expand: None,
            content_fit: None,
            filter_method: None,
            rotation: None,
            opacity: None,
            scale: None,
        }
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    pub fn content_fit(mut self, fit: impl Into<ContentFit>) -> Self {
        self.content_fit = Some(fit.into());
        self
    }

    pub fn filter_method(mut self, method: impl Into<FilterMethod>) -> Self {
        self.filter_method = Some(method.into());
        self
    }

    pub fn rotation(mut self, rotation: impl Into<Rotation>) -> Self {
        self.rotation = Some(rotation.into());
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = Some(scale);
        self
    }
}

impl<'a, Message> Widget<Message> for Image {
    fn as_element(
        self: Box<Self>,
        _: &dyn crate::element::CreateMessage<Message>,
    ) -> crate::bindings::Element {
        image_to_element(&crate::bindings::iced::app::element::Image {
            handle: self.handle,
            width: self.width.map(Into::into),
            height: self.height.map(Into::into),
            expand: self.expand,
            content_fit: self.content_fit.map(Into::into),
            filter_method: self.filter_method.map(Into::into),
            rotation: self.rotation.map(Into::into),
            opacity: self.opacity,
            scale: self.scale,
        })
    }
}

impl<'a, Message> From<Image> for Element<Message>
where
    Message: 'a,
{
    fn from(image: Image) -> Self {
        Element::new(Box::new(image))
    }
}
