use igloo_guest::{
    Alignment, Color, Element, Length,
    alignment::Horizontal,
    image::FilterMethod,
    widgets::{
        Column, Container, Slider, button, button::Button, checkbox, column, container,
        horizontal_space, radio, row, scrollable, slider, text, text_input, toggler,
        vertical_space,
    },
};

impl igloo_guest::Application<Tour, Message> for Tour {
    fn new() -> Self
    where
        Self: Sized,
    {
        Tour::new()
    }

    fn view(&self) -> Element<Message> {
        self.view()
    }

    fn update(&mut self, message: Message) {
        self.update(message);
    }
}

igloo_guest::export_guest!(Tour, Message);

#[allow(missing_debug_implementations)]
pub struct Tour {
    screen: Screen,
    slider: f32,
    layout: Layout,
    spacing: f32,
    text_size: f32,
    text_color: Color,
    language: Option<Language>,
    toggler: bool,
    image_width: f32,
    image_filter_method: FilterMethod,
    input_value: String,
    input_is_secure: bool,
    input_is_showing_icon: bool,
    debug: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    BackPressed,
    NextPressed,
    SliderChanged(f32),
    LayoutChanged(Layout),
    SpacingChanged(f32),
    TextSizeChanged(f32),
    TextColorChanged(Color),
    LanguageSelected(Language),
    ImageWidthChanged(f32),
    ImageUseNearestToggled(bool),
    InputChanged(String),
    ToggleSecureInput(bool),
    ToggleTextInputIcon(bool),
    DebugToggled(bool),
    TogglerChanged(bool),
    OpenTrunk,
}

impl Tour {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::BackPressed => {
                if let Some(screen) = self.screen.previous() {
                    self.screen = screen;
                }
            }
            Message::NextPressed => {
                if let Some(screen) = self.screen.next() {
                    self.screen = screen;
                }
            }
            Message::SliderChanged(value) => {
                self.slider = value;
            }
            Message::LayoutChanged(layout) => {
                self.layout = layout;
            }
            Message::SpacingChanged(spacing) => {
                self.spacing = spacing;
            }
            Message::TextSizeChanged(text_size) => {
                self.text_size = text_size;
            }
            Message::TextColorChanged(text_color) => {
                self.text_color = text_color;
            }
            Message::LanguageSelected(language) => {
                self.language = Some(language);
            }
            Message::ImageWidthChanged(image_width) => {
                self.image_width = image_width;
            }
            Message::ImageUseNearestToggled(use_nearest) => {
                self.image_filter_method = if use_nearest {
                    FilterMethod::Nearest
                } else {
                    FilterMethod::Linear
                };
            }
            Message::InputChanged(input_value) => {
                self.input_value = input_value;
            }
            Message::ToggleSecureInput(is_secure) => {
                self.input_is_secure = is_secure;
            }
            Message::ToggleTextInputIcon(show_icon) => {
                self.input_is_showing_icon = show_icon;
            }
            Message::DebugToggled(value) => {
                self.debug = value;
            }
            Message::TogglerChanged(toggler) => {
                self.toggler = toggler;
            }
            Message::OpenTrunk => {}
        }
    }

    pub fn view(&self) -> Element<Message> {
        let controls = row()
            .push_maybe(
                self.screen
                    .previous()
                    .is_some()
                    .then(|| padded_button("Back").on_press(Message::BackPressed)),
            )
            .push(horizontal_space())
            .push_maybe(
                self.can_continue()
                    .then(|| padded_button("Next").on_press(Message::NextPressed)),
            );

        let screen = match self.screen {
            Screen::Welcome => self.welcome(),
            Screen::Radio => self.radio(),
            Screen::Toggler => self.toggler(),
            Screen::Slider => self.slider(),
            Screen::Text => self.text(),
            Screen::Image => self.image(),
            Screen::RowsAndColumns => self.rows_and_columns(),
            Screen::Scrollable => self.scrollable(),
            Screen::TextInput => self.text_input(),
            Screen::Debugger => self.debugger(),
            Screen::End => self.end(),
        };

        let content = Element::from(
            column()
                .push(screen)
                .push(controls)
                .max_width(540)
                .spacing(20),
        );

        let scrollable = scrollable(
            container(if self.debug {
                content.explain(Color::BLACK.into())
            } else {
                content
            })
            .center_x(Length::Fill),
        )
        .spacing(10);

        container(scrollable)
            .center_y(Length::Fill)
            .padding(10)
            .into()
    }

    fn can_continue(&self) -> bool {
        match self.screen {
            Screen::Welcome => true,
            Screen::Radio => self.language == Some(Language::Rust),
            Screen::Toggler => self.toggler,
            Screen::Slider => true,
            Screen::Text => true,
            Screen::Image => true,
            Screen::RowsAndColumns => true,
            Screen::Scrollable => true,
            Screen::TextInput => !self.input_value.is_empty(),
            Screen::Debugger => true,
            Screen::End => false,
        }
    }

    fn welcome(&self) -> Column<Message> {
        Self::container("Welcome!")
            .push(
                "This is a simple tour meant to showcase a bunch of \
                widgets that can be easily implemented on top of Iced.",
            )
            .push(
                "Iced is a cross-platform GUI library for Rust focused on \
                 simplicity and type-safety. It is heavily inspired by Elm.",
            )
            .push(
                "It was originally born as part of Coffee, an opinionated \
                 2D game engine for Rust.",
            )
            .push(
                "On native platforms, Iced provides by default a renderer \
                 built on top of wgpu, a graphics library supporting Vulkan, \
                 Metal, DX11, and DX12.",
            )
            .push({
                ""
                // let theme = Theme::default();
                // let palette = theme.extended_palette();

                // rich_text![
                //     "Additionally, this tour can also run on WebAssembly ",
                //     "by leveraging ",
                //     span("trunk")
                //         .color(palette.primary.base.color)
                //         .background(palette.background.weakest.color)
                //         .border(
                //             border::rounded(2)
                //                 .width(1)
                //                 .color(palette.background.weak.color)
                //         )
                //         .padding([0, 2])
                //         .font(Font::MONOSPACE)
                //         .link(Message::OpenTrunk),
                //     "."
                // ]
                // .on_link_click(std::convert::identity)
            })
            .push(
                "You will need to interact with the UI in order to reach \
                 the end!",
            )
    }

    fn slider(&self) -> Column<Message> {
        Self::container("Slider")
            .push(
                "A slider allows you to smoothly select a value from a range \
                 of values.",
            )
            .push(
                "The following slider lets you choose an integer from \
                 0 to 100:",
            )
            .push(slider(0.0..=100.0, self.slider, Message::SliderChanged))
            .push(
                text(self.slider.to_string())
                    .width(Length::Fill)
                    .align_x(Alignment::Center),
            )
    }

    fn rows_and_columns(&self) -> Column<Message> {
        fn map_layout(layout: u64) -> Message {
            Message::LayoutChanged(Layout::from(layout))
        }

        let row_radio = radio(
            "Row",
            Layout::Row as u64,
            Some(self.layout as u64),
            map_layout,
        );

        let column_radio = radio(
            "Column",
            Layout::Column as u64,
            Some(self.layout as u64),
            map_layout,
        );

        let layout_section: Element<_> = match self.layout {
            Layout::Row => row().push(row_radio).push(column_radio).into(),
            Layout::Column => column()
                .push(row_radio)
                .push(column_radio)
                .spacing(self.spacing)
                .into(),
        };

        let spacing_section = column()
            .push(slider(0.0..=80.0, self.spacing, Message::SpacingChanged))
            .push(
                text(format!("{} px", self.spacing))
                    .width(Length::Fill)
                    .align_x(Alignment::Center),
            )
            .spacing(10);

        Self::container("Rows and columns")
            .spacing(self.spacing)
            .push(
                "Iced uses a layout model based on flexbox to position UI \
                 elements.",
            )
            .push(
                "Rows and columns can be used to distribute content \
                 horizontally or vertically, respectively.",
            )
            .push(layout_section)
            .push("You can also easily change the spacing between elements:")
            .push(spacing_section)
    }

    fn text(&self) -> Column<Message> {
        let size = self.text_size;
        let color = self.text_color;

        let size_section = column()
            .push("You can change its size")
            .push(text(format!("This text is {size} pixels")).size(size))
            .push(slider(10.0..=70.0, size, Message::TextSizeChanged))
            .padding(20)
            .spacing(20);

        let color_sliders = row()
            .push(color_sliders(color.r, move |r| Color { r, ..color }))
            .push(color_sliders(color.g, move |g| Color { g, ..color }))
            .push(color_sliders(color.b, move |b| Color { b, ..color }))
            .spacing(10);

        let color_section = column()
            .push("And its color:")
            .push(text(format!("{color:?}")).color(color))
            .push(color_sliders)
            .padding(20)
            .spacing(20);

        Self::container("Text")
            .push(
                "Text is probably the most essential widget for your UI. \
                 It will try to adapt to the dimensions of its container.",
            )
            .push(size_section)
            .push(color_section)
    }

    fn radio(&self) -> Column<Message> {
        let mut question = column();
        question = question.push(text("Iced is written in...").size(24)).push(
            Column::with_children(
                Language::all()
                    .iter()
                    .copied()
                    .map(|language| {
                        radio(
                            language,
                            language as u64,
                            self.language.map(|l| l as u64),
                            |v| Message::LanguageSelected(Language::from(v)),
                        )
                    })
                    .map(Element::from),
            )
            .spacing(10),
        );
        let question = question.padding(20).spacing(10);

        Self::container("Radio button")
            .push(
                "A radio button is normally used to represent a choice... \
                 Surprise test!",
            )
            .push(question)
            .push(
                "Iced works very well with iterators! The list above is \
                 basically created by folding a column over the different \
                 choices, creating a radio button for each one of them!",
            )
    }

    fn toggler(&self) -> Column<Message> {
        Self::container("Toggler")
            .push("A toggler is mostly used to enable or disable something.")
            .push(
                Container::new(
                    toggler(self.toggler)
                        .label("Toggle me to continue...")
                        .on_toggle(Message::TogglerChanged),
                )
                .padding([0, 40]),
            )
    }

    fn image(&self) -> Column<Message> {
        let width = self.image_width;
        let filter_method = self.image_filter_method;

        Self::container("Image")
            .push("An image that tries to keep its aspect ratio.")
            .push(ferris(width, filter_method))
            .push(slider(100.0..=500.0, width, Message::ImageWidthChanged))
            .push(
                text(format!("Width: {width} px"))
                    .width(Length::Fill)
                    .align_x(Alignment::Center),
            )
            .push(
                checkbox(
                    "Use nearest interpolation",
                    filter_method == FilterMethod::Nearest,
                )
                .on_toggle(Message::ImageUseNearestToggled),
            )
            .align_x(Horizontal::Center)
    }

    fn scrollable(&self) -> Column<Message> {
        Self::container("Scrollable")
            .push(
                "Iced supports scrollable content. Try it out! Find the \
                 button further below.",
            )
            .push(text("Tip: You can use the scrollbar to scroll down faster!").size(16))
            .push(vertical_space().height(4096))
            .push(
                text("You are halfway there!")
                    .width(Length::Fill)
                    .size(30)
                    .align_x(Alignment::Center),
            )
            .push(vertical_space().height(4096))
            .push(ferris(300.0, FilterMethod::Linear))
            .push(
                text("You made it!")
                    .width(Length::Fill)
                    .size(50)
                    .align_x(Alignment::Center),
            )
    }

    fn text_input(&self) -> Column<Message> {
        let value = &self.input_value;
        let is_secure = self.input_is_secure;
        let is_showing_icon = self.input_is_showing_icon;

        let text_input = text_input("Type something to continue...", value)
            .on_input(Message::InputChanged)
            .padding(10)
            .size(30);

        // if is_showing_icon {
        //     text_input = text_input.icon(text_input::Icon {
        //         font: Font::default(),
        //         code_point: '🚀',
        //         size: Some(Pixels(28.0)),
        //         spacing: 10.0,
        //         side: text_input::Side::Right,
        //     });
        // }

        Self::container("Text input")
            .push("Use a text input to ask for different kinds of information.")
            .push(text_input.secure(is_secure))
            .push(checkbox("Enable password mode", is_secure).on_toggle(Message::ToggleSecureInput))
            .push(checkbox("Show icon", is_showing_icon).on_toggle(Message::ToggleTextInputIcon))
            .push(
                "A text input produces a message every time it changes. It is \
                 very easy to keep track of its contents:",
            )
            .push(
                text(if value.is_empty() {
                    "You have not typed anything yet..."
                } else {
                    value
                })
                .width(Length::Fill)
                .align_x(Alignment::Center),
            )
    }

    fn debugger(&self) -> Column<Message> {
        Self::container("Debugger")
            .push(
                "You can ask Iced to visually explain the layouting of the \
                 different elements comprising your UI!",
            )
            .push(
                "Give it a shot! Check the following checkbox to be able to \
                 see element boundaries.",
            )
            .push(checkbox("Explain layout", self.debug).on_toggle(Message::DebugToggled))
            .push("Feel free to go back and take a look.")
    }

    fn end(&self) -> Column<Message> {
        Self::container("You reached the end!")
            .push("This tour will be updated as more features are added.")
            .push("Make sure to keep an eye on it!")
    }

    fn container(title: &str) -> Column<Message> {
        column().push(text(title).size(50)).spacing(20)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    Welcome,
    Slider,
    RowsAndColumns,
    Text,
    Radio,
    Toggler,
    Image,
    Scrollable,
    TextInput,
    Debugger,
    End,
}

impl Screen {
    const ALL: &'static [Self] = &[
        Self::Welcome,
        Self::Slider,
        Self::RowsAndColumns,
        Self::Text,
        Self::Radio,
        Self::Toggler,
        Self::Image,
        Self::Scrollable,
        Self::TextInput,
        Self::Debugger,
        Self::End,
    ];

    pub fn next(self) -> Option<Screen> {
        Self::ALL
            .get(
                Self::ALL
                    .iter()
                    .copied()
                    .position(|screen| screen == self)
                    .expect("Screen must exist")
                    + 1,
            )
            .copied()
    }

    pub fn previous(self) -> Option<Screen> {
        let position = Self::ALL
            .iter()
            .copied()
            .position(|screen| screen == self)
            .expect("Screen must exist");

        if position > 0 {
            Some(Self::ALL[position - 1])
        } else {
            None
        }
    }
}

fn ferris<'a>(width: f32, filter_method: FilterMethod) -> Container<Message> {
    container(
        // This should go away once we unify resource loading on native
        // platforms
        // image("tour/images/ferris.png")
        //     .filter_method(filter_method)
        //     .width(width),
        "",
    )
    .center_x(Length::Fill)
}

fn padded_button(label: &str) -> Button<Message> {
    button(text(label))
}

fn color_sliders<'a>(
    component: f32,
    update: impl Fn(f32) -> Color + Send + Sync + 'static,
) -> Slider<Message> {
    slider(0.0..=1.0, component, move |c| {
        Message::TextColorChanged(update(c))
    })
    .step(0.01)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum Language {
    Rust,
    Elm,
    Ruby,
    Haskell,
    C,
    Other,
}

impl Language {
    fn all() -> [Language; 6] {
        [
            Language::C,
            Language::Elm,
            Language::Ruby,
            Language::Haskell,
            Language::Rust,
            Language::Other,
        ]
    }
}

impl From<u64> for Language {
    fn from(value: u64) -> Self {
        match value {
            0 => Language::Rust,
            1 => Language::Elm,
            2 => Language::Ruby,
            3 => Language::Haskell,
            4 => Language::C,
            _ => Language::Other,
        }
    }
}

impl From<Language> for String {
    fn from(language: Language) -> String {
        String::from(match language {
            Language::Rust => "Rust",
            Language::Elm => "Elm",
            Language::Ruby => "Ruby",
            Language::Haskell => "Haskell",
            Language::C => "C",
            Language::Other => "Other",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum Layout {
    Row,
    Column,
}

impl From<u64> for Layout {
    fn from(value: u64) -> Self {
        match value {
            0 => Layout::Row,
            1 => Layout::Column,
            _ => Layout::Row,
        }
    }
}

impl Default for Tour {
    fn default() -> Self {
        Self {
            screen: Screen::TextInput,
            slider: 50.0,
            layout: Layout::Row,
            spacing: 20.0,
            text_size: 30.0,
            text_color: Color::BLACK,
            language: None,
            toggler: false,
            image_width: 300.0,
            image_filter_method: FilterMethod::Linear,
            input_value: String::new(),
            input_is_secure: false,
            input_is_showing_icon: false,
            debug: false,
        }
    }
}
