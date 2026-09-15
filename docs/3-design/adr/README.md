# Architecture Decision Records

**Audience**: Architects, technical leads, contributors.

| ADR | Status | Date | Decision |
|-----|--------|------|----------|
| [ADR-001](ADR-001-contract-first-domain-design.md) | Accepted | 2026-09-15 | Design `TransactionalStore`/`RecordKey`/`Record`/`WriteOp`/`StoreError` contract-first, with no existing pilot to extract from; scope to single-key CRUD plus atomic batch writes only, deferring secondary indexes/range scans, optimistic-concurrency/versioned writes, and TTL until a real consumer needs them |

[← 3-design index](../README.md)
