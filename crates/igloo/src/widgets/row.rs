use crate::{
    bindings::iced::app::{
        alignment::Vertical,
        length::Length,
        padding::Padding,
        row::{self, HostRow},
        shared::{Element, Pixels},
    },
    plugin_manager::MyState,
    widgets::{Message, ToElement, WrapperRenderer, WrapperTheme},
};
use wasmtime::component::Resource;

#[derive(Debug)]
pub struct RowResource {
    pub children: Vec<Resource<Element>>,
    pub spacing: Option<Pixels>,
    pub padding: Option<Padding>,
    pub width: Option<Length>,
    pub height: Option<Length>,
    pub align_y: Option<Vertical>,
    pub clip: Option<bool>,
    pub wrap: Option<bool>,
}

impl RowResource {
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

        let mut row = iced::widget::Row::with_children(children);
        if let Some(spacing) = self.spacing {
            row = row.spacing(spacing);
        }
        if let Some(padding) = self.padding {
            row = row.padding(padding);
        }
        if let Some(w) = self.width {
            row = row.width(w);
        }
        if let Some(h) = self.height {
            row = row.height(h);
        }
        if let Some(align_y) = self.align_y {
            row = row.align_y(align_y);
        }
        if let Some(clip) = self.clip {
            row = row.clip(clip);
        }

        // must be last as it returns a different element type
        if let Some(true) = self.wrap {
            return iced::Element::from(row.wrap());
        }

        row.into()
    }
}

impl HostRow for MyState {
    fn new(&mut self) -> Resource<row::Row> {
        self.table
            .push(RowResource {
                children: vec![],
                spacing: None,
                padding: None,
                width: None,
                height: None,
                align_y: None,
                clip: None,
                wrap: None,
            })
            .unwrap()
    }

    fn push(&mut self, self_: Resource<row::Row>, child: Resource<Element>) {
        self.table.get_mut(&self_).unwrap().children.push(child);
    }

    fn spacing(&mut self, self_: Resource<row::Row>, amount: Pixels) {
        self.table.get_mut(&self_).unwrap().spacing = Some(amount);
    }

    fn padding(&mut self, self_: Resource<row::Row>, p: Padding) {
        self.table.get_mut(&self_).unwrap().padding = Some(p);
    }

    fn width(&mut self, self_: Resource<row::Row>, w: Length) {
        self.table.get_mut(&self_).unwrap().width = Some(w);
    }

    fn height(&mut self, self_: Resource<row::Row>, h: Length) {
        self.table.get_mut(&self_).unwrap().height = Some(h);
    }

    fn align_y(&mut self, self_: Resource<row::Row>, align: Vertical) {
        self.table.get_mut(&self_).unwrap().align_y = Some(align);
    }

    fn clip(&mut self, self_: Resource<row::Row>, clip: bool) {
        self.table.get_mut(&self_).unwrap().clip = Some(clip);
    }

    fn wrap(&mut self, self_: Resource<row::Row>, wrap: bool) {
        self.table.get_mut(&self_).unwrap().wrap = Some(wrap);
    }

    fn into_element(&mut self, self_: Resource<row::Row>) -> Resource<Element> {
        let row = self.table.delete(self_).unwrap();
        self.table.push(super::Element::Row(row)).unwrap()
    }

    fn drop(&mut self, rep: Resource<row::Row>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
