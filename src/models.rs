use crate::error::RenderError;
use crate::paths::deny_parent_dirs;
use std::path::PathBuf;
use std::time::Duration;

/// Pixel layout produced by the cell renderer (BGRA8888).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    Bgra8888,
}

impl PixelFormat {
    pub const fn bytes_per_pixel(self) -> usize {
        4
    }
}

/// Fully validated offline render request.
#[derive(Debug, Clone)]
pub struct RenderJob {
    pub effect: String,
    pub plugin_path: Option<PathBuf>,
    pub seed: u64,
    pub fps: u32,
    pub duration: Duration,
    pub width: u32,
    pub height: u32,
    pub output: PathBuf,
    pub cols: Option<usize>,
    pub rows: Option<usize>,
    pub dry_run: bool,
    /// When set, encode in chunks of this length then concat.
    pub segment: Option<Duration>,
    /// Optional audio bed muxed after video (loop/shorten to video length).
    pub audio: Option<PathBuf>,
}

impl RenderJob {
    pub fn frame_count(&self) -> u64 {
        let secs = self.duration.as_secs_f64();
        if secs <= 0.0 || self.fps == 0 {
            return 0;
        }
        let n = (secs * f64::from(self.fps)).floor() as u64;
        n.max(1)
    }

    /// Number of segment files for a segmented encode (1 if no segment).
    pub fn segment_count(&self) -> u64 {
        let Some(seg) = self.segment else {
            return 1;
        };
        if seg.is_zero() {
            return 1;
        }
        let total = self.duration.as_secs_f64();
        let part = seg.as_secs_f64();
        if part <= 0.0 {
            return 1;
        }
        ((total / part).ceil() as u64).max(1)
    }

    pub fn validate(&self) -> Result<(), RenderError> {
        if self.fps == 0 {
            return Err(RenderError::Job("fps must be > 0".into()));
        }
        if self.width == 0 || self.height == 0 {
            return Err(RenderError::Job("width and height must be > 0".into()));
        }
        if self.duration.is_zero() {
            return Err(RenderError::Job("duration must be > 0".into()));
        }
        if self.effect.trim().is_empty() && self.plugin_path.is_none() {
            return Err(RenderError::Job(
                "effect name or --plugin-path required".into(),
            ));
        }
        if self.output.as_os_str().is_empty() {
            return Err(RenderError::Job("output path required".into()));
        }
        deny_parent_dirs(&self.output, "output")?;
        if let Some(p) = &self.plugin_path {
            deny_parent_dirs(p, "plugin_path")?;
        }
        if let Some(a) = &self.audio {
            deny_parent_dirs(a, "audio")?;
        }
        if let Some(seg) = self.segment {
            if seg.is_zero() {
                return Err(RenderError::Job("segment duration must be > 0".into()));
            }
            if seg >= self.duration {
                // single segment is fine — treat as unsegmented
            }
        }
        if let Some(a) = &self.audio {
            if !a.as_os_str().is_empty() && a.extension().is_none() {
                // allow extensionless; only reject empty
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RenderJob {
        RenderJob {
            effect: "beams".into(),
            plugin_path: None,
            seed: 1,
            fps: 30,
            duration: Duration::from_secs(1),
            width: 320,
            height: 180,
            output: PathBuf::from("/tmp/out.mkv"),
            cols: None,
            rows: None,
            dry_run: true,
            segment: None,
            audio: None,
        }
    }

    #[test]
    fn frame_count_one_second_30fps() {
        assert_eq!(sample().frame_count(), 30);
    }

    #[test]
    fn segment_count_two_hours_hourly() {
        let mut j = sample();
        j.duration = Duration::from_secs(7200);
        j.segment = Some(Duration::from_secs(3600));
        assert_eq!(j.segment_count(), 2);
    }

    #[test]
    fn rejects_zero_fps() {
        let mut j = sample();
        j.fps = 0;
        assert!(j.validate().is_err());
    }

    #[test]
    fn rejects_zero_size() {
        let mut j = sample();
        j.width = 0;
        assert!(j.validate().is_err());
    }
}
