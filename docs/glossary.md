# Glossary

Alphabetized list of terms used in `oltp-pattern`.

---

**StoreError** - Domain error type: `NotFound(String)`, `WriteFailed`, `TransactionFailed`, `ConnectionFailed`. Not generic over `Key` -- every variant carries a plain description, matching `WriteFailed`/`TransactionFailed`/`ConnectionFailed`'s own shape.

**TransactionalStore** - OLTP transactional record-store contract: `type Key`/`type Record` associated types, `get(key)`, `put(key, record)`, `delete(key)`, `write_batch(ops)`. Generic over its own key/record types, not a fixed opaque-bytes payload -- an in-process caller pays no serialization cost it doesn't need. Methods return `impl Future` (RPITIT), not a boxed future -- zero-cost, at the cost of object safety; see `docs/3-design/architecture.md`'s "Why `TransactionalStore` is generic over `Key`/`Record`".

**WriteOp** - One operation in an atomic `TransactionalStore::write_batch`: `Put(K, R)` or `Delete(K)`, generic over the same `Key`/`Record` types a `TransactionalStore` implementation names.

[← Docs index](README.md)
