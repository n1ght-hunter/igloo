use anyhow::Result;
use iced::{
    Task,
    widget::{self, Row, button, text},
};
use igloo::plugin_manager::PluginManager;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    iced::application(IcedApp::new, IcedApp::update, IcedApp::view).run()?;
    Ok(())
}

struct IcedApp {
    plugin_manager: PluginManager,
    current_page: String,
}

#[derive(Debug, Clone)]
enum Message {
    PluginMessage(String, igloo::Message),
    ChangePage(String),
}

impl IcedApp {
    pub fn new() -> Self {
        let mut plugin_manager = PluginManager::new().unwrap();

        plugin_manager
            .add_plugin_from_file("rust-plugin", "../../target/wasm32-wasip2/release/rust_guest.wasm")
            .unwrap();
        // plugin_manager
        //     .add_plugin_from_file("js-plugin", "plugins/js/js-app.wasm")
        //     .unwrap();

        Self {
            plugin_manager,
            current_page: "Home".to_string(),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PluginMessage(id, msg) => {
                if let Err(e) = self.plugin_manager.plugin_update(&id, msg) {
                    tracing::error!("Failed to update plugin {}: {}", id, e);
                }
            }
            Message::ChangePage(page) => {
                self.current_page = page;
            }
        };
        Task::none()
    }

    fn view(&self) -> iced::Element<'_, Message> {
        let mut pages: Vec<String> = vec!["Home".to_string()];

        self.plugin_manager
            .ids()
            .into_iter()
            .for_each(|id| pages.push(id));

        let pages = Row::from_iter(pages.into_iter().map(|page| {
            iced::Element::from(button(text(page.clone())).on_press(Message::ChangePage(page)))
        }));

        let page: iced::Element<'_, Message> = match self.current_page.as_str() {
            "Home" => iced::widget::Text::new("Home").into(),
            id => self
                .plugin_manager
                .plugin_view(id)
                .map(|e| e.map(|m| Message::PluginMessage(id.to_string(), m)))
                .unwrap_or_else(|| iced::widget::Text::new("Unknown Plugin").into()),
        };

        widget::Column::new().push(pages).push(page).into()
    }
}
