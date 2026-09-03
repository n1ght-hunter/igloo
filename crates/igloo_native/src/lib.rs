#[cfg(feature = "host")]
use abi::{ApplicationDyn, ApplicationDynMut};
#[cfg(feature = "host")]
use stabby::boxed::Box;

#[cfg(feature = "host")]
pub mod plugin_manager;
pub mod subscription;
pub mod task;
pub mod time;
#[doc(hidden)]
pub mod wire_task;
#[cfg(feature = "host")]
pub use plugin_manager::NativePluginManager;
pub mod helpers;
pub mod messages;
pub mod widgets;

pub use messages::Emit;
pub use subscription::Subscription;
pub use task::Task;
pub use widgets::Element;
pub use widgets::NodeTree;
#[cfg(feature = "host")]
pub use wire_task::Permissions;

/// Opaque handle to a plugin's state. The host never looks inside it; it receives
/// one from `boot` and hands it back to [`Application::update`] / `view`.
#[cfg(feature = "host")]
type BoxedApplication = stabby::dynptr!(Box<dyn abi::Application>);
#[cfg(feature = "host")]
type Boot = extern "C" fn() -> stabby::abi::Tuple<BoxedApplication, wire_task::Task>;

/// A typed native plugin, implemented without ABI message handling.
pub trait Application: 'static {
    type Message: Send + 'static;
    fn new() -> (Self, Task<Self::Message>)
    where
        Self: Sized;
    fn update(&mut self, message: Self::Message) -> Task<Self::Message>;
    fn view(&self) -> Element<Self::Message>;
    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }
}

/// Native wire types; plugin authors normally use the typed application API.
pub mod abi {
    pub use crate::messages::Message;
    pub use crate::subscription::Sources;
    pub use crate::wire_task::{Action, Step, Task};
    /// The ABI-stable application implemented by the export adapter.
    #[stabby::stabby]
    pub trait Application {
        /// Delivers a message created by this application's exported callbacks.
        ///
        /// # Safety
        /// The payload must have this application's associated message type.
        #[allow(unsafe_code)]
        unsafe extern "C" fn update(&mut self, message: Message) -> Task;
        extern "C" fn view(&self) -> crate::NodeTree;
        extern "C" fn subscription(&self) -> Sources;
    }
    pub type BoxedApplication = stabby::dynptr!(stabby::boxed::Box<dyn Application>);
}

#[doc(hidden)]
pub struct Adapter<A>(pub A);
impl<A> std::fmt::Debug for Adapter<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Adapter").finish_non_exhaustive()
    }
}
impl<A: Application> abi::Application for Adapter<A> {
    #[allow(unsafe_code)]
    unsafe extern "C" fn update(&mut self, message: abi::Message) -> abi::Task {
        // Only this adapter creates messages for A, so the payload type is known.
        #[allow(unsafe_code)]
        let message = unsafe { message.from_raw::<A::Message>() };
        self.0.update(*message).erase()
    }
    extern "C" fn view(&self) -> NodeTree {
        self.0.view().into_tree()
    }
    extern "C" fn subscription(&self) -> subscription::Sources {
        self.0.subscription().erase()
    }
}

#[doc(hidden)]
pub fn boot<A: Application>() -> stabby::abi::Tuple<abi::BoxedApplication, abi::Task> {
    let (app, task) = A::new();
    stabby::abi::Tuple(stabby::boxed::Box::new(Adapter(app)).into(), task.erase())
}

#[doc(hidden)]
pub use stabby;
/// Exports a typed application as a native plugin.
#[macro_export]
macro_rules! export {
    ($app:ty) => {
        #[$crate::stabby::export]
        pub extern "C" fn boot()
        -> $crate::stabby::abi::Tuple<$crate::abi::BoxedApplication, $crate::abi::Task> {
            $crate::boot::<$app>()
        }
    };
}

/// A dynamically loaded native plugin.
///
/// Every [`NodeTree`] and [`abi::Message`] the plugin produces has heap buffers
/// allocated by the library and drop glue that lives in it, so they must all be
/// dropped normally. Libraries stay mapped for the process lifetime, protecting
/// callbacks and executor wakers retained after plugin removal.
#[cfg(feature = "host")]
#[allow(missing_debug_implementations)]
pub struct NativePlugin {
    application: BoxedApplication,
    /// Kept alive so the plugin's code stays mapped; never read directly.
    _lib: &'static libloading::Library,
    instance: u64,
    prepared: widgets::PreparedView,
}

/// Everything that can go wrong loading a native plugin.
#[cfg(feature = "host")]
#[derive(Debug, thiserror::Error)]
pub enum NativePluginError {
    /// The library at the given path could not be opened.
    #[error("failed to open plugin library: {0}")]
    Open(#[source] libloading::Error),
    /// The library does not export a `boot` symbol with the expected ABI.
    #[error("plugin library has no valid `boot` export: {0}")]
    MissingBoot(#[source] std::boxed::Box<dyn std::error::Error + Send + Sync>),
}

#[cfg(feature = "host")]
impl NativePlugin {
    /// Loads the plugin at `path`, boots it, and returns its initial task mapped
    /// through `permissions`.
    pub fn load(
        path: impl AsRef<std::path::Path>,
        permissions: Permissions,
    ) -> Result<(Self, iced::Task<abi::Message>), NativePluginError> {
        use stabby::libloading::StabbyLibrary;
        #[allow(unsafe_code)]
        let (boot, lib) = unsafe {
            let lib = libloading::Library::new(path.as_ref().to_path_buf())
                .map_err(NativePluginError::Open)?;
            let boot = *lib
                .get_stabbied::<Boot>(b"boot")
                .map_err(NativePluginError::MissingBoot)?;
            (boot, lib)
        };

        let stabby::abi::Tuple(application, task) = boot();
        let prepared = widgets::PreparedView::new(&application.view());
        Ok((
            Self {
                application,
                // Streams and retained wakers may outlive the plugin instance.
                _lib: std::boxed::Box::leak(std::boxed::Box::new(lib)),
                instance: next_instance(),
                prepared,
            },
            task.into_iced(permissions),
        ))
    }

    /// Delivers `message` to the plugin and maps the resulting task through
    /// `permissions`.
    #[allow(unsafe_code)]
    pub(crate) unsafe fn update(
        &mut self,
        message: abi::Message,
        permissions: Permissions,
    ) -> iced::Task<abi::Message> {
        // The manager verifies the originating instance before forwarding this payload.
        let task = unsafe { self.application.update(message) }.into_iced(permissions);
        self.prepared.refresh(&self.application.view());
        task
    }

    /// The plugin's current UI tree.
    pub fn view(&self) -> iced::Element<'_, abi::Message> {
        self.prepared.render(self.application.view())
    }
    pub fn subscription(&self) -> iced::Subscription<abi::Message> {
        self.application.subscription().into_iced(self.instance)
    }
}

#[cfg(feature = "host")]
fn next_instance() -> u64 {
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    NEXT.try_update(
        std::sync::atomic::Ordering::Relaxed,
        std::sync::atomic::Ordering::Relaxed,
        |id| id.checked_add(1),
    )
    .expect("native plugin IDs exhausted")
}

#[cfg(all(test, feature = "host"))]
mod tests {
    use super::*;
    #[test]
    fn loading_reports_missing_library() {
        let path = std::env::temp_dir().join("igloo-native-nonexistent-plugin.invalid");
        assert!(matches!(
            NativePlugin::load(path, Permissions::empty()),
            Err(NativePluginError::Open(_))
        ));
    }
    #[cfg(target_os = "windows")]
    #[test]
    fn loading_reports_missing_boot_export() {
        assert!(matches!(
            NativePlugin::load("kernel32.dll", Permissions::empty()),
            Err(NativePluginError::MissingBoot(_))
        ));
    }
}
