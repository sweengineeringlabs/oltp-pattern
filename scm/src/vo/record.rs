//! [`Record`] — the value stored under a [`crate::RecordKey`].

/// An opaque record payload.
///
/// `Record` carries raw bytes only -- no schema, no column types. Any
/// structure (JSON, protobuf, etc.) is the caller's concern, not this
/// pattern crate's; see `message-broker-pattern::Message` for the same
/// "raw bytes, caller owns the schema" shape used in this org's other
/// zero-implementation contracts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    /// Payload bytes.
    pub payload: Vec<u8>,
}

impl Record {
    /// Construct a record from raw bytes.
    pub fn new(payload: impl Into<Vec<u8>>) -> Self {
        Self {
            payload: payload.into(),
        }
    }
}
