# Architecture Decision Records

**Audience**: Architects, technical leads, contributors.

| ADR | Status | Date | Decision |
|-----|--------|------|----------|
| [ADR-001](ADR-001-contract-first-domain-design.md) | Accepted | 2026-09-15 | Design `TransactionalStore`/`RecordKey`/`Record`/`WriteOp`/`StoreError` contract-first, with no existing pilot to extract from; scope to single-key CRUD plus atomic batch writes only, deferring secondary indexes/range scans, optimistic-concurrency/versioned writes, and TTL until a real consumer needs them |
| [ADR-002](ADR-002-candidate-durable-backends.md) | Proposed | 2026-10-24 | Candidate durable backends for `TransactionalStore` (`redb`, `rocksdb`, `rusqlite`, `deadpool-postgres`); none added until a consumer needs one, no serialization bound added to the trait |
| [ADR-003](ADR-003-relationship-to-cqrs-pattern.md) | Proposed | 2026-10-24 | Keep `TransactionalStore` unchanged; `database-cqrs-pattern` stays a separate contract, with CQRS commands implemented as an adapter over this trait (oltp-pattern#6) |

[← 3-design index](../README.md)
