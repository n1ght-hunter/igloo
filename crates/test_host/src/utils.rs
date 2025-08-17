use iced::Radians;

use crate::bindings::iced::app::{
    alignment::{Alignment, Horizontal},
    length::Length,
    message::Rectangle,
    message_types::{AbsoluteOffset, RelativeOffset, Viewport},
    padding::Padding,
    scrollable::{Anchor, Direction, Scrollbar},
    shared::{Color, ContentFit, FilterMethod, Rotation},
    text::{LineHeight, Shaping, Vertical, Wrapping},
    tooltip::Position as TooltipPosition,
};

impl From<LineHeight> for iced::widget::text::LineHeight {
    fn from(value: LineHeight) -> Self {
        match value {
            LineHeight::Relative(value) => iced::widget::text::LineHeight::Relative(value),
            LineHeight::Absolute(value) => iced::widget::text::LineHeight::Absolute(value.into()),
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

impl From<Vertical> for iced::alignment::Vertical {
    fn from(value: Vertical) -> Self {
        match value {
            Vertical::Top => iced::alignment::Vertical::Top,
            Vertical::Center => iced::alignment::Vertical::Center,
            Vertical::Bottom => iced::alignment::Vertical::Bottom,
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

impl From<Horizontal> for iced::widget::text::Alignment {
    fn from(value: Horizontal) -> Self {
        match value {
            Horizontal::Left => iced::widget::text::Alignment::Left,
            Horizontal::Center => iced::widget::text::Alignment::Center,
            Horizontal::Right => iced::widget::text::Alignment::Right,
        }
    }
}
impl From<crate::bindings::iced::app::text::Alignment> for iced::widget::text::Alignment {
    fn from(value: crate::bindings::iced::app::text::Alignment) -> Self {
        match value {
            crate::bindings::iced::app::text::Alignment::Left => {
                iced::widget::text::Alignment::Left
            }
            crate::bindings::iced::app::text::Alignment::Center => {
                iced::widget::text::Alignment::Center
            }
            crate::bindings::iced::app::text::Alignment::Right => {
                iced::widget::text::Alignment::Right
            }
            crate::bindings::iced::app::text::Alignment::Default => {
                iced::widget::text::Alignment::Default
            }
            crate::bindings::iced::app::text::Alignment::Justified => {
                iced::widget::text::Alignment::Justified
            }
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

impl From<TooltipPosition> for iced::widget::tooltip::Position {
    fn from(position: TooltipPosition) -> Self {
        match position {
            TooltipPosition::Top => iced::widget::tooltip::Position::Top,
            TooltipPosition::Bottom => iced::widget::tooltip::Position::Bottom,
            TooltipPosition::Left => iced::widget::tooltip::Position::Left,
            TooltipPosition::Right => iced::widget::tooltip::Position::Right,
            TooltipPosition::FollowCursor => iced::widget::tooltip::Position::FollowCursor,
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

impl From<Shaping> for iced::widget::text::Shaping {
    fn from(value: Shaping) -> Self {
        match value {
            Shaping::Basic => iced::widget::text::Shaping::Basic,
            Shaping::Advanced => iced::widget::text::Shaping::Advanced,
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
            Rotation::Floating(radians) => iced::Rotation::Floating(Radians(radians)),
            Rotation::Solid(radians) => iced::Rotation::Solid(Radians(radians)),
        }
    }
}

impl From<Alignment> for iced::Alignment {
    fn from(value: Alignment) -> Self {
        match value {
            Alignment::Start => iced::Alignment::Start,
            Alignment::Center => iced::Alignment::Center,
            Alignment::End => iced::Alignment::End,
        }
    }
}

impl From<iced::widget::scrollable::AbsoluteOffset> for AbsoluteOffset {
    fn from(value: iced::widget::scrollable::AbsoluteOffset) -> Self {
        AbsoluteOffset {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<iced::widget::scrollable::RelativeOffset> for RelativeOffset {
    fn from(value: iced::widget::scrollable::RelativeOffset) -> Self {
        RelativeOffset {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<iced::widget::scrollable::Viewport> for Viewport {
    fn from(value: iced::widget::scrollable::Viewport) -> Self {
        Viewport {
            absolute_offset: value.absolute_offset().into(),
            absolute_offset_reversed: value.absolute_offset_reversed().into(),
            relative_offset: value.relative_offset().into(),
            bounds: value.bounds().into(),
            content_bounds: value.content_bounds().into(),
        }
    }
}

impl From<iced::Rectangle> for Rectangle {
    fn from(value: iced::Rectangle) -> Self {
        Rectangle {
            x: value.x,
            y: value.y,
            width: value.width,
            height: value.height,
        }
    }
}

impl From<Direction> for iced::widget::scrollable::Direction {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Vertical(scroll_bar) => {
                iced::widget::scrollable::Direction::Vertical(scroll_bar.into())
            }
            Direction::Horizontal(scroll_bar) => {
                iced::widget::scrollable::Direction::Horizontal(scroll_bar.into())
            }
            Direction::Both((hoz, ver)) => iced::widget::scrollable::Direction::Both {
                vertical: ver.into(),
                horizontal: hoz.into(),
            },
        }
    }
}

impl From<Scrollbar> for iced::widget::scrollable::Scrollbar {
    fn from(value: Scrollbar) -> Self {
        let Scrollbar {
            width,
            margin,
            scroller_width,
            alignment,
            spacing,
        } = value;
        let mut scrollbar = iced::widget::scrollable::Scrollbar::new();
        if let Some(width) = width {
            scrollbar = scrollbar.width(width);
        }
        if let Some(margin) = margin {
            scrollbar = scrollbar.margin(margin);
        }
        if let Some(scroller_width) = scroller_width {
            scrollbar = scrollbar.scroller_width(scroller_width);
        }
        if let Some(alignment) = alignment {
            scrollbar = scrollbar.anchor(alignment.into());
        }
        if let Some(spacing) = spacing {
            scrollbar = scrollbar.spacing(spacing);
        }
        scrollbar
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

#[allow(unsafe_code, dead_code)]
pub mod unsafe_conversion {
    use iced::widget::pane_grid::Pane;

    struct PaneRep(usize);

    /// SAFETY: This function is only safe to call if the value is a valid representation of a Pane.  and PaneRep and Pane have the same layout in memory.
    unsafe fn pane_to_usize(pane: Pane) -> usize {
        unsafe { std::mem::transmute::<Pane, PaneRep>(pane) }.0
    }

    /// SAFETY: This function is only safe to call if the value is a valid representation of a Pane. and PaneRep and Pane have the same layout in memory.
    unsafe fn usize_to_pane(value: usize) -> Pane {
        let rep = PaneRep(value);
        unsafe { std::mem::transmute::<PaneRep, Pane>(rep) }
    }

    const _: () = {
        assert!(std::mem::align_of::<PaneRep>() == std::mem::align_of::<usize>());
        assert!(std::mem::align_of::<usize>() == std::mem::align_of::<Pane>());
        assert!(std::mem::align_of::<Pane>() == std::mem::align_of::<PaneRep>());
        assert!(std::mem::size_of::<PaneRep>() == std::mem::size_of::<usize>());
        assert!(std::mem::size_of::<usize>() == std::mem::size_of::<Pane>());
        assert!(std::mem::size_of::<Pane>() == std::mem::size_of::<PaneRep>());
    };
}
