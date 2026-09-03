use wasmtime::component::bindgen;

bindgen!({
    path: "wit",
    // The store lives on a dedicated worker thread driving one long-lived
    // `run_concurrent`, so guest exports are invoked through an `Accessor`
    // rather than `&mut Store`. `async | store` makes every `call_*` use
    // `TypedFunc::call_concurrent`, which also drives sync-lifted exports.
    exports: { default: async | store },
    imports: { default: trappable, "iced:app/capabilities": store | trappable },
});
