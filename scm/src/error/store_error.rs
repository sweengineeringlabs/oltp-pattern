//! [`StoreError`] — domain error type for the OLTP transactional-store pattern.

/// Errors that can occur while reading or writing a [`crate::TransactionalStore`].
///
/// Not generic over [`crate::TransactionalStore::Key`] -- every variant here
/// (including `NotFound`) carries a plain, human-readable description
/// instead of a structured key, the same shape `WriteFailed`/
/// `TransactionFailed`/`ConnectionFailed` already use. Making `StoreError`
/// itself generic to let `NotFound` carry a structured `Self::Key` would
/// propagate a generic parameter through every `Result<_, StoreError>` in
/// the trait for a variant no real caller constructs today (`get` returns
/// `Ok(None)` for a missing key, not an error) -- disproportionate for the
/// benefit.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum StoreError {
    /// No record exists for this key.
    #[error("record not found: {0}")]
    NotFound(String),
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
