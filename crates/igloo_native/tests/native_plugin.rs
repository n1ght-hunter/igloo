//! Headless integration with the separately built counter library.
#![cfg(feature = "host")]
use futures::StreamExt;
use igloo_native::{NativePluginManager, Permissions};

#[test]
#[ignore = "build native_counter first, then run this test with --ignored"]
fn counter_library_boots_renders_streams_and_survives_removal() {
    let executable = std::env::current_exe().unwrap();
    let path = executable
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join(libloading::library_filename("native_counter"));
    let mut plugins = NativePluginManager::new();
    drop(plugins.load("counter", path, Permissions::empty()).unwrap());
    assert_eq!(plugins.ids().collect::<Vec<_>>(), ["counter"]);
    drop(plugins.view("counter").unwrap());
    let recipe = iced::advanced::subscription::into_recipes(plugins.subscriptions())
        .pop()
        .unwrap();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap();
    let mut stream = {
        let _entered = runtime.enter();
        recipe.stream(futures::stream::empty().boxed())
    };
    let message = runtime.block_on(async {
        tokio::time::timeout(std::time::Duration::from_secs(3), stream.next())
            .await
            .unwrap()
            .unwrap()
    });
    drop(plugins.update(message));
    drop(plugins.view("counter").unwrap());
    assert!(plugins.remove("counter"));
    assert!(plugins.ids().next().is_none());
    drop(plugins);
    // A retained stream still contains plugin code, even after its application is removed.
    let message = runtime.block_on(async {
        tokio::time::timeout(std::time::Duration::from_secs(3), stream.next())
            .await
            .unwrap()
            .unwrap()
    });
    drop(message);
    drop(stream);
}
