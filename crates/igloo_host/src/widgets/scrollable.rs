use super::Message;
use crate::{
    bindings::iced::app::scrollable::Scrollable,
    widgets::{ToElement, WrapperRenderer, WrapperTheme},
};

/// Display a horizontal or vertical rule for dividing content.
impl ToElement for Scrollable {
    fn to_element<'a, Theme, Renderer>(
        self,
        resource_table: &mut wasmtime::component::ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let Scrollable {
            content,
            width,
            height,
            on_scroll,
            direction,
        } = self;

        let mut scrollable = iced::widget::Scrollable::new(
            resource_table
                .delete(content)
                .unwrap()
                .to_element(resource_table),
        );

        if let Some(width) = width {
            scrollable = scrollable.width(width);
        }

        if let Some(height) = height {
            scrollable = scrollable.height(height);
        }

        if let Some(on_scroll) = on_scroll {
            scrollable = scrollable.on_scroll(move |v| Message {
                id: on_scroll,
                content: crate::bindings::Message::Viewport(v.into()),
            });
        }

        if let Some(direction) = direction {
            scrollable = scrollable.direction(direction);
        }

        scrollable.into()
    }
}
