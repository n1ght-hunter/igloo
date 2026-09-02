use crate::bindings::iced::app::{
    alignment::{Alignment, Horizontal, Vertical},
    length::Length,
    padding::Padding,
    shared::{Color, ContentFit, FilterMethod, Rotation},
    widgets::{LineHeight, Shaping, TextAlignment, Wrapping},
};

impl From<iced_core::text::LineHeight> for LineHeight {
    fn from(line_height: iced_core::text::LineHeight) -> Self {
        match line_height {
            iced_core::text::LineHeight::Absolute(value) => LineHeight::Absolute(value.0),
            iced_core::text::LineHeight::Relative(value) => LineHeight::Relative(value),
        }
    }
}

impl From<iced_core::text::Shaping> for Shaping {
    fn from(value: iced_core::text::Shaping) -> Self {
        match value {
            iced_core::text::Shaping::Basic => Shaping::Basic,
            iced_core::text::Shaping::Advanced => Shaping::Advanced,
            iced_core::text::Shaping::Auto => Shaping::Auto,
        }
    }
}

impl From<iced_core::text::Wrapping> for Wrapping {
    fn from(value: iced_core::text::Wrapping) -> Self {
        match value {
            iced_core::text::Wrapping::None => Wrapping::None,
            iced_core::text::Wrapping::Word => Wrapping::Word,
            iced_core::text::Wrapping::Glyph => Wrapping::Glyph,
            iced_core::text::Wrapping::WordOrGlyph => Wrapping::WordOrGlyph,
        }
    }
}

impl From<iced_core::Length> for Length {
    fn from(length: iced_core::Length) -> Self {
        match length {
            iced_core::Length::Fixed(value) => Length::Fixed(value),
            iced_core::Length::Fill => Length::Fill,
            iced_core::Length::Shrink => Length::Shrink,
            iced_core::Length::FillPortion(portion) => Length::FillPortion(portion),
        }
    }
}

impl From<iced_core::text::Alignment> for TextAlignment {
    fn from(alignment: iced_core::text::Alignment) -> Self {
        match alignment {
            iced_core::text::Alignment::Default => TextAlignment::Default,
            iced_core::text::Alignment::Left => TextAlignment::Left,
            iced_core::text::Alignment::Center => TextAlignment::Center,
            iced_core::text::Alignment::Right => TextAlignment::Right,
            iced_core::text::Alignment::Justified => TextAlignment::Justified,
        }
    }
}

impl From<iced_core::alignment::Horizontal> for Horizontal {
    fn from(value: iced_core::alignment::Horizontal) -> Self {
        match value {
            iced_core::alignment::Horizontal::Left => Horizontal::Left,
            iced_core::alignment::Horizontal::Center => Horizontal::Center,
            iced_core::alignment::Horizontal::Right => Horizontal::Right,
        }
    }
}

impl From<iced_core::alignment::Vertical> for Vertical {
    fn from(value: iced_core::alignment::Vertical) -> Self {
        match value {
            iced_core::alignment::Vertical::Top => Vertical::Top,
            iced_core::alignment::Vertical::Center => Vertical::Center,
            iced_core::alignment::Vertical::Bottom => Vertical::Bottom,
        }
    }
}

impl From<iced_core::alignment::Alignment> for Alignment {
    fn from(value: iced_core::alignment::Alignment) -> Self {
        match value {
            iced_core::alignment::Alignment::Start => Alignment::Start,
            iced_core::alignment::Alignment::Center => Alignment::Center,
            iced_core::alignment::Alignment::End => Alignment::End,
        }
    }
}

impl From<iced_core::Padding> for Padding {
    fn from(padding: iced_core::Padding) -> Self {
        Padding {
            left: padding.left,
            right: padding.right,
            top: padding.top,
            bottom: padding.bottom,
        }
    }
}

impl From<iced_core::Color> for Color {
    fn from(color: iced_core::Color) -> Self {
        Color {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        }
    }
}

impl From<iced_core::ContentFit> for ContentFit {
    fn from(value: iced_core::ContentFit) -> Self {
        match value {
            iced_core::ContentFit::Contain => ContentFit::Contain,
            iced_core::ContentFit::Cover => ContentFit::Cover,
            iced_core::ContentFit::Fill => ContentFit::Fill,
            iced_core::ContentFit::None => ContentFit::None,
            iced_core::ContentFit::ScaleDown => ContentFit::ScaleDown,
        }
    }
}

impl From<iced_core::image::FilterMethod> for FilterMethod {
    fn from(value: iced_core::image::FilterMethod) -> Self {
        match value {
            iced_core::image::FilterMethod::Nearest => FilterMethod::Nearest,
            iced_core::image::FilterMethod::Linear => FilterMethod::Linear,
        }
    }
}

impl From<iced_core::Rotation> for Rotation {
    fn from(value: iced_core::Rotation) -> Self {
        match value {
            iced_core::Rotation::Floating(radians) => Rotation::Floating(radians.0),
            iced_core::Rotation::Solid(radians) => Rotation::Solid(radians.0),
        }
    }
}

impl From<crate::widgets::tooltip::Position> for crate::bindings::iced::app::widgets::Position {
    fn from(value: crate::widgets::tooltip::Position) -> Self {
        use crate::bindings::iced::app::widgets::Position as WitPosition;
        match value {
            crate::widgets::tooltip::Position::Top => WitPosition::Top,
            crate::widgets::tooltip::Position::Bottom => WitPosition::Bottom,
            crate::widgets::tooltip::Position::Left => WitPosition::Left,
            crate::widgets::tooltip::Position::Right => WitPosition::Right,
            crate::widgets::tooltip::Position::FollowCursor => WitPosition::FollowCursor,
        }
    }
}

// The generated `Scrollbar` type is defined by the `wit_bindgen::generate!` macro,
// so a `#[derive(Default)]` can't be attached to its declaration directly.
#[allow(clippy::derivable_impls)]
impl Default for crate::bindings::iced::app::widgets::Scrollbar {
    fn default() -> Self {
        Self {
            width: None,
            margin: None,
            scroller_width: None,
            anchor: None,
            spacing: None,
        }
    }
}

impl<'a, Message: 'static> From<&'a String> for crate::Element<Message> {
    fn from(s: &'a String) -> Self {
        crate::widgets::Text::new(s.clone()).into()
    }
}

impl<Message: 'static> From<String> for crate::Element<Message> {
    fn from(s: String) -> Self {
        crate::widgets::Text::new(s).into()
    }
}
