//! `oltp_pattern` — the reusable OLTP transactional-record-store pattern.
//!
//! Provides this domain's primitives: the [`TransactionalStore`] trait
//! (single-key point CRUD plus atomic multi-key batch writes),
//! [`RecordKey`]/[`Record`]/[`WriteOp`], and [`StoreError`]. Zero
//! implementation of any kind -- a single-responsibility interface package.
//! Reference and any technology-specific implementations live in
//! `oltp-svc` instead. A consumer depends on this crate plus `oltp-svc` and
//! defines no primitives of its own.
//!
//! Designed contract-first, like `scheduler-pattern` -- no existing pilot
//! for a transactional record store exists anywhere in this org yet. See
//! `docs/3-design/architecture.md` for the domain-modeling reasoning behind
//! each primitive, and for what was deliberately deferred (secondary
//! indexes/range scans, optimistic-concurrency/versioned writes, TTL)
//! rather than built speculatively with no real consumer to validate it
//! against.
//!
//! This is a distinct domain from its sibling `olap-pattern` (declarative
//! queries over the whole dataset, streamed results, not key-addressed --
//! see that crate's own doc comment; built the same day as this crate),
//! from `message-broker-pattern`'s `MessageBroker` (pub/sub delivery, no
//! persistent queryable state), and from the former `TaskQueue` (now
//! `task-queue-pattern`; competing-consumer work dequeue, not a keyed
//! store). `TransactionalStore` and `AnalyticalStore` are deliberately two
//! separate `-pattern` crates, not one "database-pattern" -- different
//! consumers, different evolution drivers, even though a single physical
//! database can implement both roles.

#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod error;
mod traits;
mod types;
mod vo;

pub use error::StoreError;
pub use traits::TransactionalStore;
pub use types::StoreFuture;
pub use vo::{Record, RecordKey, WriteOp};
