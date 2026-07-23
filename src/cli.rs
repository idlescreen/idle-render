use crate::duration::parse_duration_secs;
use crate::encode::EncodeBackend;
use crate::error::RenderError;
use crate::models::RenderJob;
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "idle-render",
    about = "Offline IdleScreen effect renderer (saver math → AV1)"
)]
pub struct Args {
    /// Effect name (allowlisted saver basename, e.g. beams)
    #[arg(long, short = 'e')]
    pub effect: String,

    /// Explicit path to plugin .so (skips discovery)
    #[arg(long)]
    pub plugin_path: Option<PathBuf>,

    /// RNG seed exported to plugins via IDLE_RENDER_SEED
    #[arg(long, default_value_t = 0xC0FF_EEu64)]
    pub seed: u64,

    /// Output timeline fps
    #[arg(long, default_value_t = 30)]
    pub fps: u32,

    /// Duration: 10s, 5m, 2h, 1d (or bare seconds)
    #[arg(long, default_value = "10s")]
    pub duration: String,

    /// Output path (.mkv recommended)
    #[arg(long, short = 'o')]
    pub output: PathBuf,

    /// Pixel width
    #[arg(long, default_value_t = 1280)]
    pub width: u32,

    /// Pixel height
    #[arg(long, default_value_t = 720)]
    pub height: u32,

    /// Optional simulation grid columns
    #[arg(long)]
    pub cols: Option<usize>,

    /// Optional simulation grid rows
    #[arg(long)]
    pub rows: Option<usize>,

    /// Validate and print plan only
    #[arg(long)]
    pub dry_run: bool,

    /// Write raw BGRA dump instead of AV1 (debug/tests)
    #[arg(long)]
    pub raw: bool,
}

impl Args {
    pub fn into_job(self) -> Result<(RenderJob, EncodeBackend), RenderError> {
        let duration = parse_duration_secs(&self.duration)?;
        let job = RenderJob {
            effect: self.effect,
            plugin_path: self.plugin_path,
            seed: self.seed,
            fps: self.fps,
            duration,
            width: self.width,
            height: self.height,
            output: self.output,
            cols: self.cols,
            rows: self.rows,
            dry_run: self.dry_run,
        };
        job.validate()?;
        let backend = if self.raw || self.dry_run {
            EncodeBackend::RawDump
        } else {
            EncodeBackend::FfmpegAv1
        };
        Ok((job, backend))
    }
}
