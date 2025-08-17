use super::Message;
use crate::{
    bindings::iced::app::container::Container,
    widgets::{ToElement, WrapperRenderer, WrapperTheme},
};

impl ToElement for Container {
    fn to_element<'a, Theme, Renderer>(
        self,
        resource_table: &mut wasmtime::component::ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let Container {
            content,
            padding,
            width,
            height,
            max_width,
            max_height,
            align_x,
            align_y,
            clip,
            center_x,
            center_y,
            center,
            align_left,
            align_right,
            align_top,
            align_bottom,
        } = self;

        let content = resource_table
            .delete(content)
            .unwrap()
            .to_element(resource_table);
        let mut container = iced::widget::Container::new(content);

        if let Some(padding) = padding {
            container = container.padding(padding);
        }

        if let Some(width) = width {
            container = container.width(width);
        }

        if let Some(height) = height {
            container = container.height(height);
        }

        if let Some(max_width) = max_width {
            container = container.max_width(max_width);
        }

        if let Some(max_height) = max_height {
            container = container.max_height(max_height);
        }

        if let Some(align_x) = align_x {
            container = container.align_x(align_x);
        }

        if let Some(align_y) = align_y {
            container = container.align_y(align_y);
        }

        if let Some(clip) = clip {
            container = container.clip(clip);
        }

        if let Some(center_x) = center_x {
            container = container.center_x(center_x);
        }

        if let Some(center_y) = center_y {
            container = container.center_y(center_y);
        }

        if let Some(center) = center {
            container = container.center(center);
        }

        if let Some(align_left) = align_left {
            container = container.align_left(align_left);
        }

        if let Some(align_right) = align_right {
            container = container.align_right(align_right);
        }

        if let Some(align_top) = align_top {
            container = container.align_top(align_top);
        }

        if let Some(align_bottom) = align_bottom {
            container = container.align_bottom(align_bottom);
        }

        container.into()
    }
}
