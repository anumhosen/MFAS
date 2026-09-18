//! MFAS GPU Execution Module & Hardware Evaluator Abstraction
//!
//! Provides:
//! - Extensible `Evaluator` trait for parallel page generation and transforms
//! - `CpuEvaluator`: Optimized baseline CPU evaluation
//! - `SimdEvaluator`: Vectorized SIMD byte synthesis
//! - `GpuEvaluator`: Compute adapter discovery and execution with transparent fallback

pub mod cpu;
pub mod error;
pub mod evaluator;
pub mod gpu;

pub use cpu::{CpuEvaluator, SimdEvaluator};
pub use error::GpuError;
pub use evaluator::{Evaluator, EvaluatorBackend, GpuDeviceInfo, PageEvalRequest};
pub use gpu::GpuEvaluator;
