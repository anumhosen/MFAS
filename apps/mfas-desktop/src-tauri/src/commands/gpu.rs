use mfas_gpu::cpu::{CpuEvaluator, SimdEvaluator};
use mfas_gpu::evaluator::{Evaluator, GpuDeviceInfo};
use mfas_gpu::gpu::GpuEvaluator;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfoDetailsDto {
    pub has_gpu: bool,
    pub adapter_name: String,
    pub backend: String,
    pub device_type: String,
    pub is_dedicated: bool,
    pub max_buffer_mb: u64,
    pub fallback_mode: bool,
    pub cpu_evaluator_ready: bool,
    pub simd_evaluator_ready: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluatorBenchmarkDto {
    pub backend: String,
    pub bytes_evaluated: usize,
    pub elapsed_ms: f64,
    pub throughput_mbps: f64,
    pub status: String,
}

#[tauri::command]
pub fn get_gpu_info() -> Result<GpuInfoDetailsDto, String> {
    let gpu = GpuEvaluator::new();
    let info = gpu.device_info().cloned().unwrap_or(GpuDeviceInfo {
        adapter_name: "No Dedicated Adapter Detected".to_string(),
        backend: "Software Fallback".to_string(),
        device_type: "CPU Fallback".to_string(),
        is_dedicated: false,
        max_buffer_size: 1024 * 1024 * 1024,
    });

    Ok(GpuInfoDetailsDto {
        has_gpu: gpu.is_available(),
        adapter_name: info.adapter_name,
        backend: info.backend,
        device_type: info.device_type,
        is_dedicated: info.is_dedicated,
        max_buffer_mb: info.max_buffer_size / (1024 * 1024),
        fallback_mode: !gpu.is_available(),
        cpu_evaluator_ready: true,
        simd_evaluator_ready: true,
    })
}

#[tauri::command]
pub fn run_evaluator_benchmark(backend: String, size_bytes: usize) -> Result<EvaluatorBenchmarkDto, String> {
    let test_size = if size_bytes == 0 { 16 * 1024 * 1024 } else { size_bytes.min(64 * 1024 * 1024) };
    let start = Instant::now();

    match backend.to_lowercase().as_str() {
        "simd" => {
            let simd = SimdEvaluator::new();
            let _ = simd
                .generate_bytes("counter", test_size, 0)
                .map_err(|e| e.to_string())?;
        }
        "gpu" => {
            let gpu = GpuEvaluator::new();
            let _ = gpu
                .generate_bytes("counter", test_size, 0)
                .map_err(|e| e.to_string())?;
        }
        _ => {
            let cpu = CpuEvaluator::new();
            let _ = cpu
                .generate_bytes("counter", test_size, 0)
                .map_err(|e| e.to_string())?;
        }
    }

    let elapsed = start.elapsed();
    let elapsed_ms = elapsed.as_secs_f64() * 1000.0;
    let seconds = elapsed.as_secs_f64().max(0.0001);
    let throughput_mbps = (test_size as f64 / (1024.0 * 1024.0)) / seconds;

    Ok(EvaluatorBenchmarkDto {
        backend,
        bytes_evaluated: test_size,
        elapsed_ms,
        throughput_mbps,
        status: "Completed".to_string(),
    })
}
