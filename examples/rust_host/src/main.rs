use anyhow::Result;
use iced::{
    Task,
    widget::{self, Row, button, text},
};
use igloo::plugin_manager::{CompiledPlugin, PluginManager, load_and_compile_plugin_async};
use std::path::PathBuf;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .init();

    iced::application(IcedApp::new, IcedApp::update, IcedApp::view).run()?;
    Ok(())
}

struct IcedApp {
    plugin_manager: PluginManager,
    current_page: String,
    plugins_loading: usize,
}

#[derive(Debug, Clone)]
enum Message {
    Plugin(String, igloo::Message),
    ChangePage(String),
    PluginLoaded(std::result::Result<CompiledPlugin, String>),
}

impl IcedApp {
    pub fn new() -> (Self, Task<Message>) {
        let plugin_manager = PluginManager::new().unwrap();
        let engine = plugin_manager.engine();

        let available_plugins = [
            (
                "rust-plugin",
                "../../target/wasm32-wasip2/release/rust_guest.wasm",
            ),
            ("js-plugin", "../../plugins/js/js-app.wasm"),
            ("python-plugin", "../../plugins/python/python-app.wasm"),
        ];

        let plugins_to_load: Vec<_> = available_plugins
            .into_iter()
            .filter_map(|(name, path)| {
                let path = PathBuf::from(path);
                if path.exists() {
                    tracing::info!("Plugin found, will load: {} from {}", name, path.display());
                    Some((name.to_string(), path))
                } else {
                    tracing::info!("Plugin not found, skipping: {}", path.display());
                    None
                }
            })
            .collect();

        let plugins_loading = plugins_to_load.len();

        // Create a task for each plugin that loads and compiles in parallel
        let tasks: Vec<_> = plugins_to_load
            .into_iter()
            .map(|(name, path)| {
                let engine = engine.clone();
                Task::future(async move {
                    Message::PluginLoaded(load_and_compile_plugin_async(engine, name, path).await)
                })
            })
            .collect();

        // Batch all tasks together
        let load_task = Task::batch(tasks);

        (
            Self {
                plugin_manager,
                current_page: "Home".to_string(),
                plugins_loading,
            },
            load_task,
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Plugin(id, msg) => {
                if let Err(e) = self.plugin_manager.plugin_update(&id, msg) {
                    tracing::error!("Failed to update plugin {}: {}", id, e);
                }
            }
            Message::ChangePage(page) => {
                self.current_page = page;
            }
            Message::PluginLoaded(result) => {
                self.plugins_loading = self.plugins_loading.saturating_sub(1);
                match result {
                    Ok(plugin) => {
                        tracing::info!("Plugin loaded successfully: {}", plugin.name);
                        if let Err(e) = self.plugin_manager.add_compiled_plugin(plugin) {
                            tracing::error!("Failed to add plugin: {}", e);
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to load plugin: {}", e);
                    }
                }
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
            "Home" => {
                if self.plugins_loading > 0 {
                    widget::Text::new(format!("Loading {} plugin(s)...", self.plugins_loading))
                        .into()
                } else {
                    widget::Text::new("Home").into()
                }
            }
            id => self
                .plugin_manager
                .plugin_view(id)
                .map(|e| e.map(|m| Message::Plugin(id.to_string(), m)))
                .unwrap_or_else(|| widget::Text::new("Unknown Plugin").into()),
        };

        widget::Column::new().push(pages).push(page).into()
    }
}
