//! Offline IdleScreen renderer (scaffold).
//!
//! Target pipeline:
//! load saver plugin → fixed-dt update/draw → raster cells → encode (ffmpeg/AV1).

use clap::Parser;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Debug, Parser)]
#[command(
    name = "idle-render",
    about = "Offline IdleScreen effect renderer (scaffold)",
    long_about = "Steps saver plugins offline and encodes video. \
Full plugin load + AV1 path is not wired yet; this binary defines the CLI surface."
)]
struct Args {
    /// Effect name (e.g. beams, storm) or path to plugin .so
    #[arg(long, short = 'e')]
    effect: String,

    /// RNG seed for deterministic output
    #[arg(long, default_value_t = 0xC0FF_EEu64)]
    seed: u64,

    /// Frames per second of the output timeline
    #[arg(long, default_value_t = 30)]
    fps: u32,

    /// Duration, e.g. 10s, 5m, 2h (parsed in a later milestone)
    #[arg(long, default_value = "10s")]
    duration: String,

    /// Output path (.mkv / .mp4 / .webm)
    #[arg(long, short = 'o')]
    output: PathBuf,

    /// Pixel width (letterbox/scale target)
    #[arg(long, default_value_t = 1920)]
    width: u32,

    /// Pixel height
    #[arg(long, default_value_t = 1080)]
    height: u32,
}

fn main() -> ExitCode {
    let args = Args::parse();
    eprintln!("idle-render scaffold — export pipeline not implemented yet");
    eprintln!(
        "planned: effect={} seed={} fps={} duration={} {}x{} -> {}",
        args.effect,
        args.seed,
        args.fps,
        args.duration,
        args.width,
        args.height,
        args.output.display()
    );
    eprintln!("see docs/PIPELINE.md and https://github.com/idlescreen/idle-pro");
    ExitCode::from(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_parses_minimal() {
        let a = Args::parse_from(["idle-render", "--effect", "beams", "-o", "/tmp/out.mkv"]);
        assert_eq!(a.effect, "beams");
        assert_eq!(a.fps, 30);
    }
}
