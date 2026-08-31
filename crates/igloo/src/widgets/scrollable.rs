use crate::{
    bindings::iced::app::{
        length::Length,
        scrollable::{self, Direction, HostScrollable},
        shared::Element,
    },
    plugin_manager::MyState,
    widgets::{Message, ToElement, WrapperRenderer, WrapperTheme},
};
use wasmtime::component::Resource;

#[derive(Debug)]
pub struct ScrollableResource {
    pub content: Resource<Element>,
    pub width: Option<Length>,
    pub height: Option<Length>,
    pub direction: Option<Direction>,
    pub on_scroll: Option<u32>,
}

impl ScrollableResource {
    pub fn to_iced_element<'a, Theme, Renderer>(
        self,
        resource_table: &mut wasmtime::component::ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let content = resource_table
            .delete(self.content)
            .unwrap()
            .to_element(resource_table);

        let mut scrollable = iced::widget::Scrollable::new(content);
        if let Some(width) = self.width {
            scrollable = scrollable.width(width);
        }
        if let Some(height) = self.height {
            scrollable = scrollable.height(height);
        }
        if let Some(direction) = self.direction {
            scrollable = scrollable.direction(direction);
        }
        if let Some(mapper) = self.on_scroll {
            scrollable = scrollable.on_scroll(move |v| Message::Viewport {
                mapper,
                value: v.into(),
            });
        }
        scrollable.into()
    }
}

impl HostScrollable for MyState {
    fn new(&mut self, content: Resource<Element>) -> Resource<scrollable::Scrollable> {
        self.table
            .push(ScrollableResource {
                content,
                width: None,
                height: None,
                direction: None,
                on_scroll: None,
            })
            .unwrap()
    }

    fn width(&mut self, self_: Resource<scrollable::Scrollable>, w: Length) {
        self.table.get_mut(&self_).unwrap().width = Some(w);
    }

    fn height(&mut self, self_: Resource<scrollable::Scrollable>, h: Length) {
        self.table.get_mut(&self_).unwrap().height = Some(h);
    }

    fn direction(&mut self, self_: Resource<scrollable::Scrollable>, d: Direction) {
        self.table.get_mut(&self_).unwrap().direction = Some(d);
    }

    fn on_scroll(&mut self, self_: Resource<scrollable::Scrollable>, mapper: u32) {
        self.table.get_mut(&self_).unwrap().on_scroll = Some(mapper);
    }

    fn into_element(&mut self, self_: Resource<scrollable::Scrollable>) -> Resource<Element> {
        let scrollable = self.table.delete(self_).unwrap();
        self.table
            .push(super::Element::Scrollable(scrollable))
            .unwrap()
    }

    fn drop(&mut self, rep: Resource<scrollable::Scrollable>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
