# Architecture — render (Build 0)

## Goal

Offline, faster-than-realtime export of IdleScreen `saver-*` effects to video
(AV1 via ffmpeg). No wall-clock screen capture.

## Layout

```
src/
  lib.rs          public modules
  error.rs        RenderError
  models.rs       RenderJob, PixelFormat, EncodeBackend
  duration.rs     parse 10s / 5m / 2h / 1d
  cli.rs          clap surface
  pipeline.rs     load plugin, fixed-dt tick, raster frames
  encode.rs       spawn ffmpeg rawvideo → AV1
  main.rs         binary entry
```

## Data flow

```
RenderJob
  → PluginSession::load_path (CPU raster)
  → for frame: tick(1/fps); render(cols,rows,w,h) → BGRA
  → write raw frames to ffmpeg stdin
  → output.mkv
```

## Invariants

- Frame count = floor(duration_secs * fps) (at least 1 if duration > 0).
- Seed is exported as `IDLESCREEN_RENDER_SEED` / `TRANCE_SEED` for plugins that honor it.
- Encoder must be one of: libsvtav1, libaom-av1, librav1e, or raw null sink for tests.
