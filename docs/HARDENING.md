# Phase B hardening notes

## Contract

- No production unwrap/expect
- Path arguments reject `..` components
- Duration and segment planning covered by proptest
- Landlock disabled only via explicit TRANCE_DISABLE_SANDBOX for export

## Adversarial notes

- Plugin load uses trance-runner allowlist when resolving by name
- Explicit --plugin-path is power-user; still rejects `..`
- Segment concat uses ffmpeg demuxer with single-quoted escaped paths
