//! Shared ABI value types used across widget nodes.

use stabby::option::Option as Opt;

/// Sizing strategy for a widget along one axis.
#[stabby::stabby]
#[repr(C, u8)]
#[derive(Debug, Clone, Copy)]
pub enum Length {
    Fill,
    FillPortion(u16),
    Shrink,
    Fixed(f32),
}

impl Length {
    /// A fixed length, in logical pixels.
    pub fn fill_portion(portion: u16) -> Self {
        Length::FillPortion(portion)
    }
}

impl From<f32> for Length {
    fn from(pixels: f32) -> Self {
        Length::Fixed(pixels)
    }
}

/// Line height, either relative to the font size or an absolute value.
#[stabby::stabby]
#[repr(C, u8)]
#[derive(Debug, Clone, Copy)]
pub enum LineHeight {
    Relative(f32),
    Absolute(f32),
}

/// Horizontal alignment of text within its bounds.
#[stabby::stabby]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum TextAlignment {
    Default,
    Left,
    Center,
    Right,
    Justified,
}

/// Vertical alignment.
#[stabby::stabby]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Vertical {
    Top,
    Center,
    Bottom,
}

/// Horizontal alignment.
#[stabby::stabby]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Horizontal {
    Left,
    Center,
    Right,
}

/// Generic cross-axis alignment.
#[stabby::stabby]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Alignment {
    Start,
    Center,
    End,
}

/// An RGBA color with components in the `0.0..=1.0` range.
#[stabby::stabby]
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    /// An opaque color from its red, green and blue components.
    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// A color from its red, green, blue and alpha components.
    pub fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

/// Padding around a widget, in logical pixels.
#[stabby::stabby]
#[derive(Debug, Clone, Copy)]
pub struct Padding {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl From<f32> for Padding {
    fn from(uniform: f32) -> Self {
        Self {
            left: uniform,
            right: uniform,
            top: uniform,
            bottom: uniform,
        }
    }
}

impl From<[f32; 2]> for Padding {
    fn from([vertical, horizontal]: [f32; 2]) -> Self {
        Self {
            top: vertical,
            bottom: vertical,
            left: horizontal,
            right: horizontal,
        }
    }
}

impl From<[f32; 4]> for Padding {
    fn from([top, right, bottom, left]: [f32; 4]) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }
}

/// Where a [`crate::widgets::tooltip::Tooltip`] appears relative to its content.
#[stabby::stabby]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Position {
    Top,
    Bottom,
    Left,
    Right,
    FollowCursor,
}

/// A 2D translation, in logical pixels.
#[stabby::stabby]
#[derive(Debug, Clone, Copy)]
pub struct Translation {
    pub x: f32,
    pub y: f32,
}

/// How an image or svg should be scaled to fit its bounds.
#[stabby::stabby]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum ContentFit {
    Contain,
    Cover,
    Fill,
    None,
    ScaleDown,
}

/// How an image is sampled when scaled.
#[stabby::stabby]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum FilterMethod {
    Nearest,
    Linear,
}

/// How text is shaped before layout.
#[stabby::stabby]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Shaping {
    Basic,
    Advanced,
    Auto,
}

/// How text wraps when it overflows its bounds.
#[stabby::stabby]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Wrapping {
    None,
    Word,
    Glyph,
    WordOrGlyph,
}

/// Rotation applied to an image, either preserving or ignoring its bounds.
#[stabby::stabby]
#[repr(C, u8)]
#[derive(Debug, Clone, Copy)]
pub enum Rotation {
    Floating(f32),
    Solid(f32),
}

/// Which edge of a [`Scrollbar`]'s axis its content is anchored to.
#[stabby::stabby]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Anchor {
    Start,
    End,
}

/// Settings for one axis of a scrollable's scrollbar.
#[stabby::stabby]
#[derive(Debug, Clone, Default)]
pub struct Scrollbar {
    pub width: Opt<f32>,
    pub margin: Opt<f32>,
    pub scroller_width: Opt<f32>,
    pub anchor: Opt<Anchor>,
    pub spacing: Opt<f32>,
}

/// Which axes a scrollable scrolls along, and each axis's [`Scrollbar`] settings.
///
/// The tuple order in [`Direction::Both`] is `(vertical, horizontal)`.
#[stabby::stabby]
#[repr(C, u8)]
#[derive(Debug, Clone)]
pub enum Direction {
    Vertical(Scrollbar),
    Horizontal(Scrollbar),
    Both(Scrollbar, Scrollbar),
}

/// A rectangle at `(x, y)` with the given `width`/`height`, in logical pixels.
#[stabby::stabby]
#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// An offset in absolute logical pixels.
#[stabby::stabby]
#[derive(Debug, Clone, Copy)]
pub struct AbsoluteOffset {
    pub x: f32,
    pub y: f32,
}

/// An offset relative to the scrollable bounds, in the `0.0..=1.0` range.
#[stabby::stabby]
#[derive(Debug, Clone, Copy)]
pub struct RelativeOffset {
    pub x: f32,
    pub y: f32,
}

/// The current scroll position and bounds of a scrollable.
#[stabby::stabby]
#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    pub absolute_offset: AbsoluteOffset,
    pub absolute_offset_reversed: AbsoluteOffset,
    pub relative_offset: RelativeOffset,
    pub bounds: Rectangle,
    pub content_bounds: Rectangle,
}

#[cfg(feature = "host")]
mod host {
    use super::{
        AbsoluteOffset, Alignment, Anchor, Color, ContentFit, Direction, FilterMethod, Horizontal,
        Length, LineHeight, Padding, Position, Rectangle, RelativeOffset, Rotation, Scrollbar,
        Shaping, TextAlignment, Vertical, Viewport, Wrapping,
    };

    impl From<Shaping> for iced::widget::text::Shaping {
        fn from(shaping: Shaping) -> Self {
            match shaping {
                Shaping::Basic => iced::widget::text::Shaping::Basic,
                Shaping::Advanced => iced::widget::text::Shaping::Advanced,
                Shaping::Auto => iced::widget::text::Shaping::Auto,
            }
        }
    }

    impl From<Wrapping> for iced::widget::text::Wrapping {
        fn from(wrapping: Wrapping) -> Self {
            match wrapping {
                Wrapping::None => iced::widget::text::Wrapping::None,
                Wrapping::Word => iced::widget::text::Wrapping::Word,
                Wrapping::Glyph => iced::widget::text::Wrapping::Glyph,
                Wrapping::WordOrGlyph => iced::widget::text::Wrapping::WordOrGlyph,
            }
        }
    }

    impl From<Alignment> for iced::Alignment {
        fn from(alignment: Alignment) -> Self {
            match alignment {
                Alignment::Start => iced::Alignment::Start,
                Alignment::Center => iced::Alignment::Center,
                Alignment::End => iced::Alignment::End,
            }
        }
    }

    impl From<Position> for iced::widget::tooltip::Position {
        fn from(position: Position) -> Self {
            match position {
                Position::Top => iced::widget::tooltip::Position::Top,
                Position::Bottom => iced::widget::tooltip::Position::Bottom,
                Position::Left => iced::widget::tooltip::Position::Left,
                Position::Right => iced::widget::tooltip::Position::Right,
                Position::FollowCursor => iced::widget::tooltip::Position::FollowCursor,
            }
        }
    }

    impl From<ContentFit> for iced::ContentFit {
        fn from(fit: ContentFit) -> Self {
            match fit {
                ContentFit::Contain => iced::ContentFit::Contain,
                ContentFit::Cover => iced::ContentFit::Cover,
                ContentFit::Fill => iced::ContentFit::Fill,
                ContentFit::None => iced::ContentFit::None,
                ContentFit::ScaleDown => iced::ContentFit::ScaleDown,
            }
        }
    }

    impl From<FilterMethod> for iced::widget::image::FilterMethod {
        fn from(method: FilterMethod) -> Self {
            match method {
                FilterMethod::Nearest => iced::widget::image::FilterMethod::Nearest,
                FilterMethod::Linear => iced::widget::image::FilterMethod::Linear,
            }
        }
    }

    impl From<Rotation> for iced::Rotation {
        fn from(rotation: Rotation) -> Self {
            match rotation {
                Rotation::Floating(radians) => iced::Rotation::Floating(iced::Radians(radians)),
                Rotation::Solid(radians) => iced::Rotation::Solid(iced::Radians(radians)),
            }
        }
    }

    impl From<Length> for iced::Length {
        fn from(length: Length) -> Self {
            match length {
                Length::Fill => iced::Length::Fill,
                Length::FillPortion(portion) => iced::Length::FillPortion(portion),
                Length::Shrink => iced::Length::Shrink,
                Length::Fixed(pixels) => iced::Length::Fixed(pixels),
            }
        }
    }

    impl From<LineHeight> for iced::widget::text::LineHeight {
        fn from(line_height: LineHeight) -> Self {
            match line_height {
                LineHeight::Relative(factor) => iced::widget::text::LineHeight::Relative(factor),
                LineHeight::Absolute(pixels) => {
                    iced::widget::text::LineHeight::Absolute(pixels.into())
                }
            }
        }
    }

    impl From<TextAlignment> for iced::widget::text::Alignment {
        fn from(alignment: TextAlignment) -> Self {
            match alignment {
                TextAlignment::Default => iced::widget::text::Alignment::Default,
                TextAlignment::Left => iced::widget::text::Alignment::Left,
                TextAlignment::Center => iced::widget::text::Alignment::Center,
                TextAlignment::Right => iced::widget::text::Alignment::Right,
                TextAlignment::Justified => iced::widget::text::Alignment::Justified,
            }
        }
    }

    impl From<Vertical> for iced::alignment::Vertical {
        fn from(vertical: Vertical) -> Self {
            match vertical {
                Vertical::Top => iced::alignment::Vertical::Top,
                Vertical::Center => iced::alignment::Vertical::Center,
                Vertical::Bottom => iced::alignment::Vertical::Bottom,
            }
        }
    }

    impl From<Horizontal> for iced::alignment::Horizontal {
        fn from(horizontal: Horizontal) -> Self {
            match horizontal {
                Horizontal::Left => iced::alignment::Horizontal::Left,
                Horizontal::Center => iced::alignment::Horizontal::Center,
                Horizontal::Right => iced::alignment::Horizontal::Right,
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

    impl From<Padding> for iced::Padding {
        fn from(padding: Padding) -> Self {
            iced::Padding {
                top: padding.top,
                right: padding.right,
                bottom: padding.bottom,
                left: padding.left,
            }
        }
    }

    impl From<Anchor> for iced::widget::scrollable::Anchor {
        fn from(anchor: Anchor) -> Self {
            match anchor {
                Anchor::Start => iced::widget::scrollable::Anchor::Start,
                Anchor::End => iced::widget::scrollable::Anchor::End,
            }
        }
    }

    impl From<Scrollbar> for iced::widget::scrollable::Scrollbar {
        fn from(scrollbar: Scrollbar) -> Self {
            let mut built = iced::widget::scrollable::Scrollbar::new();
            if let Some(width) = Option::<f32>::from(scrollbar.width) {
                built = built.width(width);
            }
            if let Some(margin) = Option::<f32>::from(scrollbar.margin) {
                built = built.margin(margin);
            }
            if let Some(scroller_width) = Option::<f32>::from(scrollbar.scroller_width) {
                built = built.scroller_width(scroller_width);
            }
            if let Some(anchor) = Option::<Anchor>::from(scrollbar.anchor) {
                built = built.anchor(anchor.into());
            }
            if let Some(spacing) = Option::<f32>::from(scrollbar.spacing) {
                built = built.spacing(spacing);
            }
            built
        }
    }

    impl From<Direction> for iced::widget::scrollable::Direction {
        fn from(direction: Direction) -> Self {
            match direction {
                Direction::Vertical(scrollbar) => {
                    iced::widget::scrollable::Direction::Vertical(scrollbar.into())
                }
                Direction::Horizontal(scrollbar) => {
                    iced::widget::scrollable::Direction::Horizontal(scrollbar.into())
                }
                Direction::Both(vertical, horizontal) => {
                    iced::widget::scrollable::Direction::Both {
                        vertical: vertical.into(),
                        horizontal: horizontal.into(),
                    }
                }
            }
        }
    }

    impl From<iced::Rectangle> for Rectangle {
        fn from(rectangle: iced::Rectangle) -> Self {
            Rectangle {
                x: rectangle.x,
                y: rectangle.y,
                width: rectangle.width,
                height: rectangle.height,
            }
        }
    }

    impl From<iced::widget::scrollable::AbsoluteOffset> for AbsoluteOffset {
        fn from(offset: iced::widget::scrollable::AbsoluteOffset) -> Self {
            AbsoluteOffset {
                x: offset.x,
                y: offset.y,
            }
        }
    }

    impl From<iced::widget::scrollable::RelativeOffset> for RelativeOffset {
        fn from(offset: iced::widget::scrollable::RelativeOffset) -> Self {
            RelativeOffset {
                x: offset.x,
                y: offset.y,
            }
        }
    }

    impl From<iced::widget::scrollable::Viewport> for Viewport {
        fn from(viewport: iced::widget::scrollable::Viewport) -> Self {
            Viewport {
                absolute_offset: viewport.absolute_offset().into(),
                absolute_offset_reversed: viewport.absolute_offset_reversed().into(),
                relative_offset: viewport.relative_offset().into(),
                bounds: viewport.bounds.into(),
                content_bounds: Rectangle {
                    x: viewport.bounds.x,
                    y: viewport.bounds.y,
                    width: viewport.content.width,
                    height: viewport.content.height,
                },
            }
        }
    }
}
