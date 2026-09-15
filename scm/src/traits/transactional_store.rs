//! [`TransactionalStore`] — OLTP transactional record-store contract.

use std::future::Future;

use crate::{StoreError, WriteOp};

/// Single-key, strongly-consistent transactional record store.
///
/// Generic over its own key and record types (`type Key`/`type Record`),
/// not a fixed opaque-bytes payload -- an in-process consumer pays no
/// serialization cost it doesn't need. Implement this trait to plug in any
/// OLTP backend -- an in-process store, or a real transactional database.
/// `oltp-svc` ships an in-process reference implementation; alternative
/// backends are added as their own `spi` crate there.
///
/// # Consistency
///
/// Every method observes the effects of every write that returned `Ok`
/// before it was called (read-your-writes). [`write_batch`] is atomic:
/// every op in the batch is applied, or none are.
///
/// [`write_batch`]: TransactionalStore::write_batch
pub trait TransactionalStore: Send + Sync {
    /// The key type this store is addressed by.
    type Key: Send + Sync + Clone + 'static;
    /// The record type this store holds.
    type Record: Send + 'static;

    /// Look up the record stored at `key`, if any.
    fn get(
        &self,
        key: &Self::Key,
    ) -> impl Future<Output = Result<Option<Self::Record>, StoreError>> + Send + '_;

    /// Upsert `record` at `key`.
    fn put(
        &self,
        key: Self::Key,
        record: Self::Record,
    ) -> impl Future<Output = Result<(), StoreError>> + Send + '_;

    /// Remove the record at `key`, if any. Deleting a key that doesn't exist is not an error.
    fn delete(&self, key: &Self::Key) -> impl Future<Output = Result<(), StoreError>> + Send + '_;

    /// Apply every op in `ops` atomically: all succeed, or none are applied.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::TransactionFailed`] if any op could not be applied -- no partial application.
    fn write_batch(
        &self,
        ops: Vec<WriteOp<Self::Key, Self::Record>>,
    ) -> impl Future<Output = Result<(), StoreError>> + Send + '_;
}
