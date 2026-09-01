use crate::{
    bindings::iced::app::{length::Length, progress_bar, shared::Element},
    plugin_manager::MyState,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};
use wasmtime::component::Resource;

#[derive(Debug)]
pub struct ProgressBarResource {
    pub range_start: f32,
    pub range_end: f32,
    pub value: f32,
    pub length: Option<Length>,
    pub girth: Option<Length>,
    pub vertical: Option<bool>,
}

impl ProgressBarResource {
    pub fn to_iced_element<'a, Theme, Renderer>(self) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let mut bar = iced::widget::ProgressBar::new(self.range_start..=self.range_end, self.value);

        if let Some(length) = self.length {
            bar = bar.length(length);
        }
        if let Some(girth) = self.girth {
            bar = bar.girth(girth);
        }
        if let Some(true) = self.vertical {
            bar = bar.vertical();
        }
        bar.into()
    }
}

impl progress_bar::HostProgressBar for MyState {
    fn new(
        &mut self,
        range_start: f32,
        range_end: f32,
        value: f32,
    ) -> Resource<progress_bar::ProgressBar> {
        self.table
            .push(ProgressBarResource {
                range_start,
                range_end,
                value,
                length: None,
                girth: None,
                vertical: None,
            })
            .unwrap()
    }

    fn length(&mut self, self_: Resource<progress_bar::ProgressBar>, l: Length) {
        self.table.get_mut(&self_).unwrap().length = Some(l);
    }

    fn girth(&mut self, self_: Resource<progress_bar::ProgressBar>, g: Length) {
        self.table.get_mut(&self_).unwrap().girth = Some(g);
    }

    fn vertical(&mut self, self_: Resource<progress_bar::ProgressBar>, v: bool) {
        self.table.get_mut(&self_).unwrap().vertical = Some(v);
    }

    fn into_element(&mut self, self_: Resource<progress_bar::ProgressBar>) -> Resource<Element> {
        let bar = self.table.delete(self_).unwrap();
        self.table.push(super::Element::ProgressBar(bar)).unwrap()
    }

    fn drop(&mut self, rep: Resource<progress_bar::ProgressBar>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
