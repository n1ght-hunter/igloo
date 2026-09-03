use iced::Task;
use igloo_native::{NativePluginManager, Permissions, plugin_manager::Message};

fn main() -> iced::Result {
    iced::application(State::new, State::update, State::view)
        .subscription(State::subscription)
        .run()
}
struct State {
    plugins: NativePluginManager,
}
impl State {
    fn new() -> (Self, Task<Message>) {
        let mut plugins = NativePluginManager::new();
        let path = std::env::current_exe()
            .expect("host executable path")
            .parent()
            .expect("host executable directory")
            .parent()
            .expect("target profile directory")
            .join(libloading::library_filename("native_counter"));
        let task = plugins
            .load("counter", path, Permissions::empty())
            .expect("failed to load native_counter plugin");
        (Self { plugins }, task)
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        self.plugins.update(message)
    }
    fn subscription(&self) -> iced::Subscription<Message> {
        self.plugins.subscriptions()
    }
    fn view(&self) -> iced::Element<'_, Message> {
        self.plugins.view("counter").expect("counter plugin loaded")
    }
}
