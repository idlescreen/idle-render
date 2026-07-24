# AGENT.md — render (Phase B contract)

- Strict Rust, Apache-2.0.
- Max 250 lines per `.rs` file.
- Zero `.unwrap()` / `.expect()` in production code.
- Prefer `std`; vetted crates only (clap, thiserror, tracing, proptest in dev).
- Protocol/parsing logic must have proptest coverage.
- Default branch: master. Commit after each hardening barrier.
