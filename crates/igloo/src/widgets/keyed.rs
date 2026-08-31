use crate::{
    bindings::iced::app::{
        alignment::Alignment,
        keyed::{self, HostKeyedColumn},
        length::Length,
        padding::Padding,
        shared::{Element, Pixels},
    },
    plugin_manager::MyState,
    widgets::{Message, ToElement, WrapperRenderer, WrapperTheme},
};
use wasmtime::component::Resource;

#[derive(Debug)]
pub struct KeyedColumnResource {
    pub keys: Vec<u64>,
    pub children: Vec<Resource<Element>>,
    pub spacing: Option<Pixels>,
    pub padding: Option<Padding>,
    pub width: Option<Length>,
    pub height: Option<Length>,
    pub max_width: Option<Pixels>,
    pub align_items: Option<Alignment>,
}

impl KeyedColumnResource {
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

        let mut keyed_column = iced::widget::keyed::Column::from_vecs(self.keys, children);
        if let Some(spacing) = self.spacing {
            keyed_column = keyed_column.spacing(spacing);
        }
        if let Some(padding) = self.padding {
            keyed_column = keyed_column.padding(padding);
        }
        if let Some(w) = self.width {
            keyed_column = keyed_column.width(w);
        }
        if let Some(h) = self.height {
            keyed_column = keyed_column.height(h);
        }
        if let Some(max) = self.max_width {
            keyed_column = keyed_column.max_width(max);
        }
        if let Some(align) = self.align_items {
            keyed_column = keyed_column.align_items(align.into());
        }
        keyed_column.into()
    }
}

impl HostKeyedColumn for MyState {
    fn new(&mut self) -> Resource<keyed::KeyedColumn> {
        self.table
            .push(KeyedColumnResource {
                keys: vec![],
                children: vec![],
                spacing: None,
                padding: None,
                width: None,
                height: None,
                max_width: None,
                align_items: None,
            })
            .unwrap()
    }

    fn push(&mut self, self_: Resource<keyed::KeyedColumn>, key: u64, child: Resource<Element>) {
        let resource = self.table.get_mut(&self_).unwrap();
        resource.keys.push(key);
        resource.children.push(child);
    }

    fn spacing(&mut self, self_: Resource<keyed::KeyedColumn>, amount: Pixels) {
        self.table.get_mut(&self_).unwrap().spacing = Some(amount);
    }

    fn padding(&mut self, self_: Resource<keyed::KeyedColumn>, p: Padding) {
        self.table.get_mut(&self_).unwrap().padding = Some(p);
    }

    fn width(&mut self, self_: Resource<keyed::KeyedColumn>, w: Length) {
        self.table.get_mut(&self_).unwrap().width = Some(w);
    }

    fn height(&mut self, self_: Resource<keyed::KeyedColumn>, h: Length) {
        self.table.get_mut(&self_).unwrap().height = Some(h);
    }

    fn max_width(&mut self, self_: Resource<keyed::KeyedColumn>, amount: Pixels) {
        self.table.get_mut(&self_).unwrap().max_width = Some(amount);
    }

    fn align_items(&mut self, self_: Resource<keyed::KeyedColumn>, align: Alignment) {
        self.table.get_mut(&self_).unwrap().align_items = Some(align);
    }

    fn into_element(&mut self, self_: Resource<keyed::KeyedColumn>) -> Resource<Element> {
        let col = self.table.delete(self_).unwrap();
        self.table.push(super::Element::Keyed(col)).unwrap()
    }

    fn drop(&mut self, rep: Resource<keyed::KeyedColumn>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
