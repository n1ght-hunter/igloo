use wasmtime::component::bindgen;

bindgen!({
    path: "../../wit",
    with: {
        "iced:app/shared/element": crate::widgets::Element,
    },
});
