use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, ThreadId},
};

use crate::RequestHandle;

type ResolveFn<Output> = Box<dyn FnMut(&mut RequestHandle<Output>, Output) + Send>;

pub struct EffectResolver<Output: Send + 'static> {
    handle: RequestHandle<Output>,
    resolve_fn: ResolveFn<Output>,
    /// `true` while `try_process_effect` is executing on the call stack.
    active: Arc<AtomicBool>,
    /// The thread that called `try_process_effect`.
    calling_thread: ThreadId,
}

impl<Output: Send + 'static> EffectResolver<Output> {
    /// Resolve the effect with the given output.
    ///
    /// For one-shot effects this should be called exactly once. For streaming
    /// effects it can be called multiple times.
    ///
    /// # Panics
    ///
    /// Panics if called synchronously from within
    /// [`EffectMiddleware::try_process_effect`]. Middleware must dispatch work
    /// asynchronously (e.g. `std::thread::spawn`, `spawn_local`, or a channel)
    /// and call `resolve` from there.
    ///
    /// See <https://github.com/redbadger/crux/issues/492>
    pub fn resolve(&mut self, output: Output) {
        assert!(
            !(self.active.load(Ordering::Acquire) && thread::current().id() == self.calling_thread),
            "EffectMiddleware::try_process_effect must not call resolve() synchronously. \
             Dispatch work asynchronously (thread, spawn_local, channel, etc.). \
             See https://github.com/redbadger/crux/issues/492"
        );
        (self.resolve_fn)(&mut self.handle, output);
    }
}
