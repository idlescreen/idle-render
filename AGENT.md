# AGENT.md — idle-render

- Strict Rust, Apache-2.0.
- Max 250 lines per `.rs` file.
- Zero `.unwrap()` / `.expect()` in production code.
- Prefer `std`; vetted crates only (clap, thiserror).
- Path-deps sibling `idle-core` for trance-runner / plugin host.
- Default branch: `master`. Commit and push after each barrier.
