use super::Message;
use crate::bindings::iced::app::progress_bar::ProgressBar;
use crate::widgets::{ToElement, WrapperRenderer, WrapperTheme};

impl ToElement for ProgressBar {
    fn to_element<'a, Theme, Renderer>(
        self,
        _resource_table: &mut wasmtime::component::ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let ProgressBar {
            range_start,
            range_end,
            value,
            length,
            girth,
            vertical,
        } = self;

        let mut bar = iced::widget::ProgressBar::new(range_start..=range_end, value);

        if let Some(length) = length {
            bar = bar.length(length);
        }

        if let Some(girth) = girth {
            bar = bar.girth(girth);
        }

        if let Some(true) = vertical {
            bar = bar.vertical();
        }

        bar.into()
    }
}
