//! Offline IdleScreen render library (Build 0+ segments/audio).

pub mod audio;
pub mod cli;
pub mod duration;
pub mod encode;
pub mod error;
pub mod models;
pub mod pipeline;
pub mod segment;

pub use duration::parse_duration_secs;
pub use encode::{encode_raw_bgra_to_file, EncodeBackend};
pub use error::RenderError;
pub use models::RenderJob;
pub use pipeline::{run_pipeline, PipelineResult};
