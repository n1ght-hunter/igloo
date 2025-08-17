use crate::bindings::iced::app::text::Text;

impl<'a, Message, Theme, Renderer> From<Text> for iced::Element<'a, Message, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer + 'a,
{
    fn from(value: Text) -> Self {
        let Text {
            text,
            size,
            line_height,
            width,
            height,
            center,
            align_x,
            align_y,
            shaping,
            wrapping,
            color: _color,
        } = value;
        let mut text = iced::widget::Text::new(text);

        if let Some(size) = size {
            text = text.size(size);
        }
        if let Some(line_height) = line_height {
            text = text.line_height(line_height);
        }
        if let Some(width) = width {
            text = text.width(width);
        }

        if let Some(height) = height {
            text = text.height(height);
        }

        if let Some(true) = center {
            text = text.center();
        }

        if let Some(horizontal_alignment) = align_x {
            text = text.align_x(horizontal_alignment);
        }

        if let Some(vertical_alignment) = align_y {
            text = text.align_y(vertical_alignment);
        }

        if let Some(shaping) = shaping {
            text = text.shaping(shaping.into());
        }

        if let Some(wrapping) = wrapping {
            text = text.wrapping(wrapping.into());
        }

        iced::Element::from(text)
    }
}
