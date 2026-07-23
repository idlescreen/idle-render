use crate::encode::{encode_raw_bgra_to_file, EncodeBackend};
use crate::error::RenderError;
use crate::models::RenderJob;
use std::path::PathBuf;
use std::time::Duration;
use trance_runner::plugin_session::PluginSession;

/// Outcome of a finished (or dry-run) pipeline.
#[derive(Debug, Clone)]
pub struct PipelineResult {
    pub frames: u64,
    pub output: PathBuf,
    pub dry_run: bool,
}

/// Export seed env vars so plugins using [`trance_api::LcgRng::from_env_or_random`] match.
pub fn export_seed_env(seed: u64) {
    // SAFETY: single-threaded CLI before plugin load; values are numeric strings.
    unsafe {
        std::env::set_var("IDLE_RENDER_SEED", seed.to_string());
        std::env::set_var("TRANCE_SEED", seed.to_string());
    }
}

fn resolve_plugin(job: &RenderJob) -> Result<PluginSession, RenderError> {
    // Force CPU path for headless determinism.
    if let Some(path) = &job.plugin_path {
        return PluginSession::load_path_with_options(path, Some(false), Some(1.0))
            .map_err(|e| RenderError::Plugin(e.to_string()));
    }
    PluginSession::load_with_options(
        &job.effect,
        &trance_runner::launcher::LaunchMode::Preview,
        Some(false),
        Some(1.0),
    )
    .map_err(|e| RenderError::Plugin(e.to_string()))
}

/// Run the offline simulation and encode loop.
pub fn run_pipeline(
    job: &RenderJob,
    backend: EncodeBackend,
) -> Result<PipelineResult, RenderError> {
    job.validate()?;
    export_seed_env(job.seed);

    let frames_total = job.frame_count();
    if job.dry_run {
        return Ok(PipelineResult {
            frames: frames_total,
            output: job.output.clone(),
            dry_run: true,
        });
    }

    let mut session = resolve_plugin(job)?;
    let (cols, rows) = match (job.cols, job.rows) {
        (Some(c), Some(r)) => (c, r),
        _ => session.grid_for_pixels(job.width, job.height),
    };
    if cols == 0 || rows == 0 {
        return Err(RenderError::Job("grid cols/rows resolved to zero".into()));
    }
    session.init(cols, rows);
    session.set_simulation_rate(job.fps as f32);

    let dt = Duration::from_secs_f64(1.0 / f64::from(job.fps));
    let width = job.width;
    let height = job.height;
    let mut index = 0u64;

    let iter = std::iter::from_fn(|| {
        if index >= frames_total {
            return None;
        }
        session.tick(dt);
        let pixels = session.render(cols, rows, width, height);
        index += 1;
        if index.is_multiple_of(30) || index == frames_total {
            tracing::info!(frame = index, total = frames_total, "render progress");
        }
        Some(Ok(pixels))
    });

    let written = encode_raw_bgra_to_file(backend, width, height, job.fps, &job.output, iter)?;
    Ok(PipelineResult {
        frames: written,
        output: job.output.clone(),
        dry_run: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn dry_run_reports_frame_count() {
        let job = RenderJob {
            effect: "beams".into(),
            plugin_path: None,
            seed: 42,
            fps: 30,
            duration: Duration::from_secs(1),
            width: 64,
            height: 64,
            output: PathBuf::from("/tmp/unused.mkv"),
            cols: None,
            rows: None,
            dry_run: true,
        };
        let r = run_pipeline(&job, EncodeBackend::RawDump).expect("dry");
        assert_eq!(r.frames, 30);
        assert!(r.dry_run);
    }
}
