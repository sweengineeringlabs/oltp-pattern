//! Integration tests for [`oltp_pattern::WriteOp`].

use oltp_pattern::WriteOp;

/// @covers: WriteOp::Put/Delete — the two variants carry distinct data and
/// are not conflated by equality.
#[test]
fn test_put_and_delete_for_the_same_key_are_not_equal() {
    let key = "k".to_string();
    let put: WriteOp<String, Vec<u8>> = WriteOp::Put(key.clone(), b"v".to_vec());
    let delete: WriteOp<String, Vec<u8>> = WriteOp::Delete(key);
    assert_ne!(put, delete);
}

/// @covers: WriteOp::Put — carries the exact key and record it was
/// constructed with, for any concrete Key/Record type a caller picks.
#[test]
fn test_put_carries_its_key_and_record() {
    let op: WriteOp<String, Vec<u8>> = WriteOp::Put("k".to_string(), b"v".to_vec());
    match op {
        WriteOp::Put(key, record) => {
            assert_eq!(key, "k");
            assert_eq!(record, b"v".to_vec());
        }
        WriteOp::Delete(_) => panic!("expected Put"),
    }
}
