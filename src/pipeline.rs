use crate::audio::mux_audio_bed;
use crate::encode::{encode_raw_bgra_to_file, EncodeBackend};
use crate::error::RenderError;
use crate::models::RenderJob;
use crate::segment::{concat_segments, plan_segments};
use idle_runner::plugin_session::PluginSession;
use std::path::PathBuf;
use std::time::Duration;

/// Outcome of a finished (or dry-run) pipeline.
#[derive(Debug, Clone)]
pub struct PipelineResult {
    pub frames: u64,
    pub output: PathBuf,
    pub dry_run: bool,
    pub segments: u32,
}

/// Export seed env vars so plugins using [`idle_api::LcgRng::from_env_or_random`] match.
pub fn export_seed_env(seed: u64) {
    // SAFETY: single-threaded CLI before plugin load; values are numeric strings.
    unsafe {
        // Dual names: historical TRANCE_SEED / IDLE_RENDER_SEED + product RENDER_SEED.
        std::env::set_var("RENDER_SEED", seed.to_string());
        std::env::set_var("IDLE_RENDER_SEED", seed.to_string());
        std::env::set_var("TRANCE_SEED", seed.to_string());
        std::env::set_var("TRANCE_DISABLE_SANDBOX", "1");
    }
}

fn resolve_plugin(job: &RenderJob) -> Result<PluginSession, RenderError> {
    if let Some(path) = &job.plugin_path {
        return PluginSession::load_path_with_options(path, Some(false), Some(1.0))
            .map_err(|e| RenderError::Plugin(e.to_string()));
    }
    PluginSession::load_with_options(
        &job.effect,
        &idle_runner::launcher::LaunchMode::Preview,
        Some(false),
        Some(1.0),
    )
    .map_err(|e| RenderError::Plugin(e.to_string()))
}

#[allow(clippy::too_many_arguments)]
fn encode_one(
    session: &mut PluginSession,
    cols: usize,
    rows: usize,
    width: u32,
    height: u32,
    fps: u32,
    frames_total: u64,
    frame_offset: u64,
    output: &std::path::Path,
    backend: EncodeBackend,
) -> Result<u64, RenderError> {
    let dt = Duration::from_secs_f64(1.0 / f64::from(fps));
    let mut index = 0u64;
    let iter = std::iter::from_fn(|| {
        if index >= frames_total {
            return None;
        }
        session.tick(dt);
        let pixels = session.render(cols, rows, width, height);
        index += 1;
        let global = frame_offset + index;
        if global.is_multiple_of(30) || index == frames_total {
            tracing::info!(frame = global, "render progress");
        }
        Some(Ok(pixels))
    });
    encode_raw_bgra_to_file(backend, width, height, fps, output, iter)
}

/// Run the offline simulation and encode loop (optional segments + audio).
pub fn run_pipeline(
    job: &RenderJob,
    backend: EncodeBackend,
) -> Result<PipelineResult, RenderError> {
    job.validate()?;
    export_seed_env(job.seed);

    let plans = plan_segments(job)?;
    let frames_total = job.frame_count();
    if job.dry_run {
        return Ok(PipelineResult {
            frames: frames_total,
            output: job.output.clone(),
            dry_run: true,
            segments: plans.len() as u32,
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

    let mut written = 0u64;
    let mut frame_offset = 0u64;
    let mut part_paths = Vec::new();
    for plan in &plans {
        let part_frames = {
            let secs = plan.duration.as_secs_f64();
            let n = (secs * f64::from(job.fps)).floor() as u64;
            n.max(1)
        };
        let n = encode_one(
            &mut session,
            cols,
            rows,
            job.width,
            job.height,
            job.fps,
            part_frames,
            frame_offset,
            &plan.path,
            backend,
        )?;
        written += n;
        frame_offset += part_frames;
        part_paths.push(plan.path.clone());
        tracing::info!(segment = plan.index, frames = n, path = %plan.path.display(), "segment done");
    }

    if part_paths.len() > 1 {
        concat_segments(&part_paths, &job.output)?;
    } else if part_paths.len() == 1 && part_paths[0] != job.output {
        // should not happen for unsegmented
        concat_segments(&part_paths, &job.output)?;
    }

    if let Some(audio) = &job.audio {
        mux_audio_bed(&job.output, audio, &job.output)?;
    }

    Ok(PipelineResult {
        frames: written,
        output: job.output.clone(),
        dry_run: false,
        segments: plans.len() as u32,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn dry_run_reports_frame_count_and_segments() {
        let job = RenderJob {
            effect: "beams".into(),
            plugin_path: None,
            seed: 42,
            fps: 30,
            duration: Duration::from_secs(120),
            width: 64,
            height: 64,
            output: PathBuf::from("/tmp/unused.mkv"),
            cols: None,
            rows: None,
            dry_run: true,
            segment: Some(Duration::from_secs(60)),
            audio: None,
        };
        let r = run_pipeline(&job, EncodeBackend::RawDump).expect("dry");
        assert_eq!(r.frames, 3600);
        assert_eq!(r.segments, 2);
        assert!(r.dry_run);
    }
}
