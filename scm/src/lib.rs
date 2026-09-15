//! `oltp_pattern` — the reusable OLTP transactional-record-store pattern.
//!
//! Provides this domain's primitives: the [`TransactionalStore`] trait
//! (single-key point CRUD plus atomic multi-key batch writes, generic over
//! its own `Key`/`Record` associated types -- not a fixed opaque-bytes
//! payload), [`WriteOp`], and [`StoreError`]. Zero implementation of any
//! kind -- a single-responsibility interface package. Reference and any
//! technology-specific implementations live in `oltp-svc` instead. A
//! consumer depends on this crate plus `oltp-svc` and defines no primitives
//! of its own.
//!
//! Originally designed contract-first on the mistaken belief that no
//! existing pilot for a transactional record store existed anywhere in
//! this org -- `edge-application-repository-api`'s `Repository<Entity,
//! Id>` was real, existing prior art, found only after this crate first
//! shipped without checking for it. `TransactionalStore` was redesigned
//! around that precedent's real shape (associated types, not opaque bytes)
//! once found -- see `docs/3-design/architecture.md` for the full
//! reasoning, and for what was deferred (secondary indexes/range scans,
//! optimistic-concurrency/versioned writes, TTL) pending a real consumer to
//! validate against.
//!
//! This is a distinct domain from its sibling `olap-pattern` (declarative
//! queries over the whole dataset, streamed results, not key-addressed --
//! see that crate's own doc comment; built the same day as this crate),
//! from `message-broker-pattern`'s `MessageBroker` (pub/sub delivery, no
//! persistent queryable state), and from the former `TaskQueue` (now
//! `task-queue-pattern`; competing-consumer work dequeue, not a keyed
//! store). `TransactionalStore` and `AnalyticalStore` are two separate
//! `-pattern` crates, not one "database-pattern" -- different consumers,
//! different evolution drivers, even though a single physical database can
//! implement both roles.

#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod error;
mod traits;
mod vo;

pub use error::StoreError;
pub use traits::TransactionalStore;
pub use vo::WriteOp;
