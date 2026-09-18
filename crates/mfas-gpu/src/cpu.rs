use crate::error::GpuError;
use crate::evaluator::{Evaluator, EvaluatorBackend, PageEvalRequest};

/// Standard CPU execution evaluator
#[derive(Debug, Clone, Default)]
pub struct CpuEvaluator;

impl CpuEvaluator {
    pub fn new() -> Self {
        Self
    }
}

impl Evaluator for CpuEvaluator {
    fn name(&self) -> &str {
        "Standard CPU Evaluator"
    }

    fn backend(&self) -> EvaluatorBackend {
        EvaluatorBackend::Cpu
    }

    fn is_available(&self) -> bool {
        true
    }

    fn generate_bytes(&self, pattern: &str, length: usize, seed: u64) -> Result<Vec<u8>, GpuError> {
        let mut buf = vec![0u8; length];
        match pattern {
            "zeros" => {
                buf.fill(0);
            }
            "counter" => {
                for (i, byte) in buf.iter_mut().enumerate() {
                    *byte = (i % 256) as u8;
                }
            }
            "repeat" => {
                let pat = [0x4D, 0x46, 0x41, 0x53]; // "MFAS" default
                for (i, byte) in buf.iter_mut().enumerate() {
                    *byte = pat[i % pat.len()];
                }
            }
            "random" => {
                let mut state = if seed == 0 { 0xDEADBEEFCAFEBABE } else { seed };
                for byte in buf.iter_mut() {
                    let mut x = state;
                    x ^= x << 13;
                    x ^= x >> 7;
                    x ^= x << 17;
                    state = x;
                    *byte = (x & 0xFF) as u8;
                }
            }
            other => return Err(GpuError::UnsupportedPattern(other.to_string())),
        }
        Ok(buf)
    }

    fn batch_evaluate_pages(&self, requests: &[PageEvalRequest]) -> Result<Vec<Vec<u8>>, GpuError> {
        let mut results = Vec::with_capacity(requests.len());
        for req in requests {
            results.push(self.generate_bytes(&req.pattern, req.logical_len, req.seed)?);
        }
        Ok(results)
    }
}

/// Vectorized SIMD-style chunked evaluator
#[derive(Debug, Clone, Default)]
pub struct SimdEvaluator;

impl SimdEvaluator {
    pub fn new() -> Self {
        Self
    }
}

impl Evaluator for SimdEvaluator {
    fn name(&self) -> &str {
        "SIMD Vectorized Evaluator"
    }

    fn backend(&self) -> EvaluatorBackend {
        EvaluatorBackend::Simd
    }

    fn is_available(&self) -> bool {
        true
    }

    fn generate_bytes(&self, pattern: &str, length: usize, seed: u64) -> Result<Vec<u8>, GpuError> {
        let mut buf = vec![0u8; length];

        match pattern {
            "zeros" => {
                // Bulk 64-bit zeroing
                let (prefix, words, suffix) = unsafe { buf.align_to_mut::<u64>() };
                prefix.fill(0);
                words.fill(0);
                suffix.fill(0);
            }
            "counter" => {
                for (i, byte) in buf.iter_mut().enumerate() {
                    *byte = (i % 256) as u8;
                }
            }
            "repeat" => {
                let pat_word = u64::from_ne_bytes([0x4D, 0x46, 0x41, 0x53, 0x4D, 0x46, 0x41, 0x53]);
                let (prefix, words, suffix) = unsafe { buf.align_to_mut::<u64>() };
                for (i, byte) in prefix.iter_mut().enumerate() {
                    *byte = [0x4D, 0x46, 0x41, 0x53][i % 4];
                }
                words.fill(pat_word);
                for (i, byte) in suffix.iter_mut().enumerate() {
                    *byte = [0x4D, 0x46, 0x41, 0x53][i % 4];
                }
            }
            "random" => {
                let mut state = if seed == 0 { 0xDEADBEEFCAFEBABE } else { seed };
                for byte in buf.iter_mut() {
                    let mut x = state;
                    x ^= x << 13;
                    x ^= x >> 7;
                    x ^= x << 17;
                    state = x;
                    *byte = (x & 0xFF) as u8;
                }
            }
            other => return Err(GpuError::UnsupportedPattern(other.to_string())),
        }

        Ok(buf)
    }
}
