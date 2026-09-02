use crate::{
    bindings::iced::app::widgets::ContainerNode,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};

/// Builds an `iced::Container` from its node and its already-built content element.
pub fn build<'a, Theme, Renderer>(
    node: ContainerNode,
    content: iced::Element<'a, Message, Theme, Renderer>,
) -> iced::Element<'a, Message, Theme, Renderer>
where
    Theme: WrapperTheme + 'a,
    Renderer: WrapperRenderer + 'a,
{
    let mut container = iced::widget::Container::new(content);

    if let Some(padding) = node.padding {
        container = container.padding(padding);
    }
    if let Some(width) = node.width {
        container = container.width(width);
    }
    if let Some(height) = node.height {
        container = container.height(height);
    }
    if let Some(max_width) = node.max_width {
        container = container.max_width(max_width);
    }
    if let Some(max_height) = node.max_height {
        container = container.max_height(max_height);
    }
    if let Some(align_x) = node.align_x {
        container = container.align_x(align_x);
    }
    if let Some(align_y) = node.align_y {
        container = container.align_y(align_y);
    }
    if let Some(clip) = node.clip {
        container = container.clip(clip);
    }
    if let Some(center_x) = node.center_x {
        container = container.center_x(center_x);
    }
    if let Some(center_y) = node.center_y {
        container = container.center_y(center_y);
    }
    if let Some(center) = node.center {
        container = container.center(center);
    }
    if let Some(align_left) = node.align_left {
        container = container.align_left(align_left);
    }
    if let Some(align_right) = node.align_right {
        container = container.align_right(align_right);
    }
    if let Some(align_top) = node.align_top {
        container = container.align_top(align_top);
    }
    if let Some(align_bottom) = node.align_bottom {
        container = container.align_bottom(align_bottom);
    }

    container.into()
}
