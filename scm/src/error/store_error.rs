//! [`StoreError`] — domain error type for the OLTP transactional-store pattern.

use crate::RecordKey;

/// Errors that can occur while reading or writing a [`crate::TransactionalStore`].
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum StoreError {
    /// No record exists for this key.
    #[error("record not found: {0}")]
    NotFound(RecordKey),
    /// A single-key write (`put`/`delete`) could not be applied.
    #[error("write failed: {0}")]
    WriteFailed(String),
    /// A `write_batch` could not be applied atomically -- no ops in the batch were applied.
    #[error("transaction failed: {0}")]
    TransactionFailed(String),
    /// The backend could not be reached.
    #[error("connection failed: {0}")]
    ConnectionFailed(String),
}
