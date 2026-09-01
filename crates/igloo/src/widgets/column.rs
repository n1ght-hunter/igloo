use crate::{
    bindings::iced::app::{
        alignment::Horizontal,
        column::{self, HostColumn},
        length::Length,
        padding::Padding,
        shared::{Element, Pixels},
    },
    plugin_manager::MyState,
    widgets::{Message, ToElement, WrapperRenderer, WrapperTheme},
};
use wasmtime::component::Resource;

#[derive(Debug)]
pub struct ColumnResource {
    pub children: Vec<Resource<Element>>,
    pub spacing: Option<Pixels>,
    pub padding: Option<Padding>,
    pub width: Option<Length>,
    pub height: Option<Length>,
    pub max_width: Option<Pixels>,
    pub align_x: Option<Horizontal>,
    pub clip: Option<bool>,
}

impl ColumnResource {
    pub fn to_iced_element<'a, Theme, Renderer>(
        self,
        resource_table: &mut wasmtime::component::ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let children: Vec<iced::Element<'a, Message, Theme, Renderer>> = self
            .children
            .into_iter()
            .map(|e| resource_table.delete(e).unwrap().to_element(resource_table))
            .collect();

        let mut col = iced::widget::Column::with_children(children);
        if let Some(spacing) = self.spacing {
            col = col.spacing(spacing);
        }
        if let Some(padding) = self.padding {
            col = col.padding(padding);
        }
        if let Some(w) = self.width {
            col = col.width(w);
        }
        if let Some(h) = self.height {
            col = col.height(h);
        }
        if let Some(max) = self.max_width {
            col = col.max_width(max);
        }
        if let Some(align) = self.align_x {
            col = col.align_x(align);
        }
        if let Some(clip) = self.clip {
            col = col.clip(clip);
        }
        col.into()
    }
}

impl HostColumn for MyState {
    fn new(&mut self) -> Resource<column::Column> {
        self.table
            .push(ColumnResource {
                children: vec![],
                spacing: None,
                padding: None,
                width: None,
                height: None,
                max_width: None,
                align_x: None,
                clip: None,
            })
            .unwrap()
    }

    fn push(&mut self, self_: Resource<column::Column>, child: Resource<Element>) {
        self.table.get_mut(&self_).unwrap().children.push(child);
    }

    fn spacing(&mut self, self_: Resource<column::Column>, amount: Pixels) {
        self.table.get_mut(&self_).unwrap().spacing = Some(amount);
    }

    fn padding(&mut self, self_: Resource<column::Column>, p: Padding) {
        self.table.get_mut(&self_).unwrap().padding = Some(p);
    }

    fn width(&mut self, self_: Resource<column::Column>, w: Length) {
        self.table.get_mut(&self_).unwrap().width = Some(w);
    }

    fn height(&mut self, self_: Resource<column::Column>, h: Length) {
        self.table.get_mut(&self_).unwrap().height = Some(h);
    }

    fn max_width(&mut self, self_: Resource<column::Column>, max: Pixels) {
        self.table.get_mut(&self_).unwrap().max_width = Some(max);
    }

    fn align_x(&mut self, self_: Resource<column::Column>, align: Horizontal) {
        self.table.get_mut(&self_).unwrap().align_x = Some(align);
    }

    fn clip(&mut self, self_: Resource<column::Column>, clip: bool) {
        self.table.get_mut(&self_).unwrap().clip = Some(clip);
    }

    fn into_element(&mut self, self_: Resource<column::Column>) -> Resource<Element> {
        let col = self.table.delete(self_).unwrap();
        self.table.push(super::Element::Column(col)).unwrap()
    }

    fn drop(&mut self, rep: Resource<column::Column>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
