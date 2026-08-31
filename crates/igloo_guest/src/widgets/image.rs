use iced_core::{ContentFit, Length, Rotation, image::FilterMethod};

use crate::Element;
use crate::bindings::iced::app::image::Image as WitImage;

/// A frame that displays an image.
pub struct Image {
    raw: WitImage,
}

impl Image {
    /// Creates a new [`Image`] with the given path.
    pub fn new(handle: impl Into<String>) -> Self {
        Self {
            raw: WitImage::new(&handle.into()),
        }
    }

    pub fn width(self, width: impl Into<Length>) -> Self {
        self.raw.width(width.into().into());
        self
    }

    pub fn height(self, height: impl Into<Length>) -> Self {
        self.raw.height(height.into().into());
        self
    }

    pub fn expand(self, expand: bool) -> Self {
        self.raw.expand(expand);
        self
    }

    pub fn content_fit(self, fit: impl Into<ContentFit>) -> Self {
        self.raw.content_fit(fit.into().into());
        self
    }

    pub fn filter_method(self, method: impl Into<FilterMethod>) -> Self {
        self.raw.filter_method(method.into().into());
        self
    }

    pub fn rotation(self, rotation: impl Into<Rotation>) -> Self {
        self.raw.rotation(rotation.into().into());
        self
    }

    pub fn opacity(self, opacity: f32) -> Self {
        self.raw.opacity(opacity);
        self
    }

    pub fn scale(self, scale: f32) -> Self {
        self.raw.scale(scale);
        self
    }
}

impl<Message: 'static> From<Image> for Element<Message> {
    fn from(image: Image) -> Self {
        Element::new(move |_realize| WitImage::into_element(image.raw))
    }
}
