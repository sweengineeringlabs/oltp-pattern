# Glossary

Alphabetized list of terms used in `oltp-pattern`.

---

**Record** - The opaque value stored under a `RecordKey`: raw bytes only, no schema. Caller owns any structure (JSON, protobuf, etc.).

**RecordKey** - Opaque key addressing a single record in a `TransactionalStore`. Wraps a caller-supplied `String`, not a backend-generated ID.

**StoreError** - Domain error type: `NotFound(RecordKey)`, `WriteFailed`, `TransactionFailed`, `ConnectionFailed`.

**StoreFuture** - Local boxed-future wrapper type, the async return type for every `TransactionalStore` method. Same shape as `message-broker-pattern::BrokerFuture`.

**TransactionalStore** - OLTP transactional record-store contract: `get(key)`, `put(key, record)`, `delete(key)`, `write_batch(ops)`.

**WriteOp** - One operation in an atomic `TransactionalStore::write_batch`: `Put(RecordKey, Record)` or `Delete(RecordKey)`.

[← Docs index](README.md)
