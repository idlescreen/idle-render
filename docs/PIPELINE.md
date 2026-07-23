# Offline render pipeline

```
seed, effect, cols/rows, fps, duration, output
        │
        ▼
 load saver .so  →  ScreensaverInstance
        │
 for frame in 0..N:
        ├─ update(1/fps)     // synthetic time
        ├─ draw(grid)
        ├─ CellRenderer → BGRA
        ├─ scale to target resolution
        └─ encode (pipe to ffmpeg / SVT-AV1)
        │
        ▼
     output.mkv
```

Requirements on plugins: no wall-clock in the sim path; accept a fixed seed
(upstream API work in idle-core as needed).

Operational preference: segment long jobs (1h chunks + concat) rather than one
24h process.
