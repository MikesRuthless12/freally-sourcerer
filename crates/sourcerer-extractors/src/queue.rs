//! Bounded extraction queue with priority + back-pressure (TASK-049).
//!
//! Pattern (mirrors `sourcerer-index::EventQueue`):
//!
//!   journal subscriber → indexd dispatch → ExtractionQueue::push →
//!     extractor worker → Pipeline::dispatch + Sandbox::execute →
//!     BlobStore::put
//!
//! Priority is "recently-touched first" (Build Guide). The internal
//! data structure is a `BinaryHeap` keyed on
//! `(mtime_ns, Reverse(seq))` so the most recent file wins, with
//! FIFO tie-breaking among same-mtime entries.
//!
//! Back-pressure surfaces as [`QueueError::Full`] from `try_push` /
//! [`QueueError::Closed`] when the daemon is shutting down. The
//! blocking variant `push_blocking` waits until either room appears
//! or the queue closes — same contract as `EventQueue::push_blocking`.
//!
//! Concurrency invariant: `closed` is stored *inside* the same mutex
//! as the heap itself so a concurrent `close()` cannot race against a
//! waiter's "is the heap empty?" check. Same correctness fix Phase 4
//! ate during its review pass; we apply it from day one here.

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use parking_lot::{Condvar, Mutex};
use thiserror::Error;

use crate::ExtractorId;

#[derive(Debug, Error)]
pub enum QueueError {
    #[error("extraction queue at capacity ({0})")]
    Full(usize),
    #[error("extraction queue is closed")]
    Closed,
}

#[derive(Debug, Clone)]
pub struct ExtractionRequest {
    /// Path the extractor will be run against.
    pub path: PathBuf,
    /// File mtime in ns since epoch — drives priority (max wins).
    /// Callers without a known mtime can pass `0`; those entries sort
    /// to the bottom of the heap and are processed last.
    pub mtime_ns: i128,
    /// Optional pre-dispatched extractor. `None` means "let the
    /// pipeline pick on dequeue" — useful when the journal subscriber
    /// hasn't read the file head yet.
    pub extractor_id: Option<ExtractorId>,
}

impl ExtractionRequest {
    pub fn new(path: PathBuf, mtime_ns: i128) -> Self {
        Self {
            path,
            mtime_ns,
            extractor_id: None,
        }
    }

    pub fn with_extractor(mut self, id: ExtractorId) -> Self {
        self.extractor_id = Some(id);
        self
    }
}

#[derive(Debug, Clone)]
struct Entry {
    request: ExtractionRequest,
    seq: u64,
}

impl Eq for Entry {}
impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Primary: mtime ns asc → reversed at heap level (BinaryHeap
        // is a max-heap, so the larger `cmp` value pops first).
        // Secondary: smaller seq pops first → reverse the seq
        // comparison so the older insertion wins ties.
        match self.request.mtime_ns.cmp(&other.request.mtime_ns) {
            Ordering::Equal => other.seq.cmp(&self.seq),
            ord => ord,
        }
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone)]
pub struct ExtractionQueue {
    inner: Arc<Inner>,
}

struct Inner {
    capacity: usize,
    state: Mutex<State>,
    not_full: Condvar,
    not_empty: Condvar,
}

struct State {
    heap: BinaryHeap<Entry>,
    next_seq: u64,
    closed: bool,
}

impl ExtractionQueue {
    pub fn new(capacity: usize) -> Self {
        let capacity = capacity.max(1);
        Self {
            inner: Arc::new(Inner {
                capacity,
                state: Mutex::new(State {
                    heap: BinaryHeap::new(),
                    next_seq: 0,
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
        self.inner.state.lock().heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_closed(&self) -> bool {
        self.inner.state.lock().closed
    }

    pub fn try_push(&self, request: ExtractionRequest) -> Result<(), QueueError> {
        let mut s = self.inner.state.lock();
        if s.closed {
            return Err(QueueError::Closed);
        }
        if s.heap.len() >= self.inner.capacity {
            return Err(QueueError::Full(self.inner.capacity));
        }
        let seq = s.next_seq;
        s.next_seq = s.next_seq.wrapping_add(1);
        s.heap.push(Entry { request, seq });
        // Notify *one* — only one popper can take this entry.
        self.inner.not_empty.notify_one();
        Ok(())
    }

    /// Blocks until the request fits (room appears) or the queue
    /// closes. On close, returns `Err(QueueError::Closed)`.
    pub fn push_blocking(&self, request: ExtractionRequest) -> Result<(), QueueError> {
        let mut s = self.inner.state.lock();
        loop {
            if s.closed {
                return Err(QueueError::Closed);
            }
            if s.heap.len() < self.inner.capacity {
                let seq = s.next_seq;
                s.next_seq = s.next_seq.wrapping_add(1);
                s.heap.push(Entry { request, seq });
                self.inner.not_empty.notify_one();
                return Ok(());
            }
            self.inner.not_full.wait(&mut s);
        }
    }

    /// Non-blocking dequeue. Returns the highest-priority request, or
    /// `None` when the queue is empty (regardless of close state).
    pub fn try_pop(&self) -> Option<ExtractionRequest> {
        let mut s = self.inner.state.lock();
        let entry = s.heap.pop()?;
        self.inner.not_full.notify_one();
        Some(entry.request)
    }

    /// Blocks until a request arrives or the queue closes-and-drains.
    /// Returns `None` once the queue is *both* closed *and* empty —
    /// the standard "channel closed" signal for the worker loop.
    pub fn pop_blocking(&self) -> Option<ExtractionRequest> {
        let mut s = self.inner.state.lock();
        loop {
            if let Some(entry) = s.heap.pop() {
                self.inner.not_full.notify_one();
                return Some(entry.request);
            }
            if s.closed {
                return None;
            }
            self.inner.not_empty.wait(&mut s);
        }
    }

    /// Close the queue. Drains stay accessible; new pushes refuse.
    /// Wakes every waiting popper (so they can either return their
    /// result or learn the queue is closed) and pusher (so they can
    /// surface `Closed`).
    pub fn close(&self) {
        let mut s = self.inner.state.lock();
        if s.closed {
            return;
        }
        s.closed = true;
        self.inner.not_empty.notify_all();
        self.inner.not_full.notify_all();
    }

    /// Test/diag helper: wait up to `timeout` for the queue to be
    /// non-empty, returning `true` on a hit.
    pub fn wait_for_non_empty(&self, timeout: Duration) -> bool {
        let mut s = self.inner.state.lock();
        if !s.heap.is_empty() {
            return true;
        }
        let res = self.inner.not_empty.wait_for(&mut s, timeout);
        !s.heap.is_empty() && !res.timed_out()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    fn req(path: &str, mtime_ns: i128) -> ExtractionRequest {
        ExtractionRequest::new(PathBuf::from(path), mtime_ns)
    }

    #[test]
    fn highest_mtime_pops_first() {
        let q = ExtractionQueue::new(8);
        q.try_push(req("/old", 100)).unwrap();
        q.try_push(req("/new", 300)).unwrap();
        q.try_push(req("/mid", 200)).unwrap();
        let r1 = q.try_pop().unwrap();
        let r2 = q.try_pop().unwrap();
        let r3 = q.try_pop().unwrap();
        assert_eq!(r1.path, PathBuf::from("/new"));
        assert_eq!(r2.path, PathBuf::from("/mid"));
        assert_eq!(r3.path, PathBuf::from("/old"));
        assert!(q.try_pop().is_none());
    }

    #[test]
    fn ties_pop_fifo() {
        // Same mtime → insertion order wins (the Phase-7 contract:
        // recently-touched first; FIFO inside ties keeps the dispatch
        // pattern predictable).
        let q = ExtractionQueue::new(8);
        q.try_push(req("/a", 500)).unwrap();
        q.try_push(req("/b", 500)).unwrap();
        q.try_push(req("/c", 500)).unwrap();
        assert_eq!(q.try_pop().unwrap().path, PathBuf::from("/a"));
        assert_eq!(q.try_pop().unwrap().path, PathBuf::from("/b"));
        assert_eq!(q.try_pop().unwrap().path, PathBuf::from("/c"));
    }

    #[test]
    fn try_push_at_capacity_returns_full() {
        let q = ExtractionQueue::new(2);
        q.try_push(req("/a", 1)).unwrap();
        q.try_push(req("/b", 2)).unwrap();
        let err = q.try_push(req("/c", 3)).unwrap_err();
        assert!(matches!(err, QueueError::Full(2)));
    }

    #[test]
    fn try_push_after_close_refuses() {
        let q = ExtractionQueue::new(2);
        q.close();
        let err = q.try_push(req("/x", 1)).unwrap_err();
        assert!(matches!(err, QueueError::Closed));
    }

    #[test]
    fn pop_blocking_returns_none_when_closed_and_empty() {
        let q = ExtractionQueue::new(2);
        q.close();
        assert!(q.pop_blocking().is_none());
    }

    #[test]
    fn pop_blocking_drains_before_returning_none() {
        let q = ExtractionQueue::new(2);
        q.try_push(req("/a", 1)).unwrap();
        q.close();
        // Drain
        assert!(q.pop_blocking().is_some());
        // Now empty + closed → None
        assert!(q.pop_blocking().is_none());
    }

    #[test]
    fn close_unblocks_push_blocking() {
        let q = Arc::new(ExtractionQueue::new(1));
        q.try_push(req("/a", 1)).unwrap(); // queue at capacity
        let q_clone = Arc::clone(&q);
        let h = thread::spawn(move || q_clone.push_blocking(req("/b", 2)));
        thread::sleep(Duration::from_millis(50));
        q.close();
        let r = h.join().unwrap();
        assert!(matches!(r, Err(QueueError::Closed)));
    }

    #[test]
    fn close_unblocks_pop_blocking() {
        let q = Arc::new(ExtractionQueue::new(1));
        let q_clone = Arc::clone(&q);
        let h = thread::spawn(move || q_clone.pop_blocking());
        thread::sleep(Duration::from_millis(50));
        q.close();
        let r = h.join().unwrap();
        assert!(r.is_none());
    }

    #[test]
    fn push_blocking_unblocks_when_room_appears() {
        let q = Arc::new(ExtractionQueue::new(1));
        q.try_push(req("/a", 1)).unwrap();
        let q_clone = Arc::clone(&q);
        let h = thread::spawn(move || q_clone.push_blocking(req("/b", 2)));
        thread::sleep(Duration::from_millis(50));
        // pop drains the slot → blocking pusher wakes
        let _ = q.try_pop().unwrap();
        let r = h.join().unwrap();
        assert!(r.is_ok());
    }

    #[test]
    fn extractor_id_round_trips() {
        let r =
            ExtractionRequest::new(PathBuf::from("/x"), 1).with_extractor(ExtractorId::new("pdf"));
        assert_eq!(r.extractor_id, Some(ExtractorId::new("pdf")));
    }

    #[test]
    fn close_is_idempotent() {
        let q = ExtractionQueue::new(2);
        q.close();
        q.close();
        assert!(q.is_closed());
    }
}
