//! [`WriteOp`] — one operation in an atomic [`crate::TransactionalStore::write_batch`].

use crate::{Record, RecordKey};

/// A single write, as part of an atomic batch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WriteOp {
    /// Upsert `key` to `record`.
    Put(RecordKey, Record),
    /// Remove `key`, if present.
    Delete(RecordKey),
}
