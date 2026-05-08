//! Bounded back-pressure queue for `JournalEvent` batching (TASK-031).
//!
//! Pattern:
//!
//!   subscriber thread → EventQueue::push → indexer task → Index::apply
//!
//! The queue is a `Mutex<State>` rather than `tokio::mpsc` because the
//! indexer side is `&self`-shaped (the Build Guide's public API), and a
//! single dedicated tokio task is what does the draining. When a
//! subscriber outpaces the indexer, `try_push` returns `QueueFull` —
//! callers can either back off or block (`push_blocking`), the latter
//! being the contract Phases 1–3 promised when they refused to drop
//! events.
//!
//! Concurrency invariant: `closed` lives **inside** the same mutex as
//! the queue itself, so a concurrent `close()` cannot land between a
//! waiter's "is the queue empty?" check and the underlying
//! `Condvar::wait`. Earlier drafts kept `closed` in a sibling mutex,
//! which leaked the wake-up race into a possible-forever sleep on
//! `wait_for_events` / `push_blocking` (Phase-4 review #9 + #10).

use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Duration;

use parking_lot::{Condvar, Mutex};
use sourcerer_journal::JournalEvent;

use crate::error::IndexError;

/// Default capacity called out in the Build Guide.
pub const DEFAULT_CAPACITY: usize = 10_000;

#[derive(Clone)]
pub struct EventQueue {
    inner: Arc<Inner>,
}

struct Inner {
    capacity: usize,
    state: Mutex<State>,
    not_full: Condvar,
    not_empty: Condvar,
}

struct State {
    queue: VecDeque<JournalEvent>,
    closed: bool,
}

impl EventQueue {
    pub fn new(capacity: usize) -> Self {
        let capacity = capacity.max(1);
        Self {
            inner: Arc::new(Inner {
                capacity,
                state: Mutex::new(State {
                    queue: VecDeque::with_capacity(capacity),
                    closed: false,
                }),
                not_full: Condvar::new(),
                not_empty: Condvar::new(),
            }),
        }
    }

    pub fn capacity(&self) -> usize {
        self.inner.capacity
    }

    pub fn len(&self) -> usize {
        self.inner.state.lock().queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_closed(&self) -> bool {
        self.inner.state.lock().closed
    }

    /// Non-blocking push. Returns `QueueFull` when the queue is at
    /// capacity — callers in Phases 1–3 surface this as a back-pressure
    /// signal to the journal subscriber. A closed queue refuses pushes
    /// the same way capacity-exhausted ones do.
    pub fn try_push(&self, ev: JournalEvent) -> Result<(), IndexError> {
        let mut state = self.inner.state.lock();
        if state.closed || state.queue.len() >= self.inner.capacity {
            return Err(IndexError::QueueFull {
                capacity: self.inner.capacity,
                pending: state.queue.len(),
            });
        }
        state.queue.push_back(ev);
        let was_empty = state.queue.len() == 1;
        drop(state);
        if was_empty {
            self.inner.not_empty.notify_one();
        }
        Ok(())
    }

    /// Blocking push — waits until the queue has room. Honors `close()`.
    /// Returns `QueueFull` if the queue closes while we're waiting so
    /// callers don't sleep forever on shutdown.
    pub fn push_blocking(&self, ev: JournalEvent) -> Result<(), IndexError> {
        let mut state = self.inner.state.lock();
        loop {
            if state.closed {
                return Err(IndexError::QueueFull {
                    capacity: self.inner.capacity,
                    pending: state.queue.len(),
                });
            }
            if state.queue.len() < self.inner.capacity {
                state.queue.push_back(ev);
                let was_empty = state.queue.len() == 1;
                drop(state);
                if was_empty {
                    self.inner.not_empty.notify_one();
                }
                return Ok(());
            }
            self.inner.not_full.wait(&mut state);
        }
    }

    /// Drain up to `max` events. Returns immediately with whatever is
    /// pending (possibly empty); the indexer task is expected to call
    /// `wait_for_events` first when the queue is empty.
    pub fn drain(&self, max: usize) -> Vec<JournalEvent> {
        let mut state = self.inner.state.lock();
        let n = state.queue.len().min(max);
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            if let Some(ev) = state.queue.pop_front() {
                out.push(ev);
            }
        }
        if !out.is_empty() {
            self.inner.not_full.notify_all();
        }
        out
    }

    /// Block (with timeout) until at least one event is available or
    /// the queue is closed. Returns `false` on timeout or close-while-
    /// empty.
    pub fn wait_for_events(&self, timeout: Duration) -> bool {
        let mut state = self.inner.state.lock();
        if !state.queue.is_empty() {
            return true;
        }
        if state.closed {
            return false;
        }
        let _ = self.inner.not_empty.wait_for(&mut state, timeout);
        !state.queue.is_empty()
    }

    /// Mark the queue closed and wake up every waiter. Producers see
    /// `QueueFull`; consumers see `wait_for_events` return `false` once
    /// drained.
    pub fn close(&self) {
        let mut state = self.inner.state.lock();
        state.closed = true;
        drop(state);
        self.inner.not_full.notify_all();
        self.inner.not_empty.notify_all();
    }
}
