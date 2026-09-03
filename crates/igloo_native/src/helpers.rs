use stabby::{abi::Option as Opt, boxed::Box, future::DynFutureUnsync};

use crate::messages::Message;

/// `futures::Stream<Item = Message>`, made ABI-stable for the plugin boundary.
#[stabby::stabby]
pub trait Stream {
    /// Resolves to the next message, or `None` once the stream is exhausted.
    extern "C" fn next<'a>(&'a mut self) -> DynFutureUnsync<'a, Opt<Message>>;
}

/// A boxed [`Stream`]; the ABI-stable analogue of `iced::futures::BoxStream`.
pub type BoxStream = stabby::dynptr!(Box<dyn Stream + Send>);
