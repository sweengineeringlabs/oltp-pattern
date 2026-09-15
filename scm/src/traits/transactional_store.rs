//! [`TransactionalStore`] — OLTP transactional record-store contract.

use crate::{Record, RecordKey, StoreError, StoreFuture, WriteOp};

/// Single-key, strongly-consistent transactional record store.
///
/// Implement this trait to plug in any OLTP backend -- an in-process store,
/// or a real transactional database. `oltp-svc` ships an in-process
/// reference implementation; alternative backends are added as their own
/// `spi` crate there.
///
/// # Consistency
///
/// Every method observes the effects of every write that returned `Ok`
/// before it was called (read-your-writes). [`write_batch`] is atomic:
/// every op in the batch is applied, or none are.
///
/// [`write_batch`]: TransactionalStore::write_batch
pub trait TransactionalStore: Send + Sync {
    /// Look up the record stored at `key`, if any.
    fn get(&self, key: &RecordKey) -> StoreFuture<'_, Result<Option<Record>, StoreError>>;

    /// Upsert `record` at `key`.
    fn put(&self, key: RecordKey, record: Record) -> StoreFuture<'_, Result<(), StoreError>>;

    /// Remove the record at `key`, if any. Deleting a key that doesn't exist is not an error.
    fn delete(&self, key: &RecordKey) -> StoreFuture<'_, Result<(), StoreError>>;

    /// Apply every op in `ops` atomically: all succeed, or none are applied.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::TransactionFailed`] if any op could not be applied -- no partial application.
    fn write_batch(&self, ops: Vec<WriteOp>) -> StoreFuture<'_, Result<(), StoreError>>;
}
