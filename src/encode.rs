use crate::error::RenderError;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Preferred ffmpeg AV1 encoder names, in order.
pub const AV1_CANDIDATES: &[&str] = &["libsvtav1", "libaom-av1", "librav1e"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodeBackend {
    /// Pipe BGRA frames to ffmpeg AV1.
    FfmpegAv1,
    /// Write a single raw BGRA dump (tests / no ffmpeg).
    RawDump,
}

/// Pick first available AV1 encoder from `ffmpeg -encoders`.
pub fn detect_av1_encoder() -> Result<String, RenderError> {
    let out = Command::new("ffmpeg")
        .args(["-hide_banner", "-encoders"])
        .output()
        .map_err(|_| RenderError::FfmpegMissing)?;
    if !out.status.success() {
        return Err(RenderError::FfmpegMissing);
    }
    let text = String::from_utf8_lossy(&out.stdout);
    for name in AV1_CANDIDATES {
        if text.contains(name) {
            return Ok((*name).to_string());
        }
    }
    Err(RenderError::Ffmpeg(
        "no AV1 encoder (libsvtav1/libaom-av1/librav1e) found".into(),
    ))
}

/// Encode a sequence of BGRA frames by spawning ffmpeg (or raw dump).
pub fn encode_raw_bgra_to_file(
    backend: EncodeBackend,
    width: u32,
    height: u32,
    fps: u32,
    output: &Path,
    mut frames: impl Iterator<Item = Result<Vec<u8>, RenderError>>,
) -> Result<u64, RenderError> {
    match backend {
        EncodeBackend::RawDump => write_raw_dump(output, &mut frames),
        EncodeBackend::FfmpegAv1 => write_ffmpeg_av1(width, height, fps, output, &mut frames),
    }
}

fn write_raw_dump(
    output: &Path,
    frames: &mut dyn Iterator<Item = Result<Vec<u8>, RenderError>>,
) -> Result<u64, RenderError> {
    let mut file = std::fs::File::create(output).map_err(|source| RenderError::Io {
        path: output.to_path_buf(),
        source,
    })?;
    let mut n = 0u64;
    for frame in frames {
        let buf = frame?;
        file.write_all(&buf).map_err(|source| RenderError::Io {
            path: output.to_path_buf(),
            source,
        })?;
        n += 1;
    }
    if n == 0 {
        return Err(RenderError::EmptyOutput);
    }
    Ok(n)
}

fn write_ffmpeg_av1(
    width: u32,
    height: u32,
    fps: u32,
    output: &Path,
    frames: &mut dyn Iterator<Item = Result<Vec<u8>, RenderError>>,
) -> Result<u64, RenderError> {
    let encoder = detect_av1_encoder()?;
    let size = format!("{width}x{height}");
    let mut child = Command::new("ffmpeg")
        .args([
            "-hide_banner",
            "-loglevel",
            "error",
            "-y",
            "-f",
            "rawvideo",
            "-pix_fmt",
            "bgra",
            "-s",
            &size,
            "-r",
            &fps.to_string(),
            "-i",
            "-",
            "-an",
            "-c:v",
            &encoder,
            "-crf",
            "35",
            "-pix_fmt",
            "yuv420p",
        ])
        .arg(output)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| RenderError::Ffmpeg(e.to_string()))?;

    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| RenderError::Ffmpeg("stdin missing".into()))?;

    let mut n = 0u64;
    for frame in frames {
        let buf = frame?;
        if let Err(e) = stdin.write_all(&buf) {
            // Broken pipe if ffmpeg exited early — collect status below.
            let _ = e;
            break;
        }
        n += 1;
    }
    drop(stdin);

    let out = child
        .wait_with_output()
        .map_err(|e| RenderError::Ffmpeg(e.to_string()))?;
    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr);
        return Err(RenderError::Ffmpeg(format!(
            "exit {:?}: {err}",
            out.status.code()
        )));
    }
    if n == 0 {
        return Err(RenderError::EmptyOutput);
    }
    let _path: PathBuf = output.to_path_buf();
    Ok(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn candidates_nonempty() {
        assert!(!AV1_CANDIDATES.is_empty());
    }
}
