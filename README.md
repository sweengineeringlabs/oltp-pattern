# oltp-pattern

> **TLDR:** The reusable OLTP transactional-record-store pattern — single-key
> point CRUD plus atomic multi-key batch writes. Zero implementation. See
> [Architecture](docs/3-design/architecture.md) for the full design.

A single crate, not a workspace, not a store. Implement `TransactionalStore`
to plug in any OLTP backend; this repo ships none — see
[`oltp-svc`](https://github.com/sweengineeringlabs/oltp-svc) for a reference
in-process implementation. A consumer depends on this crate plus `oltp-svc`
and defines no primitives of its own.

Originally designed contract-first on the belief that no existing pilot
existed to extract from — that belief was wrong (see
[ADR-001](docs/3-design/adr/ADR-001-contract-first-domain-design.md)'s
amendment note); `TransactionalStore` is now shaped around
`edge-application-repository-api`'s real `Repository<Entity, Id>`
precedent instead — generic over its own key/record types, not opaque
bytes.

## Quick Start

```rust
use oltp_pattern::TransactionalStore;

async fn store_and_fetch(store: &impl TransactionalStore<Key = String, Record = Vec<u8>>) {
    let key = "order-42".to_string();
    store.put(key.clone(), b"paid".to_vec()).await.unwrap();
    let record = store.get(&key).await.unwrap();
    assert_eq!(record.unwrap(), b"paid".to_vec());
}
```

## Documentation

| Document | Description |
|----------|--------------|
| [Docs index](docs/README.md) | Full documentation index |
| [Architecture](docs/3-design/architecture.md) | Component diagram, dependency rationale |
| [ADR-001](docs/3-design/adr/ADR-001-contract-first-domain-design.md) | Why this crate is contract-first, the prior-art miss and correction, what's still deferred |
| [Developer Guide](docs/4-development/developer_guide.md) | Repo layout, branching, working on this crate |

## License

MIT OR Apache-2.0
