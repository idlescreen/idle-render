# idle-render

Offline renderer for [IdleScreen](https://github.com/idlescreen/idle-core) effects.

Plugins already expose pure simulation (`update(dt)` + `draw`). This tool steps
that math as fast as the CPU allows, rasterizes terminal cells, and encodes
video (target: AV1 via ffmpeg/SVT-AV1). No wall-clock screen capture.

## Status

**Scaffold.** CLI surface and docs exist; plugin load + encode loop is next.

```bash
cargo run -- --effect beams --seed 1 --duration 10s -o /tmp/out.mkv
# currently exits with code 2 and prints the plan
```

## Planned CLI

```bash
idle-render \
  --effect beams \
  --seed 0xC0FFEE \
  --fps 30 \
  --duration 8h \
  --width 1920 --height 1080 \
  -o masters/beams-8h.mkv
```

## Docs

- [docs/PIPELINE.md](docs/PIPELINE.md) — offline pipeline
- Product / monetization: [idle-pro](https://github.com/idlescreen/idle-pro)
- Creative TUI (later): [idle-studio](https://github.com/idlescreen/idle-studio)

## License

Apache-2.0.
