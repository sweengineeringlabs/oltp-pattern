# oltp-pattern Developer Guide

**Audience**: Developers, contributors.

## Repo Structure

```
oltp-pattern/
├── README.md
├── docs/
│   ├── README.md
│   ├── glossary.md
│   ├── 0-ideation/papers/README.md
│   ├── 3-design/README.md, architecture.md
│   ├── 3-design/compliance/compliance_checklist.md
│   ├── 3-design/adr/README.md, ADR-001-contract-first-domain-design.md
│   └── 4-development/README.md, developer_guide.md   # this file
└── scm/
    ├── Cargo.toml          # [package] oltp-pattern -- a single crate,
    │                       #  not a workspace
    ├── src/
    │   ├── lib.rs
    │   ├── traits/          # TransactionalStore
    │   ├── vo/               # WriteOp
    │   └── error/            # StoreError
    └── tests/                 # one *_int_test.rs per public trait/type
```

## Branching and Releases

- `dev` is the default branch; all work lands there first.
- `main` gets fast-forwarded to `dev` after a shipped change, not on every commit.
- Not yet published to crates.io. Current version `0.2.0` — bumped from
  `0.1.0` when `TransactionalStore` moved to associated types, a breaking,
  object-safety-losing change; see architecture.md.

## Working on This Crate

```
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

Dependency footprint is `thiserror` only (error boilerplate) — verify with
`cargo tree --depth 1` after any change that touches `Cargo.toml`.
`unsafe_code = "deny"` and `missing_docs = "warn"` are enforced via
`[lints.rust]` in `Cargo.toml`.

## This Was Designed Contract-First — the "No Existing Pilot" Premise Was Wrong

Every primitive here was originally designed from domain first principles,
on the belief that no prior art for an OLTP transactional record store
existed anywhere in this org. That belief was checked only after the fact
and turned out to be false —
`edge-application-repository-api`'s `Repository<Entity, Id>` was real,
existing prior art, and `TransactionalStore` was redesigned around its
real shape (associated types, not opaque bytes) once found. See
[ADR-001](../3-design/adr/ADR-001-contract-first-domain-design.md)'s own
amendment note and `docs/3-design/architecture.md` for the full story, and
for what's still genuinely scoped out pending a real consumer (secondary
indexes/range scans, optimistic-concurrency/versioned writes, TTL, a
`Validator` trait).

## See Also

- [Architecture](../3-design/architecture.md)
- [ADR-001](../3-design/adr/ADR-001-contract-first-domain-design.md)
- [Pattern/Svc Workflow](https://github.com/sweengineeringlabs/template-engine/blob/main/pattern_svc_workflow.md)
