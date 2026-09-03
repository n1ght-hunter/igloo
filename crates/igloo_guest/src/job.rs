//! Owns lazy work, cancellation and at most one unacknowledged message per job.

use crate::{
    bindings::{
        self,
        exports::iced::app::app_instance::{Execution, GuestJob, JobError},
    },
    task::Factory,
};
use futures::{
    StreamExt,
    channel::oneshot,
    future::{AbortHandle, Abortable},
};
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};

struct Pending<M> {
    owner: u64,
    message: M,
    ack: oneshot::Sender<()>,
}

pub(crate) struct Registry<M> {
    next_message: Cell<u64>,
    next_owner: Cell<u64>,
    messages: RefCell<HashMap<u64, Pending<M>>>,
}

impl<M> Default for Registry<M> {
    fn default() -> Self {
        Self {
            next_message: Cell::new(0),
            next_owner: Cell::new(0),
            messages: RefCell::new(HashMap::new()),
        }
    }
}

fn fresh(counter: &Cell<u64>) -> u64 {
    let value = counter.get();
    counter.set(value.checked_add(1).expect("message identity exhausted"));
    value
}

impl<M> Registry<M> {
    pub fn take(&self, id: u64) -> Option<M> {
        self.messages.borrow_mut().remove(&id).map(|pending| {
            let _ = pending.ack.send(());
            pending.message
        })
    }
    fn clear(&self, owner: u64) {
        self.messages
            .borrow_mut()
            .retain(|_, message| message.owner != owner);
    }
}

/// Exported job resource; cancellation also reclaims undelivered messages.
pub struct JobResource<M: 'static> {
    make: RefCell<Option<Factory<M>>>,
    registry: Rc<Registry<M>>,
    owner: u64,
    abort: RefCell<Option<AbortHandle>>,
    cancelled: Cell<bool>,
}

impl<M> std::fmt::Debug for JobResource<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JobResource")
            .field("owner", &self.owner)
            .field("cancelled", &self.cancelled.get())
            .finish()
    }
}

impl<M: 'static> JobResource<M> {
    pub(crate) fn new(make: Factory<M>, registry: Rc<Registry<M>>) -> Self {
        let owner = fresh(&registry.next_owner);
        Self {
            make: RefCell::new(Some(make)),
            registry,
            owner,
            abort: RefCell::new(None),
            cancelled: Cell::new(false),
        }
    }
    fn cancel_work(&self) {
        self.cancelled.set(true);
        self.make.borrow_mut().take();
        if let Some(abort) = self.abort.borrow_mut().take() {
            abort.abort();
        }
        self.registry.clear(self.owner);
    }
}

impl<M: 'static> GuestJob for JobResource<M> {
    async fn start(&self) -> Result<Execution, JobError> {
        if self.cancelled.get() {
            return Err(JobError::Cancelled);
        }
        let make = self
            .make
            .borrow_mut()
            .take()
            .ok_or(JobError::AlreadyStarted)?;
        let (mut messages, reader) = bindings::wit_stream::new::<u64>();
        let (completion, finished) =
            bindings::wit_future::new::<Result<(), JobError>>(|| Err(JobError::Cancelled));
        let (abort, registration) = AbortHandle::new_pair();
        self.abort.replace(Some(abort));
        let registry = self.registry.clone();
        let owner = self.owner;
        wit_bindgen::spawn_local(async move {
            let work = async {
                let mut stream = make();
                while let Some(message) = stream.next().await {
                    let message = message?;
                    let id = fresh(&registry.next_message);
                    let (ack, delivered) = oneshot::channel();
                    registry.messages.borrow_mut().insert(
                        id,
                        Pending {
                            owner,
                            message,
                            ack,
                        },
                    );
                    if messages.write_one(id).await.is_some() {
                        return Err(JobError::Cancelled);
                    }
                    delivered.await.map_err(|_| JobError::Cancelled)?;
                }
                Ok(())
            };
            let result = Abortable::new(work, registration)
                .await
                .unwrap_or(Err(JobError::Cancelled));
            registry.clear(owner);
            drop(messages);
            let _ = completion.write(result).await;
        });
        Ok(Execution {
            messages: reader,
            completion: finished,
        })
    }
    fn cancel(&self) {
        self.cancel_work();
    }
}

impl<M: 'static> Drop for JobResource<M> {
    fn drop(&mut self) {
        self.cancel_work();
    }
}
