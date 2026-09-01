use crate::{
    bindings::iced::app::{
        alignment::{Horizontal, Vertical},
        container::{self, HostContainer},
        length::Length,
        padding::Padding,
        shared::{Element, Pixels},
    },
    plugin_manager::MyState,
    widgets::{Message, ToElement, WrapperRenderer, WrapperTheme},
};
use wasmtime::component::Resource;

#[derive(Debug)]
pub struct ContainerResource {
    pub content: Resource<Element>,
    pub padding: Option<Padding>,
    pub width: Option<Length>,
    pub height: Option<Length>,
    pub max_width: Option<Pixels>,
    pub max_height: Option<Pixels>,
    pub align_x: Option<Horizontal>,
    pub align_y: Option<Vertical>,
    pub clip: Option<bool>,
    pub center_x: Option<Length>,
    pub center_y: Option<Length>,
    pub center: Option<Length>,
    pub align_left: Option<Length>,
    pub align_right: Option<Length>,
    pub align_top: Option<Length>,
    pub align_bottom: Option<Length>,
}

impl ContainerResource {
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
        let mut container = iced::widget::Container::new(content);

        if let Some(padding) = self.padding {
            container = container.padding(padding);
        }
        if let Some(width) = self.width {
            container = container.width(width);
        }
        if let Some(height) = self.height {
            container = container.height(height);
        }
        if let Some(max_width) = self.max_width {
            container = container.max_width(max_width);
        }
        if let Some(max_height) = self.max_height {
            container = container.max_height(max_height);
        }
        if let Some(align_x) = self.align_x {
            container = container.align_x(align_x);
        }
        if let Some(align_y) = self.align_y {
            container = container.align_y(align_y);
        }
        if let Some(clip) = self.clip {
            container = container.clip(clip);
        }
        if let Some(center_x) = self.center_x {
            container = container.center_x(center_x);
        }
        if let Some(center_y) = self.center_y {
            container = container.center_y(center_y);
        }
        if let Some(center) = self.center {
            container = container.center(center);
        }
        if let Some(align_left) = self.align_left {
            container = container.align_left(align_left);
        }
        if let Some(align_right) = self.align_right {
            container = container.align_right(align_right);
        }
        if let Some(align_top) = self.align_top {
            container = container.align_top(align_top);
        }
        if let Some(align_bottom) = self.align_bottom {
            container = container.align_bottom(align_bottom);
        }

        container.into()
    }
}

impl HostContainer for MyState {
    fn new(&mut self, content: Resource<Element>) -> Resource<container::Container> {
        self.table
            .push(ContainerResource {
                content,
                padding: None,
                width: None,
                height: None,
                max_width: None,
                max_height: None,
                align_x: None,
                align_y: None,
                clip: None,
                center_x: None,
                center_y: None,
                center: None,
                align_left: None,
                align_right: None,
                align_top: None,
                align_bottom: None,
            })
            .unwrap()
    }

    fn padding(&mut self, self_: Resource<container::Container>, p: Padding) {
        self.table.get_mut(&self_).unwrap().padding = Some(p);
    }

    fn width(&mut self, self_: Resource<container::Container>, w: Length) {
        self.table.get_mut(&self_).unwrap().width = Some(w);
    }

    fn height(&mut self, self_: Resource<container::Container>, h: Length) {
        self.table.get_mut(&self_).unwrap().height = Some(h);
    }

    fn max_width(&mut self, self_: Resource<container::Container>, max: Pixels) {
        self.table.get_mut(&self_).unwrap().max_width = Some(max);
    }

    fn max_height(&mut self, self_: Resource<container::Container>, max: Pixels) {
        self.table.get_mut(&self_).unwrap().max_height = Some(max);
    }

    fn center_x(&mut self, self_: Resource<container::Container>, w: Length) {
        self.table.get_mut(&self_).unwrap().center_x = Some(w);
    }

    fn center_y(&mut self, self_: Resource<container::Container>, h: Length) {
        self.table.get_mut(&self_).unwrap().center_y = Some(h);
    }

    fn center(&mut self, self_: Resource<container::Container>, l: Length) {
        self.table.get_mut(&self_).unwrap().center = Some(l);
    }

    fn align_left(&mut self, self_: Resource<container::Container>, w: Length) {
        self.table.get_mut(&self_).unwrap().align_left = Some(w);
    }

    fn align_right(&mut self, self_: Resource<container::Container>, w: Length) {
        self.table.get_mut(&self_).unwrap().align_right = Some(w);
    }

    fn align_top(&mut self, self_: Resource<container::Container>, h: Length) {
        self.table.get_mut(&self_).unwrap().align_top = Some(h);
    }

    fn align_bottom(&mut self, self_: Resource<container::Container>, h: Length) {
        self.table.get_mut(&self_).unwrap().align_bottom = Some(h);
    }

    fn align_x(&mut self, self_: Resource<container::Container>, align: Horizontal) {
        self.table.get_mut(&self_).unwrap().align_x = Some(align);
    }

    fn align_y(&mut self, self_: Resource<container::Container>, align: Vertical) {
        self.table.get_mut(&self_).unwrap().align_y = Some(align);
    }

    fn clip(&mut self, self_: Resource<container::Container>, clip: bool) {
        self.table.get_mut(&self_).unwrap().clip = Some(clip);
    }

    fn into_element(&mut self, self_: Resource<container::Container>) -> Resource<Element> {
        let container = self.table.delete(self_).unwrap();
        self.table
            .push(super::Element::Container(container))
            .unwrap()
    }

    fn drop(&mut self, rep: Resource<container::Container>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
