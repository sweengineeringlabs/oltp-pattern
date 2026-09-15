//! Integration tests for [`oltp_pattern::WriteOp`].

use oltp_pattern::{Record, RecordKey, WriteOp};

/// @covers: WriteOp::Put/Delete — the two variants carry distinct data and
/// are not conflated by equality.
#[test]
fn test_put_and_delete_for_the_same_key_are_not_equal() {
    let key = RecordKey::new("k");
    let put = WriteOp::Put(key.clone(), Record::new(b"v".to_vec()));
    let delete = WriteOp::Delete(key);
    assert_ne!(put, delete);
}

/// @covers: WriteOp::Put — carries the exact key and record it was
/// constructed with.
#[test]
fn test_put_carries_its_key_and_record() {
    let op = WriteOp::Put(RecordKey::new("k"), Record::new(b"v".to_vec()));
    match op {
        WriteOp::Put(key, record) => {
            assert_eq!(key.as_str(), "k");
            assert_eq!(record.payload, b"v".to_vec());
        }
        WriteOp::Delete(_) => panic!("expected Put"),
    }
}
