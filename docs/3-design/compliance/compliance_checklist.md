# Architecture Compliance Checklist

**Audience**: Architects, contributors, reviewers.

Derived from [architecture.md](../architecture.md). Every rule here is enforceable —
re-run the listed command after any change and expect the stated result.

## 1. Zero implementation

| # | Rule | Verify |
|---|------|--------|
| 1 | This crate implements none of its own primary traits (`TransactionalStore`) | `grep -rn "^impl TransactionalStore for" scm/src/` returns nothing |
| 2 | No backend/storage technology named anywhere | `grep -nE "^\s*(pub )?(struct\|enum\|fn) \w*(Postgres\|MySql\|Sqlite\|Redis\|DynamoDb)" scm/src/*.rs scm/src/**/*.rs` returns nothing |

## 2. Dependency footprint

| # | Rule | Verify |
|---|------|--------|
| 3 | Only `thiserror` in `[dependencies]` | `grep -A3 "^\[dependencies\]" scm/Cargo.toml` shows exactly this one crate |

## 3. Lint gates

| # | Rule | Verify |
|---|------|--------|
| 4 | `unsafe_code = "deny"` enforced | `cargo build` fails on any `unsafe` block |
| 5 | `missing_docs = "warn"` enforced | `cargo doc --no-deps` warns on any undocumented public item |
| 6 | `cargo clippy --all-targets -- -D warnings` clean | Run before every commit |
| 7 | `cargo fmt --check` clean | Run before every commit |

[← 3-design index](../README.md)
