use crate::bindings::iced::app::{
    alignment::{Alignment, Horizontal, Vertical},
    length::Length,
    message_types,
    padding::Padding,
    shared::{Color, ContentFit, FilterMethod, Rotation},
    widgets::{Anchor, Direction, LineHeight, Scrollbar, Shaping, TextAlignment, Wrapping},
};

impl From<LineHeight> for iced::widget::text::LineHeight {
    fn from(value: LineHeight) -> Self {
        match value {
            LineHeight::Relative(value) => iced::widget::text::LineHeight::Relative(value),
            LineHeight::Absolute(value) => iced::widget::text::LineHeight::Absolute(value.into()),
        }
    }
}

impl From<Shaping> for iced::widget::text::Shaping {
    fn from(value: Shaping) -> Self {
        match value {
            Shaping::Basic => iced::widget::text::Shaping::Basic,
            Shaping::Advanced => iced::widget::text::Shaping::Advanced,
            Shaping::Auto => iced::widget::text::Shaping::Auto,
        }
    }
}

impl From<Wrapping> for iced::widget::text::Wrapping {
    fn from(value: Wrapping) -> Self {
        match value {
            Wrapping::None => iced::widget::text::Wrapping::None,
            Wrapping::Word => iced::widget::text::Wrapping::Word,
            Wrapping::Glyph => iced::widget::text::Wrapping::Glyph,
            Wrapping::WordOrGlyph => iced::widget::text::Wrapping::WordOrGlyph,
        }
    }
}

impl From<Length> for iced::Length {
    fn from(value: Length) -> Self {
        match value {
            Length::Fill => iced::Length::Fill,
            Length::FillPortion(value) => iced::Length::FillPortion(value),
            Length::Shrink => iced::Length::Shrink,
            Length::Fixed(value) => iced::Length::Fixed(value),
        }
    }
}

impl From<Horizontal> for iced::alignment::Horizontal {
    fn from(value: Horizontal) -> Self {
        match value {
            Horizontal::Left => iced::alignment::Horizontal::Left,
            Horizontal::Center => iced::alignment::Horizontal::Center,
            Horizontal::Right => iced::alignment::Horizontal::Right,
        }
    }
}

impl From<Vertical> for iced::alignment::Vertical {
    fn from(value: Vertical) -> Self {
        match value {
            Vertical::Top => iced::alignment::Vertical::Top,
            Vertical::Center => iced::alignment::Vertical::Center,
            Vertical::Bottom => iced::alignment::Vertical::Bottom,
        }
    }
}

impl From<Alignment> for iced::alignment::Alignment {
    fn from(value: Alignment) -> Self {
        match value {
            Alignment::Start => iced::alignment::Alignment::Start,
            Alignment::Center => iced::alignment::Alignment::Center,
            Alignment::End => iced::alignment::Alignment::End,
        }
    }
}

impl From<TextAlignment> for iced::widget::text::Alignment {
    fn from(value: TextAlignment) -> Self {
        match value {
            TextAlignment::Left => iced::widget::text::Alignment::Left,
            TextAlignment::Center => iced::widget::text::Alignment::Center,
            TextAlignment::Right => iced::widget::text::Alignment::Right,
            TextAlignment::Default => iced::widget::text::Alignment::Default,
            TextAlignment::Justified => iced::widget::text::Alignment::Justified,
        }
    }
}

impl From<Anchor> for iced::widget::scrollable::Anchor {
    fn from(value: Anchor) -> Self {
        match value {
            Anchor::Start => iced::widget::scrollable::Anchor::Start,
            Anchor::End => iced::widget::scrollable::Anchor::End,
        }
    }
}

impl From<Scrollbar> for iced::widget::scrollable::Scrollbar {
    fn from(value: Scrollbar) -> Self {
        let mut scrollbar = iced::widget::scrollable::Scrollbar::new();
        if let Some(width) = value.width {
            scrollbar = scrollbar.width(width);
        }
        if let Some(margin) = value.margin {
            scrollbar = scrollbar.margin(margin);
        }
        if let Some(scroller_width) = value.scroller_width {
            scrollbar = scrollbar.scroller_width(scroller_width);
        }
        if let Some(anchor) = value.anchor {
            scrollbar = scrollbar.anchor(anchor.into());
        }
        if let Some(spacing) = value.spacing {
            scrollbar = scrollbar.spacing(spacing);
        }
        scrollbar
    }
}

impl From<Direction> for iced::widget::scrollable::Direction {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Vertical(scrollbar) => {
                iced::widget::scrollable::Direction::Vertical(scrollbar.into())
            }
            Direction::Horizontal(scrollbar) => {
                iced::widget::scrollable::Direction::Horizontal(scrollbar.into())
            }
            Direction::Both((vertical, horizontal)) => iced::widget::scrollable::Direction::Both {
                vertical: vertical.into(),
                horizontal: horizontal.into(),
            },
        }
    }
}

impl From<iced::widget::scrollable::Viewport> for message_types::Viewport {
    fn from(value: iced::widget::scrollable::Viewport) -> Self {
        message_types::Viewport {
            absolute_offset: value.absolute_offset().into(),
            absolute_offset_reversed: value.absolute_offset_reversed().into(),
            relative_offset: value.relative_offset().into(),
            bounds: value.bounds().into(),
            content_bounds: value.content_bounds().into(),
        }
    }
}

impl From<iced::widget::scrollable::AbsoluteOffset> for message_types::AbsoluteOffset {
    fn from(value: iced::widget::scrollable::AbsoluteOffset) -> Self {
        message_types::AbsoluteOffset {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<iced::widget::scrollable::RelativeOffset> for message_types::RelativeOffset {
    fn from(value: iced::widget::scrollable::RelativeOffset) -> Self {
        message_types::RelativeOffset {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<iced::Rectangle> for message_types::Rectangle {
    fn from(value: iced::Rectangle) -> Self {
        message_types::Rectangle {
            x: value.x,
            y: value.y,
            width: value.width,
            height: value.height,
        }
    }
}

impl From<Padding> for iced::Padding {
    fn from(value: Padding) -> Self {
        iced::Padding {
            left: value.left,
            right: value.right,
            top: value.top,
            bottom: value.bottom,
        }
    }
}

impl From<Color> for iced::Color {
    fn from(color: Color) -> Self {
        iced::Color {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        }
    }
}

impl From<ContentFit> for iced::ContentFit {
    fn from(value: ContentFit) -> Self {
        match value {
            ContentFit::Contain => iced::ContentFit::Contain,
            ContentFit::Cover => iced::ContentFit::Cover,
            ContentFit::Fill => iced::ContentFit::Fill,
            ContentFit::None => iced::ContentFit::None,
            ContentFit::ScaleDown => iced::ContentFit::ScaleDown,
        }
    }
}

impl From<FilterMethod> for iced::widget::image::FilterMethod {
    fn from(value: FilterMethod) -> Self {
        match value {
            FilterMethod::Nearest => iced::widget::image::FilterMethod::Nearest,
            FilterMethod::Linear => iced::widget::image::FilterMethod::Linear,
        }
    }
}

impl From<Rotation> for iced::Rotation {
    fn from(value: Rotation) -> Self {
        match value {
            Rotation::Floating(radians) => iced::Rotation::Floating(iced::Radians(radians)),
            Rotation::Solid(radians) => iced::Rotation::Solid(iced::Radians(radians)),
        }
    }
}
