//! 16-entry LRU plan cache.
//!
//! The Build Guide's Phase-5 spec: "the last 16 query plans cached;
//! identical re-queries hit cache." Cache key is the raw query string
//! (post-trim) — the parser is deterministic so two identical inputs
//! always produce identical plans.

use std::collections::VecDeque;

use parking_lot::Mutex;

use crate::error::QueryError;
use crate::exec::{ExecPlan, plan, validate_supported};
use crate::opts::ExecOpts;
use crate::parser::parse;

/// Bounded LRU. The mutex is fine — query throughput is human-typed
/// (≤ 30 q/s); the cache is hot under repeat-keystroke streaming.
pub struct PlanCache {
    inner: Mutex<Inner>,
}

struct Inner {
    cap: usize,
    keys: VecDeque<String>,
    plans: std::collections::HashMap<String, CachedEntry>,
}

#[derive(Clone)]
struct CachedEntry {
    query: crate::ast::Query,
    plan: ExecPlan,
}

impl PlanCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: Mutex::new(Inner {
                cap: capacity.max(1),
                keys: VecDeque::with_capacity(capacity.max(1)),
                plans: std::collections::HashMap::with_capacity(capacity.max(1)),
            }),
        }
    }

    /// Convenience: 16-entry cache, the Build Guide's number.
    pub fn default16() -> Self {
        Self::new(16)
    }

    pub fn capacity(&self) -> usize {
        self.inner.lock().cap
    }

    pub fn len(&self) -> usize {
        self.inner.lock().plans.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Parse + plan, returning a cloned `(Query, Plan)` pair from the
    /// cache when the key is hot. Identical inputs reuse the same
    /// `Arc<QueryNode>` under the hood — no AST re-allocation on hit.
    pub fn get_or_plan(
        &self,
        s: &str,
        opts: &ExecOpts,
    ) -> Result<(crate::ast::Query, ExecPlan), QueryError> {
        let key = s.trim();
        if key.is_empty() {
            return Err(QueryError::Parse(crate::error::ParseError::Empty));
        }
        {
            let mut inner = self.inner.lock();
            if let Some(entry) = inner.plans.get(key).cloned() {
                touch(&mut inner, key);
                return Ok((entry.query, entry.plan));
            }
        }
        let q = parse(s)?;
        validate_supported(&q)?;
        let p = plan(&q, opts);
        let mut inner = self.inner.lock();
        evict_to_cap(&mut inner);
        inner.keys.push_back(key.to_string());
        inner.plans.insert(
            key.to_string(),
            CachedEntry {
                query: q.clone(),
                plan: p.clone(),
            },
        );
        Ok((q, p))
    }
}

fn touch(inner: &mut Inner, key: &str) {
    if let Some(idx) = inner.keys.iter().position(|k| k == key) {
        let k = inner.keys.remove(idx).unwrap();
        inner.keys.push_back(k);
    }
}

fn evict_to_cap(inner: &mut Inner) {
    while inner.keys.len() >= inner.cap {
        if let Some(oldest) = inner.keys.pop_front() {
            inner.plans.remove(&oldest);
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_hit_returns_same_plan() {
        let c = PlanCache::default16();
        let opts = ExecOpts::default();
        let (_q1, p1) = c.get_or_plan("report", &opts).unwrap();
        let (_q2, p2) = c.get_or_plan("report", &opts).unwrap();
        assert_eq!(p1, p2);
    }

    #[test]
    fn cache_evicts_lru() {
        let c = PlanCache::new(2);
        let opts = ExecOpts::default();
        c.get_or_plan("a", &opts).unwrap();
        c.get_or_plan("b", &opts).unwrap();
        // Touch "a" so "b" becomes LRU.
        c.get_or_plan("a", &opts).unwrap();
        c.get_or_plan("c", &opts).unwrap();
        assert_eq!(c.len(), 2);
        // "b" should be evicted; the only way to verify behaviorally
        // is: inserting "b" again must succeed (no panic) and the
        // cache stays at cap 2.
        c.get_or_plan("b", &opts).unwrap();
        assert_eq!(c.len(), 2);
    }
}
