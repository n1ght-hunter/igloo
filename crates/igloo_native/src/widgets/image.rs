//! A frame that displays an image.

use stabby::option::Option as Opt;

use crate::widgets::Node;
use crate::widgets::element::{Arena, Element, Realize};
use crate::widgets::types::{ContentFit, FilterMethod, Length, Rotation};

/// A frame that displays an image.
#[stabby::stabby]
#[derive(Debug)]
pub struct ImageNode {
    pub handle: stabby::string::String,
    pub width: Opt<Length>,
    pub height: Opt<Length>,
    pub expand: Opt<bool>,
    pub content_fit: Opt<ContentFit>,
    pub filter_method: Opt<FilterMethod>,
    pub rotation: Opt<Rotation>,
    pub opacity: Opt<f32>,
    pub scale: Opt<f32>,
}

#[cfg(feature = "host")]
pub(crate) fn build(image: ImageNode) -> iced::Element<'static, crate::widgets::Emitted> {
    let handle = iced::advanced::image::Handle::from_path(std::string::String::from(image.handle));
    let mut widget = iced::widget::Image::new(handle);
    if let Some(width) = Option::<Length>::from(image.width) {
        widget = widget.width(iced::Length::from(width));
    }
    if let Some(height) = Option::<Length>::from(image.height) {
        widget = widget.height(iced::Length::from(height));
    }
    if let Some(expand) = Option::<bool>::from(image.expand) {
        widget = widget.expand(expand);
    }
    if let Some(fit) = Option::<ContentFit>::from(image.content_fit) {
        widget = widget.content_fit(iced::ContentFit::from(fit));
    }
    if let Some(method) = Option::<FilterMethod>::from(image.filter_method) {
        widget = widget.filter_method(iced::widget::image::FilterMethod::from(method));
    }
    if let Some(rotation) = Option::<Rotation>::from(image.rotation) {
        widget = widget.rotation(iced::Rotation::from(rotation));
    }
    if let Some(opacity) = Option::<f32>::from(image.opacity) {
        widget = widget.opacity(opacity);
    }
    if let Some(scale) = Option::<f32>::from(image.scale) {
        widget = widget.scale(scale);
    }
    widget.into()
}

/// A builder for an [`ImageNode`].
#[derive(Debug)]
pub struct Image {
    handle: std::string::String,
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
    /// Creates a new image with the given path.
    pub fn new(handle: impl Into<std::string::String>) -> Self {
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

    pub fn content_fit(mut self, fit: ContentFit) -> Self {
        self.content_fit = Some(fit);
        self
    }

    pub fn filter_method(mut self, method: FilterMethod) -> Self {
        self.filter_method = Some(method);
        self
    }

    pub fn rotation(mut self, rotation: Rotation) -> Self {
        self.rotation = Some(rotation);
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
        Element::new(move |_realize: &dyn Realize<Message>, arena: &mut Arena| {
            arena.push(Node::Image(ImageNode {
                handle: image.handle.into(),
                width: image.width.into(),
                height: image.height.into(),
                expand: image.expand.into(),
                content_fit: image.content_fit.into(),
                filter_method: image.filter_method.into(),
                rotation: image.rotation.into(),
                opacity: image.opacity.into(),
                scale: image.scale.into(),
            }))
        })
    }
}

/// Creates an [`Image`] with the given path.
pub fn image(handle: impl Into<std::string::String>) -> Image {
    Image::new(handle)
}
