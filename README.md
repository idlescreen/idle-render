# idle-render

Offline renderer for [IdleScreen](https://github.com/idlescreen/idle-core) effects.

Steps saver plugin math (`update` / `draw`) as fast as the CPU allows, rasterizes
cells, and encodes **AV1** via ffmpeg (`libsvtav1` preferred).

## Build

```bash
git clone https://github.com/idlescreen/idle-core.git
git clone https://github.com/idlescreen/idle-render.git
cd idle-render
cargo build --release
```

## Usage

```bash
# Plan only
idle-render --effect beams --duration 10s --seed 1 --dry-run -o /tmp/out.mkv

# Real encode (plugin must be installed or --plugin-path)
idle-render --effect beams --duration 5s --fps 30 \
  --width 1280 --height 720 --seed 0xC0FFEE \
  -o /tmp/beams.mkv
```

Seed is exported as `IDLE_RENDER_SEED` / `TRANCE_SEED` for plugins that honor it.

## License

Apache-2.0.
