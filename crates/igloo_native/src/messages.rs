/// The message type exchanged with the host.
///
/// An owned, type-erased pointer to one plugin-defined value. Only the plugin
/// ever creates one (via [`Message::new`]) or reads one (via
/// [`Message::from_raw`]); the host only moves it around and hands it back.
/// `drop` is monomorphized in the plugin, so the payload's destructor and its
/// allocator both stay on the plugin's side of the boundary.
#[stabby::stabby]
#[repr(C)]
pub struct Message {
    ptr: *mut core::ffi::c_void,
    drop: extern "C" fn(*mut core::ffi::c_void),
}

/// The pointer is uniquely owned and never aliased; `T: Send` is enforced by
/// [`Message::new`].
#[allow(unsafe_code)]
unsafe impl Send for Message {}

impl Message {
    /// Boxes a plugin message for the trip across the FFI boundary.
    ///
    /// `T: Send` because the host may poll the task that carries this message
    /// on an executor thread; `Message` itself is unconditionally `Send`, so
    /// the bound has to be enforced here.
    pub fn new<T: Send + 'static>(message: T) -> Self {
        extern "C" fn dropit<T>(ptr: *mut core::ffi::c_void) {
            #[allow(unsafe_code)]
            unsafe {
                drop(Box::from_raw(ptr.cast::<T>()));
            }
        }
        Self {
            ptr: Box::into_raw(Box::new(message)).cast::<core::ffi::c_void>(),
            drop: dropit::<T>,
        }
    }

    /// Recovers the payload boxed by [`Message::new`].
    ///
    /// # Safety
    /// `T` must be exactly the type that was passed to [`Message::new`].
    #[allow(unsafe_code)]
    pub unsafe fn from_raw<T>(self) -> Box<T> {
        // `ManuallyDrop` so `Drop` does not also free `ptr` after the move.
        let this = core::mem::ManuallyDrop::new(self);
        unsafe { Box::from_raw(this.ptr.cast::<T>()) }
    }
}

impl Drop for Message {
    fn drop(&mut self) {
        (self.drop)(self.ptr);
    }
}

impl core::fmt::Debug for Message {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Message").finish_non_exhaustive()
    }
}

/// A plugin-supplied recipe for producing a [`Message`].
///
/// Callable any number of times, it can sit in a widget tree and mint a fresh
/// [`Message`] on every interaction, which is what a raw `Message` (uniquely
/// owned, not `Clone`) cannot do.
pub type Emit = stabby::dynptr!(stabby::boxed::Box<dyn stabby::closure::Call0<Message>>);

type EmitUnit<Arg = ()> =
    stabby::dynptr!(stabby::boxed::Box<dyn stabby::closure::Call1<Message, Arg>>);

/// A plugin-supplied recipe for producing a [`Message`] from a `bool`.
pub type EmitBool = EmitUnit<bool>;

/// A plugin-supplied recipe for producing a [`Message`] from an `f32`.
pub type EmitF32 = EmitUnit<f32>;

/// A plugin-supplied recipe for producing a [`Message`] from a `u64`.
pub type EmitU64 = EmitUnit<u64>;

/// A plugin-supplied recipe for producing a [`Message`] from a `String`.
pub type EmitString = EmitUnit<stabby::string::String>;

/// A plugin-supplied recipe for producing a [`Message`] from a [`crate::widgets::Viewport`].
pub type EmitViewport = EmitUnit<crate::widgets::Viewport>;

/// An asynchronous callback whose captures can cross executor threads.
pub type SendEmit = stabby::dynptr!(stabby::boxed::Box<dyn stabby::closure::Call0<Message> + Send>);
