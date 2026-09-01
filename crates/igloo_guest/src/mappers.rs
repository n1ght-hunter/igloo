//! The guest-owned callback arena.
//!
//! A `Callback<M>` holds either the plugin's own message value directly (for a
//! fixed callback like `on_press`) or a closure that turns a widget's runtime
//! value into one (for a value-carrying callback like `on_toggle`). Each is
//! stored in a [`Frame`] and handed to the host as a plain `u32` id — never as
//! a resource — so there is no handle to cross between the import and export
//! views of a WIT type.

use crate::bindings::iced::app::message_types::Viewport;

/// A guest-owned callback, keyed by the `u32` id it was assigned when pushed.
pub(crate) enum Callback<M> {
    /// `Option`, not a bare `M`: `Application::Message` has no `Clone` bound,
    /// so dispatching takes the value out rather than cloning it.
    Fixed(Option<M>),
    Bool(Box<dyn Fn(bool) -> M>),
    F32(Box<dyn Fn(f32) -> M>),
    F64(Box<dyn Fn(f64) -> M>),
    U64(Box<dyn Fn(u64) -> M>),
    Str(Box<dyn Fn(String) -> M>),
    Viewport(Box<dyn Fn(Viewport) -> M>),
}

/// One frame's worth of callbacks. `base` is the id of slot 0, so ids minted
/// by an earlier frame fall below it and miss cleanly on lookup instead of
/// resolving to the wrong callback.
pub(crate) struct Frame<M> {
    base: u32,
    slots: Vec<Callback<M>>,
}

impl<M> Frame<M> {
    /// Starts a new, empty frame whose ids continue on from `base`.
    pub(crate) fn new(base: u32) -> Self {
        Self {
            base,
            slots: Vec::new(),
        }
    }

    /// The id one past the last slot in this frame — the `base` for the frame
    /// that replaces it.
    pub(crate) fn next_base(&self) -> u32 {
        self.base + self.slots.len() as u32
    }

    fn push(&mut self, cb: Callback<M>) -> u32 {
        let id = self.next_base();
        self.slots.push(cb);
        id
    }

    /// Resolves `id` to a slot in this frame, if it falls within it.
    pub(crate) fn get_mut(&mut self, id: u32) -> Option<&mut Callback<M>> {
        let index = id.checked_sub(self.base)?;
        self.slots.get_mut(index as usize)
    }
}

impl<M> Frame<M> {
    pub(crate) fn push_fixed(&mut self, msg: M) -> u32 {
        self.push(Callback::Fixed(Some(msg)))
    }

    pub(crate) fn push_bool(&mut self, f: Box<dyn Fn(bool) -> M>) -> u32 {
        self.push(Callback::Bool(f))
    }

    pub(crate) fn push_f32(&mut self, f: Box<dyn Fn(f32) -> M>) -> u32 {
        self.push(Callback::F32(f))
    }

    pub(crate) fn push_f64(&mut self, f: Box<dyn Fn(f64) -> M>) -> u32 {
        self.push(Callback::F64(f))
    }

    pub(crate) fn push_u64(&mut self, f: Box<dyn Fn(u64) -> M>) -> u32 {
        self.push(Callback::U64(f))
    }

    pub(crate) fn push_string(&mut self, f: Box<dyn Fn(String) -> M>) -> u32 {
        self.push(Callback::Str(f))
    }

    pub(crate) fn push_viewport(&mut self, f: Box<dyn Fn(Viewport) -> M>) -> u32 {
        self.push(Callback::Viewport(f))
    }
}
