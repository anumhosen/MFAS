use crate::error::GpuError;
use serde::{Deserialize, Serialize};

/// Backend classification of an evaluator
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvaluatorBackend {
    Cpu,
    Simd,
    Gpu {
        adapter_name: String,
        backend: String,
    },
    Mock,
}

/// Request descriptor for batch page synthesis / evaluation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageEvalRequest {
    pub page_index: u64,
    pub logical_len: usize,
    pub pattern: String,
    pub seed: u64,
}

/// Information about a detected compute adapter or GPU device
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GpuDeviceInfo {
    pub adapter_name: String,
    pub backend: String,
    pub device_type: String,
    pub is_dedicated: bool,
    pub max_buffer_size: u64,
}

/// Extensible execution interface for CPU and GPU evaluators
pub trait Evaluator: Send + Sync {
    /// Human-readable identifier of the evaluator
    fn name(&self) -> &str;

    /// Architectural backend
    fn backend(&self) -> EvaluatorBackend;

    /// Returns true if this compute engine is currently operational on this machine
    fn is_available(&self) -> bool;

    /// Generate a stream of synthesized bytes according to a pattern
    fn generate_bytes(&self, pattern: &str, length: usize, seed: u64) -> Result<Vec<u8>, GpuError>;

    /// Evaluate a batch of logical pages in parallel
    fn batch_evaluate_pages(&self, requests: &[PageEvalRequest]) -> Result<Vec<Vec<u8>>, GpuError> {
        let mut results = Vec::with_capacity(requests.len());
        for req in requests {
            let bytes = self.generate_bytes(&req.pattern, req.logical_len, req.seed)?;
            results.push(bytes);
        }
        Ok(results)
    }
}
