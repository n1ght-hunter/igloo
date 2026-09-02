use wasmtime::component::bindgen;

bindgen!({
    path: "wit",
    with: {
        "iced:app/task.task": crate::task::HostTask,
    },
});
