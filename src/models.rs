use crate::error::RenderError;
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
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

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
        }
    }

    #[test]
    fn frame_count_one_second_30fps() {
        assert_eq!(sample().frame_count(), 30);
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
