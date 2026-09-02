use iced_core::{ContentFit, Length, Rotation, image::FilterMethod};

use crate::Element;
use crate::bindings::iced::app::widgets::{ImageNode, Node};

/// A frame that displays an image.
pub struct Image {
    handle: String,
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
    pub fn new(handle: impl Into<String>) -> Self {
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

impl<Message: 'static> From<Image> for Element<Message> {
    fn from(image: Image) -> Self {
        Element::new(move |_realize, arena| {
            let node = ImageNode {
                handle: image.handle,
                width: image.width.map(Into::into),
                height: image.height.map(Into::into),
                expand: image.expand,
                content_fit: image.content_fit.map(Into::into),
                filter_method: image.filter_method.map(Into::into),
                rotation: image.rotation.map(Into::into),
                opacity: image.opacity,
                scale: image.scale,
            };
            arena.push(Node::Image(node))
        })
    }
}
