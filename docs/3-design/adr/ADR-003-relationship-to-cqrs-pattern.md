# ADR-003: Relationship to `database-cqrs-pattern`

**Status**: Proposed
**Date**: 2026-10-24
**Author**: edge platform engineering
**Relates to**: oltp-pattern#6, `database-cqrs-pattern` ADR-001, `olap-pattern` ADR-003, `edge-data`#10

## Context

oltp-pattern#6 asks whether `TransactionalStore` can serve as the contract for
CQRS-style services. `database-cqrs-pattern` (design discussion, no code yet)
proposes a contract of entity-repository command and query operations:
`{Verb}{EntityName}{Command|Query}{Request|Response}`, a fixed operation set
(`create`, `update`, `delete`, `upsert`, `find`, `list`, and more), one
request, response and error type per operation, and repository traits split
into a write trait and a read trait.

`TransactionalStore` today is a key-addressed store: `get`, `put`, `delete`,
`write_batch`, with bare arguments and a single shared `StoreError`. It is
already generic over `Key` and `Record`.

## Proposed answers to the questions in #6

1. **Command side.** `put`, `delete` and `write_batch` can be expressed as command operations: `put` is the `upsert` command, `delete` is `delete`, and `write_batch` is the atomic batch. `create` (fail if exists) and `update` (fail if absent) are **not** expressible on top of `put` without a check-then-write that races. The CQRS ADR makes the same point in its decision 10. A backend that offers conditional writes would be needed.
2. **Query side.** `get` is a query by key. `list`, filters, ordering and paging are not point CRUD; secondary indexes and range scans are already deferred (see #2). Those belong to the analytical side or a richer contract, not here.
3. **Entity-repository shape.** `Employer` + `EmployerRepository` + `CreateEmployerCommandRequest` is a consumer-level concern. This crate cannot name entities, which matches ADR-001's scoping.
4. **Generic vs per-entity types.** Undecided in `database-cqrs-pattern` (open question 6 there). It does not affect this crate.
5. **Boundary.** Keep `oltp-pattern` as it is and treat `database-cqrs-pattern` as a separate, higher-level contract.
6. **Cost of extending.** Replacing bare arguments with request and response types, and the shared `StoreError` with per-operation errors, is a breaking change for `oltp-svc` and every `TransactionalStore` consumer. The cost was not measured; no consumer search was done.

## Decision (proposed)

- **Keep `TransactionalStore` unchanged.** It remains the write-store contract for key-addressed storage.
- **Do not absorb the CQRS contract into this crate.** ADR-001 already separated OLTP from OLAP on single-responsibility grounds; entity-repository CRUD with request and response types is a third concern.
- **A CQRS command repository may be implemented on top of `TransactionalStore`** by an adapter outside this repo, mapping `upsert` to `put`, `delete` to `delete` and the atomic batch to `write_batch`.

Illustrative only, not compiled, and not a committed API:

```rust
// Adapter in a consumer or a future database-cqrs svc crate.
struct StoreCommandRepository<S: TransactionalStore> { store: S }

// For any entity E (Employer, Customer, ...) with Id = S::Key and E = S::Record:
//   upsert(UpsertEntityCommandRequest<E>) -> store.put(id, entity)
//   delete(DeleteEntityCommandRequest<E>) -> store.delete(&id)
// create / update need conditional writes TransactionalStore does not offer.
```

The same adapter shape serves `Employer` and `Customer` unchanged; only the
entity type parameter differs.

## Alternatives considered

- **Extend `TransactionalStore`** with request and response types and per-operation errors. Rejected: breaking for `oltp-svc` and all consumers, and it still could not express `create` and `update` safely.
- **Retire one of the two repos.** Not warranted yet: `database-cqrs-pattern` has no code, and its own ADR leaves the relationship open.

## Open questions

1. Does `database-cqrs-pattern` want `create` and `update` as distinct operations? If so, `TransactionalStore` needs a conditional-write operation (a possible future addition, which must be evaluated separately, since it is a trait change).
2. Does `database-cqrs-pattern` end up depending on this crate, or only being compatible with it?

## Status of #6's acceptance criteria

The ADR records a decision and the alternatives. The worked example above is a sketch, not a compiled one, and it shows the adapter shape once rather than a full `Employer` / `Customer` walk-through. #6 should not be closed on this ADR alone until the CQRS contract's open questions are settled.
