# Test matrix

| ID | Assertion |
|----|-----------|
| D1 | `1s` at 30fps → 30 frames |
| D2 | `2m` parses to 120 seconds |
| D3 | `0s` / empty duration is error |
| D4 | `10` bare number means seconds |
| J1 | RenderJob rejects zero width/height/fps |
| J2 | frame_count saturates reasonably |
| E1 | encode backend list includes libsvtav1 when ffmpeg present |
| P1 | pipeline dry-run without plugin path returns structured error |
