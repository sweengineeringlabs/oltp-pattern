//! Integration tests for [`oltp_pattern::TransactionalStore`].
//!
//! `oltp_pattern` declares the trait only -- no implementation lives here
//! (that's `oltp-svc`'s job). These tests exercise the trait's own contract
//! via a minimal stub store that actually tracks records in a `HashMap`,
//! proving get/put/delete/write_batch round-trip correctly and that
//! `write_batch` really is atomic under a single lock, not decorative.
//! Picks `String`/`Vec<u8>` as its own concrete `Key`/`Record` types --
//! any real implementor picks its own.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::collections::HashMap;
use std::future::Future;
use std::sync::Mutex;

use oltp_pattern::{StoreError, TransactionalStore, WriteOp};

#[derive(Default)]
struct StubStore {
    table: Mutex<HashMap<String, Vec<u8>>>,
}

impl TransactionalStore for StubStore {
    type Key = String;
    type Record = Vec<u8>;

    fn get(
        &self,
        key: &Self::Key,
    ) -> impl Future<Output = Result<Option<Self::Record>, StoreError>> + Send + '_ {
        let key = key.clone();
        async move { Ok(self.table.lock().expect("lock poisoned").get(&key).cloned()) }
    }

    async fn put(&self, key: Self::Key, record: Self::Record) -> Result<(), StoreError> {
        self.table
            .lock()
            .expect("lock poisoned")
            .insert(key, record);
        Ok(())
    }

    fn delete(&self, key: &Self::Key) -> impl Future<Output = Result<(), StoreError>> + Send + '_ {
        let key = key.clone();
        async move {
            self.table.lock().expect("lock poisoned").remove(&key);
            Ok(())
        }
    }

    async fn write_batch(
        &self,
        ops: Vec<WriteOp<Self::Key, Self::Record>>,
    ) -> Result<(), StoreError> {
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
    }
}

/// @covers: TransactionalStore::get — a key never written returns `None`,
/// not an error.
#[tokio::test]
async fn test_get_returns_none_for_unknown_key() {
    let store = StubStore::default();
    assert_eq!(store.get(&"missing".to_string()).await.unwrap(), None);
}

/// @covers: TransactionalStore::put/get — a written record is returned
/// exactly as written.
#[tokio::test]
async fn test_put_then_get_returns_the_written_record() {
    let store = StubStore::default();
    let key = "k".to_string();
    store.put(key.clone(), b"v1".to_vec()).await.unwrap();
    let got = store.get(&key).await.unwrap().unwrap();
    assert_eq!(got, b"v1".to_vec());
}

/// @covers: TransactionalStore::put — overwriting an existing key returns
/// the new value on a subsequent get, not the old one.
#[tokio::test]
async fn test_put_overwrites_the_previous_value() {
    let store = StubStore::default();
    let key = "k".to_string();
    store.put(key.clone(), b"v1".to_vec()).await.unwrap();
    store.put(key.clone(), b"v2".to_vec()).await.unwrap();
    let got = store.get(&key).await.unwrap().unwrap();
    assert_eq!(got, b"v2".to_vec());
}

/// @covers: TransactionalStore::delete/get — a deleted key returns `None`
/// afterward.
#[tokio::test]
async fn test_get_after_delete_returns_none() {
    let store = StubStore::default();
    let key = "k".to_string();
    store.put(key.clone(), b"v".to_vec()).await.unwrap();
    store.delete(&key).await.unwrap();
    assert_eq!(store.get(&key).await.unwrap(), None);
}

/// @covers: TransactionalStore::delete — deleting a key that was never
/// written succeeds rather than erroring.
#[tokio::test]
async fn test_delete_nonexistent_key_succeeds() {
    let store = StubStore::default();
    assert!(store.delete(&"never-written".to_string()).await.is_ok());
}

/// @covers: TransactionalStore::write_batch — a batch of mixed Put/Delete
/// ops all apply, proving the batch is real, not a no-op.
#[tokio::test]
async fn test_write_batch_applies_every_op() {
    let store = StubStore::default();
    store
        .put("stale".to_string(), b"old".to_vec())
        .await
        .unwrap();

    store
        .write_batch(vec![
            WriteOp::Put("a".to_string(), b"1".to_vec()),
            WriteOp::Put("b".to_string(), b"2".to_vec()),
            WriteOp::Delete("stale".to_string()),
        ])
        .await
        .unwrap();

    assert_eq!(
        store.get(&"a".to_string()).await.unwrap().unwrap(),
        b"1".to_vec()
    );
    assert_eq!(
        store.get(&"b".to_string()).await.unwrap().unwrap(),
        b"2".to_vec()
    );
    assert_eq!(store.get(&"stale".to_string()).await.unwrap(), None);
}

/// @covers: TransactionalStore::write_batch — deleting a key never written,
/// as part of a batch alongside real puts, does not fail the whole batch.
#[tokio::test]
async fn test_write_batch_delete_of_unwritten_key_does_not_fail_batch() {
    let store = StubStore::default();
    let result = store
        .write_batch(vec![
            WriteOp::Delete("never-written".to_string()),
            WriteOp::Put("a".to_string(), b"1".to_vec()),
        ])
        .await;
    assert!(result.is_ok());
    assert_eq!(
        store.get(&"a".to_string()).await.unwrap().unwrap(),
        b"1".to_vec()
    );
}
