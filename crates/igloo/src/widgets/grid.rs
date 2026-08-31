use crate::{
    bindings::iced::app::{
        grid::{self, HostGrid},
        shared::{Element, Pixels},
    },
    plugin_manager::MyState,
    widgets::{Message, ToElement, WrapperRenderer, WrapperTheme},
};
use wasmtime::component::Resource;

#[derive(Debug)]
pub struct GridResource {
    pub elements: Vec<Resource<Element>>,
    pub spacing: Option<Pixels>,
    pub width: Option<Pixels>,
    pub height: Option<Pixels>,
    pub columns: Option<u64>,
    pub fluid: Option<Pixels>,
}

impl GridResource {
    pub fn to_iced_element<'a, Theme, Renderer>(
        self,
        resource_table: &mut wasmtime::component::ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let children: Vec<_> = self
            .elements
            .into_iter()
            .map(|e| resource_table.delete(e).unwrap().to_element(resource_table))
            .collect();

        let mut grid = iced::widget::Grid::with_children(children);

        if let Some(spacing) = self.spacing {
            grid = grid.spacing(spacing);
        }
        if let Some(width) = self.width {
            grid = grid.width(width);
        }
        if let Some(height) = self.height {
            grid = grid.height(height);
        }
        if let Some(columns) = self.columns {
            grid = grid.columns(columns as usize);
        }
        if let Some(fluid) = self.fluid {
            grid = grid.fluid(fluid);
        }

        grid.into()
    }
}

impl HostGrid for MyState {
    fn new(&mut self) -> Resource<grid::Grid> {
        self.table
            .push(GridResource {
                elements: vec![],
                spacing: None,
                width: None,
                height: None,
                columns: None,
                fluid: None,
            })
            .unwrap()
    }

    fn push(&mut self, self_: Resource<grid::Grid>, child: Resource<Element>) {
        self.table.get_mut(&self_).unwrap().elements.push(child);
    }

    fn spacing(&mut self, self_: Resource<grid::Grid>, amount: Pixels) {
        self.table.get_mut(&self_).unwrap().spacing = Some(amount);
    }

    fn width(&mut self, self_: Resource<grid::Grid>, w: Pixels) {
        self.table.get_mut(&self_).unwrap().width = Some(w);
    }

    fn height(&mut self, self_: Resource<grid::Grid>, h: Pixels) {
        self.table.get_mut(&self_).unwrap().height = Some(h);
    }

    fn columns(&mut self, self_: Resource<grid::Grid>, columns: u64) {
        self.table.get_mut(&self_).unwrap().columns = Some(columns);
    }

    fn fluid(&mut self, self_: Resource<grid::Grid>, amount: Pixels) {
        self.table.get_mut(&self_).unwrap().fluid = Some(amount);
    }

    fn into_element(&mut self, self_: Resource<grid::Grid>) -> Resource<Element> {
        let grid = self.table.delete(self_).unwrap();
        self.table.push(super::Element::Grid(grid)).unwrap()
    }

    fn drop(&mut self, rep: Resource<grid::Grid>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
