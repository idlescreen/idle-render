# render

Offline renderer for IdleScreen visual effects. Steps saver plugin math on a
fixed timeline, rasterizes terminal cells to pixels, and encodes AV1 video with
ffmpeg. No wall-clock screen capture.

## Why

IdleScreen plugins already expose pure simulation (`update` and `draw`). This
tool runs that simulation as fast as the CPU allows so multi-hour masters can be
produced without leaving a display running overnight.

## Install and build

Requires a sibling checkout of idle-core for path dependencies:

```bash
git clone https://github.com/idlescreen/idle-core.git
git clone https://github.com/idlescreen/render.git
cd render
cargo build --release
```

System tools: a C toolchain, font stack used by the cell renderer, and ffmpeg
with an AV1 encoder (`libsvtav1` preferred).

## Usage

```bash
# Plan only
render --effect beams --duration 10s --seed 1 --dry-run -o /tmp/out.mkv

# Encode (plugin installed or --plugin-path)
render --effect beams --duration 5s --fps 30 \
  --width 1280 --height 720 --seed 0xC0FFEE -o /tmp/beams.mkv

# Long encode in one-hour segments, then concat
render --effect storm --duration 8h --segment 1h -o /tmp/night.mkv

# Optional looping audio bed
render --effect beams --duration 10m --audio bed.mp3 -o /tmp/with-audio.mkv
```

Seed is exported as `IDLESCREEN_RENDER_SEED` and `TRANCE_SEED` for plugins that honor
deterministic RNG. Export sets `TRANCE_DISABLE_SANDBOX=1` so frame output is not
blocked by the live daemon Landlock profile.

## Safety

Output, plugin, and audio paths must not contain `..` components. Prefer
allowlisted effect names resolved by idle-runner discovery; use `--plugin-path`
only for controlled builds.

## Related

| Project | Role |
|---------|------|
| idle-core | Daemon, plugin API, CLI |
| app-studio | Job queue and Director TUI |
| saver-* | Official effects |
| packages | APT/DNF host |

## License

Apache-2.0.
