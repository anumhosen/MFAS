use mfas_core::CoreError;
use thiserror::Error;

/// Errors arising during GPU/CPU hardware evaluation and execution
#[derive(Error, Debug)]
pub enum GpuError {
    #[error("No compatible compute adapter or GPU device found")]
    AdapterNotFound,

    #[error("Compute device is unavailable: {0}")]
    DeviceUnavailable(String),

    #[error("Unsupported generation pattern: {0}")]
    UnsupportedPattern(String),

    #[error("Compute execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Underlying core error: {0}")]
    Core(#[from] CoreError),
}
