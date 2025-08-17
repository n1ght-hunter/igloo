use crate::{
    Element,
    bindings::iced::app::{
        alignment::{Alignment, Horizontal, Vertical},
        length::Length,
        padding::Padding,
        shared::{Color, ContentFit, FilterMethod, Rotation},
        text::{self, LineHeight},
        tooltip::Position as TooltipPosition,
    },
};

impl From<iced_core::text::LineHeight> for LineHeight {
    fn from(line_height: iced_core::text::LineHeight) -> Self {
        match line_height {
            iced_core::text::LineHeight::Absolute(value) => LineHeight::Absolute(value.0),
            iced_core::text::LineHeight::Relative(value) => LineHeight::Relative(value),
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

impl From<iced_core::text::Alignment> for text::Alignment {
    fn from(alignment: iced_core::text::Alignment) -> Self {
        match alignment {
            iced_core::text::Alignment::Default => text::Alignment::Default,
            iced_core::text::Alignment::Left => text::Alignment::Left,
            iced_core::text::Alignment::Center => text::Alignment::Center,
            iced_core::text::Alignment::Right => text::Alignment::Right,
            iced_core::text::Alignment::Justified => text::Alignment::Justified,
        }
    }
}

impl From<iced_core::Alignment> for Alignment {
    fn from(alignment: iced_core::Alignment) -> Self {
        match alignment {
            iced_core::Alignment::Start => Alignment::Start,
            iced_core::Alignment::Center => Alignment::Center,
            iced_core::Alignment::End => Alignment::End,
        }
    }
}

impl From<iced_core::alignment::Vertical> for Vertical {
    fn from(vertical: iced_core::alignment::Vertical) -> Self {
        match vertical {
            iced_core::alignment::Vertical::Top => Vertical::Top,
            iced_core::alignment::Vertical::Center => Vertical::Center,
            iced_core::alignment::Vertical::Bottom => Vertical::Bottom,
        }
    }
}

impl From<iced_core::text::Shaping> for text::Shaping {
    fn from(shaping: iced_core::text::Shaping) -> Self {
        match shaping {
            iced_core::text::Shaping::Basic => text::Shaping::Basic,
            iced_core::text::Shaping::Advanced => text::Shaping::Advanced,
        }
    }
}

impl From<iced_core::text::Wrapping> for text::Wrapping {
    fn from(wrapping: iced_core::text::Wrapping) -> Self {
        match wrapping {
            iced_core::text::Wrapping::None => text::Wrapping::None,
            iced_core::text::Wrapping::Word => text::Wrapping::Word,
            iced_core::text::Wrapping::Glyph => text::Wrapping::Glyph,
            iced_core::text::Wrapping::WordOrGlyph => text::Wrapping::WordOrGlyph,
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

impl From<iced_core::alignment::Horizontal> for Horizontal {
    fn from(value: iced_core::alignment::Horizontal) -> Self {
        match value {
            iced_core::alignment::Horizontal::Left => Horizontal::Left,
            iced_core::alignment::Horizontal::Center => Horizontal::Center,
            iced_core::alignment::Horizontal::Right => Horizontal::Right,
        }
    }
}

impl<'a, Message: 'a, T: Into<String>> From<T> for Element<Message> {
    fn from(value: T) -> Self {
        super::text(value).into()
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

impl From<super::tooltip::Position> for TooltipPosition {
    fn from(position: super::tooltip::Position) -> Self {
        match position {
            super::tooltip::Position::Top => TooltipPosition::Top,
            super::tooltip::Position::Bottom => TooltipPosition::Bottom,
            super::tooltip::Position::Left => TooltipPosition::Left,
            super::tooltip::Position::Right => TooltipPosition::Right,
            super::tooltip::Position::FollowCursor => TooltipPosition::FollowCursor,
        }
    }
}
