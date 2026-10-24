# ADR-002: Candidate durable backends for `TransactionalStore`

**Status**: Proposed
**Date**: 2026-10-24
**Author**: edge platform engineering
**Relates to**: ADR-001 (contract-first design), `oltp-svc` ADR-002 (where backends live), `olap-pattern` ADR-002

> Nothing in this ADR has been built or run. Library capabilities below are
> stated from general knowledge of each crate and have **not** been verified
> against current releases, and no `cargo audit` / `cargo deny` result exists
> for any of them. Those checks gate acceptance (see Open questions).

## Context

`TransactionalStore` has one implementation today: `oltp-svc`'s
`InMemoryTransactionalStore`, a single `RwLock<HashMap<..>>` with no
persistence. No durable backend exists, and ADR-001's rule against
speculative generalization still holds: do not build one until a consumer
needs it. This ADR records the candidates and the constraints the trait
places on them, so the choice is not re-derived when that consumer appears.

What the trait demands of a backend:

- **Point operations**: `get`, `put` (upsert), `delete` (absent key is not an error).
- **Atomic batch**: `write_batch` applies every `WriteOp` or none (`StoreError::TransactionFailed`).
- **Read-your-writes** consistency.
- **Async methods** (`impl Future`), `Send + Sync`.
- **Generic `Key` / `Record`** with bounds `Send + Sync + Clone + 'static` / `Send + 'static`. There is **no serialization bound**.

## Candidates

| Candidate | Kind | Fit with the trait |
|-----------|------|--------------------|
| `redb` | embedded, pure-Rust key-value | ACID write transactions map onto `write_batch`. Synchronous API. |
| `rocksdb` | embedded key-value (C++ bindings) | `WriteBatch` maps onto `write_batch`. Native build dependency. Synchronous API. |
| `rusqlite` | embedded SQL | A table with a key column and a blob column; a transaction maps onto `write_batch`. Already in the org via `edge-data` (bundled). Synchronous API. |
| `tokio-postgres` + `deadpool-postgres` | networked SQL | Same table shape; native async. Already in the org via `edge-data`. |
| `fjall`, `heed` (LMDB), `sled` | embedded key-value | Same shape as `redb`. `sled`'s maintenance status needs checking before any use. |

## Constraints that apply to every candidate

1. **Encoding.** A durable store must turn `Key` and `Record` into bytes, but the trait does not bound them. A backend must add the bound on its own type (for example `impl<K: AsRef<[u8]>, R: Encode> TransactionalStore for RedbStore<K, R>`), not on the trait. Adding a bound to the trait would be a breaking change for `oltp-svc` and every consumer.
2. **Sync-to-async bridging.** `redb`, `rocksdb` and `rusqlite` are synchronous. The backend would run calls through `tokio::task::spawn_blocking` (or a dedicated thread). That is a cost and a place for a bug (cancellation, panics inside the blocking task) that the in-memory store never had.
3. **Error mapping.** Backend errors map onto `StoreError` (`WriteFailed`, `TransactionFailed`, `ConnectionFailed`). Driver types must not leak through the contract.
4. **Dependency audit.** This org has already rejected `sqlx` over RUSTSEC-2023-0071. Each candidate needs its own `cargo audit` and `cargo deny` pass before adoption.

## Decision (proposed)

- Do **not** add a durable backend in this repo. `oltp-pattern` stays contract-only with `thiserror` as its only dependency.
- When a consumer first needs durability, evaluate **`redb`** first for an embedded store and reuse the **`deadpool-postgres`** stack from `edge-data` for a networked one. `redb` is proposed first because it is pure Rust and its transaction model matches `write_batch` directly. This is a starting point for the evaluation, not a verified choice.
- Backends live in `oltp-svc` as `spi` crates (see `oltp-svc` ADR-002), never in this repo.

## Alternatives considered

- **Ship a SQLite backend now.** Rejected for now: no consumer, and ADR-001 rules out speculative work.
- **Add `Serialize`/`Encode` bounds to the trait.** Rejected: breaking, and it taxes the in-memory case that currently pays no serialization cost.

## Open questions

1. Which consumer triggers the first durable backend, and does it need embedded or networked storage?
2. Does `cargo audit` / `cargo deny` pass for the chosen crate?
3. What is the encoding contract for `Key` and `Record` (a small local trait, or reuse of an existing one)?
4. Should the blocking-to-async bridge be shared across `spi` crates?

## Consequences

- No code or dependency change today.
- The first durable backend is a minor addition in `oltp-svc`; if it ever requires changing the trait, that is a breaking release of this crate and must be called out.
