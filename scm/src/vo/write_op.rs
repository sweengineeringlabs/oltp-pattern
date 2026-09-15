//! [`WriteOp`] — one operation in an atomic [`crate::TransactionalStore::write_batch`].

/// A single write, as part of an atomic batch.
///
/// Generic over the same `K`/`R` a [`crate::TransactionalStore`]
/// implementation names via its own `Key`/`Record` associated types -- not
/// a fixed opaque-bytes payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WriteOp<K, R> {
    /// Upsert `key` to `record`.
    Put(K, R),
    /// Remove `key`, if present.
    Delete(K),
}
