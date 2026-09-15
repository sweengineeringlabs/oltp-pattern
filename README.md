# oltp-pattern

> **TLDR:** The reusable OLTP transactional-record-store pattern — single-key
> point CRUD plus atomic multi-key batch writes. Zero implementation. See
> [Architecture](docs/3-design/architecture.md) for the full design.

A single crate, not a workspace, not a store. Implement `TransactionalStore`
to plug in any OLTP backend; this repo ships none — see
[`oltp-svc`](https://github.com/sweengineeringlabs/oltp-svc) for a reference
in-process implementation. A consumer depends on this crate plus `oltp-svc`
and defines no primitives of its own.

Designed contract-first, with no existing pilot to extract from — see
[ADR-001](docs/3-design/adr/ADR-001-contract-first-domain-design.md) for the
domain-modeling reasoning.

## Quick Start

```rust
use oltp_pattern::{Record, RecordKey, TransactionalStore};

async fn store_and_fetch(store: &impl TransactionalStore) {
    let key = RecordKey::new("order-42");
    store.put(key.clone(), Record::new(b"paid".to_vec())).await.unwrap();
    let record = store.get(&key).await.unwrap();
    assert_eq!(record.unwrap().payload, b"paid".to_vec());
}
```

## Documentation

| Document | Description |
|----------|--------------|
| [Docs index](docs/README.md) | Full documentation index |
| [Architecture](docs/3-design/architecture.md) | Component diagram, dependency rationale |
| [ADR-001](docs/3-design/adr/ADR-001-contract-first-domain-design.md) | Why this crate is contract-first, what's deliberately deferred |
| [Developer Guide](docs/4-development/developer_guide.md) | Repo layout, branching, working on this crate |

## License

MIT OR Apache-2.0
