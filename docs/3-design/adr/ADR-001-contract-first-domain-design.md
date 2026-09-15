# ADR-001: Contract-first domain design, no existing pilot

**Status**: Accepted
**Date**: 2026-09-15

## Context

Like `scheduler-pattern`, no existing implementation of an OLTP
transactional record store exists anywhere in this org. This crate is
designed contract-first: model the domain soundly from first principles,
rather than waiting for an accidental existing implementation to extract
from. See
[Pattern/Svc Workflow](https://github.com/sweengineeringlabs/template-engine/blob/main/pattern_svc_workflow.md)'s
own correction on this point — a real implementation to extract from is not
a prerequisite for a sound contract.

"Contract-first" is not license for unconstrained scope, though: every
primitive below is scoped to what's needed to express "read, write, delete
one record by key, and atomically apply a small batch of such writes" — the
smallest real, well-understood OLTP domain — not an attempt to anticipate
every possible transactional-store need up front.

This crate was designed alongside its sibling, `olap-pattern` (OLAP
analytical queries), the same day, deliberately as two separate crates
rather than one "database-pattern" — see both crates' own lib.rs doc
comments and `template-engine`'s own SRP guidance
(`pattern_svc_workflow.md`'s "Single Responsibility" section) for why.

## Decision

- **`TransactionalStore`**: `get(key)`, `put(key, record)`, `delete(key)`,
  `write_batch(ops)`. Four operations, covering point read/write/delete
  plus atomic multi-key write — the well-understood core of OLTP. No query
  method (that's `olap-pattern`'s job).
- **`RecordKey`**: opaque `String` wrapper. Simpler than
  `message-broker-pattern::TaskId`/`scheduler-pattern::JobId` (both `Uuid`
  wrappers) because a record key is caller-chosen and caller-meaningful
  (`"order-42"`), not backend-generated — a store doesn't mint its own
  keys, callers address records by keys they already know.
- **`Record`**: raw bytes only, no schema — same "caller owns the schema"
  shape as `message-broker-pattern::Message`.
- **`WriteOp`**: `Put(RecordKey, Record)` / `Delete(RecordKey)`, the
  payload of `write_batch`. A single sum type covering both mutation kinds
  keeps `write_batch` able to express "replace this with those" atomically
  in one call, rather than requiring two separate batch methods that could
  be split across two round trips.
- **`StoreError`**: `NotFound(RecordKey)`, `WriteFailed`,
  `TransactionFailed`, `ConnectionFailed`. Four variants covering the read
  path, single-key writes, the atomic batch path, and backend
  reachability — no separate error type per operation, since a caller
  handling `StoreError` shouldn't need to match a different type depending
  on which method it called.
- **No `Validator`/config-validation trait** in this crate, unlike
  `message-broker-pattern`'s `Validator` or `executor-pattern`'s
  `Validator`. No backend config type exists yet to validate — adding this
  speculatively, before `oltp-svc` has a real backend with real config
  fields, would be exactly the premature-generalization trap this ADR's
  own "contract-first is not unconstrained scope" principle warns against.
  Add it when `oltp-svc`'s first real backend needs it, following
  `executor-pattern`'s own `Validator` as the shape to match. Same
  reasoning `scheduler-pattern`'s own ADR-001 gives for the same omission.

## Consequences

- `oltp-pattern` depends on `thiserror` only (error boilerplate) — no
  `futures` dependency, unlike `message-broker-pattern`/`scheduler-pattern`,
  because `StoreFuture` only needs `std::future::Future`/`std::pin::Pin`,
  not `futures::future::BoxFuture` (whose lifetime shape those two crates'
  own future wrappers reuse directly, but this crate defines its own local
  equivalent from `std` alone rather than pull in `futures` for one type
  alias). No `uuid` dependency either, since `RecordKey` wraps a caller-
  supplied `String`, not a backend-generated `Uuid`.
- `oltp-svc` (companion repo) starts with a `core` reference implementation
  only — no `spi` backend yet, since no external OLTP technology (a real
  transactional database) has a real consumer demanding it. See that
  repo's own architecture.md.
- Every primitive here should be revisited once `oltp-svc` has a real
  backend and `oltp-pattern` has a real consumer outside this org's own
  repos — contract-first design is a starting point verified by later real
  use, not a one-shot guess treated as final.
