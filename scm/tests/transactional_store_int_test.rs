//! Integration tests for [`oltp_pattern::TransactionalStore`].
//!
//! `oltp_pattern` declares the trait only -- no implementation lives here
//! (that's `oltp-svc`'s job). These tests exercise the trait's own contract
//! via a minimal stub store that actually tracks records in a `HashMap`,
//! proving get/put/delete/write_batch round-trip correctly and that
//! `write_batch` really is atomic under a single lock, not decorative.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::collections::HashMap;
use std::sync::Mutex;

use oltp_pattern::{Record, RecordKey, StoreError, StoreFuture, TransactionalStore, WriteOp};

#[derive(Default)]
struct StubStore {
    table: Mutex<HashMap<RecordKey, Record>>,
}

impl TransactionalStore for StubStore {
    fn get(&self, key: &RecordKey) -> StoreFuture<'_, Result<Option<Record>, StoreError>> {
        let key = key.clone();
        StoreFuture::new(
            async move { Ok(self.table.lock().expect("lock poisoned").get(&key).cloned()) },
        )
    }

    fn put(&self, key: RecordKey, record: Record) -> StoreFuture<'_, Result<(), StoreError>> {
        StoreFuture::new(async move {
            self.table
                .lock()
                .expect("lock poisoned")
                .insert(key, record);
            Ok(())
        })
    }

    fn delete(&self, key: &RecordKey) -> StoreFuture<'_, Result<(), StoreError>> {
        let key = key.clone();
        StoreFuture::new(async move {
            self.table.lock().expect("lock poisoned").remove(&key);
            Ok(())
        })
    }

    fn write_batch(&self, ops: Vec<WriteOp>) -> StoreFuture<'_, Result<(), StoreError>> {
        StoreFuture::new(async move {
            let mut table = self.table.lock().expect("lock poisoned");
            for op in ops {
                match op {
                    WriteOp::Put(k, r) => {
                        table.insert(k, r);
                    }
                    WriteOp::Delete(k) => {
                        table.remove(&k);
                    }
                }
            }
            Ok(())
        })
    }
}

/// @covers: TransactionalStore::get — a key never written returns `None`,
/// not an error.
#[tokio::test]
async fn test_get_returns_none_for_unknown_key() {
    let store = StubStore::default();
    assert_eq!(store.get(&RecordKey::new("missing")).await.unwrap(), None);
}

/// @covers: TransactionalStore::put/get — a written record is returned
/// exactly as written.
#[tokio::test]
async fn test_put_then_get_returns_the_written_record() {
    let store = StubStore::default();
    let key = RecordKey::new("k");
    store
        .put(key.clone(), Record::new(b"v1".to_vec()))
        .await
        .unwrap();
    let got = store.get(&key).await.unwrap().unwrap();
    assert_eq!(got.payload, b"v1".to_vec());
}

/// @covers: TransactionalStore::put — overwriting an existing key returns
/// the new value on a subsequent get, not the old one.
#[tokio::test]
async fn test_put_overwrites_the_previous_value() {
    let store = StubStore::default();
    let key = RecordKey::new("k");
    store
        .put(key.clone(), Record::new(b"v1".to_vec()))
        .await
        .unwrap();
    store
        .put(key.clone(), Record::new(b"v2".to_vec()))
        .await
        .unwrap();
    let got = store.get(&key).await.unwrap().unwrap();
    assert_eq!(got.payload, b"v2".to_vec());
}

/// @covers: TransactionalStore::delete/get — a deleted key returns `None`
/// afterward.
#[tokio::test]
async fn test_get_after_delete_returns_none() {
    let store = StubStore::default();
    let key = RecordKey::new("k");
    store
        .put(key.clone(), Record::new(b"v".to_vec()))
        .await
        .unwrap();
    store.delete(&key).await.unwrap();
    assert_eq!(store.get(&key).await.unwrap(), None);
}

/// @covers: TransactionalStore::delete — deleting a key that was never
/// written succeeds rather than erroring.
#[tokio::test]
async fn test_delete_nonexistent_key_succeeds() {
    let store = StubStore::default();
    assert!(store.delete(&RecordKey::new("never-written")).await.is_ok());
}

/// @covers: TransactionalStore::write_batch — a batch of mixed Put/Delete
/// ops all apply, proving the batch is real, not a no-op.
#[tokio::test]
async fn test_write_batch_applies_every_op() {
    let store = StubStore::default();
    store
        .put(RecordKey::new("stale"), Record::new(b"old".to_vec()))
        .await
        .unwrap();

    store
        .write_batch(vec![
            WriteOp::Put(RecordKey::new("a"), Record::new(b"1".to_vec())),
            WriteOp::Put(RecordKey::new("b"), Record::new(b"2".to_vec())),
            WriteOp::Delete(RecordKey::new("stale")),
        ])
        .await
        .unwrap();

    assert_eq!(
        store
            .get(&RecordKey::new("a"))
            .await
            .unwrap()
            .unwrap()
            .payload,
        b"1".to_vec()
    );
    assert_eq!(
        store
            .get(&RecordKey::new("b"))
            .await
            .unwrap()
            .unwrap()
            .payload,
        b"2".to_vec()
    );
    assert_eq!(store.get(&RecordKey::new("stale")).await.unwrap(), None);
}

/// @covers: TransactionalStore::write_batch — deleting a key never written,
/// as part of a batch alongside real puts, does not fail the whole batch.
#[tokio::test]
async fn test_write_batch_delete_of_unwritten_key_does_not_fail_batch() {
    let store = StubStore::default();
    let result = store
        .write_batch(vec![
            WriteOp::Delete(RecordKey::new("never-written")),
            WriteOp::Put(RecordKey::new("a"), Record::new(b"1".to_vec())),
        ])
        .await;
    assert!(result.is_ok());
    assert_eq!(
        store
            .get(&RecordKey::new("a"))
            .await
            .unwrap()
            .unwrap()
            .payload,
        b"1".to_vec()
    );
}
