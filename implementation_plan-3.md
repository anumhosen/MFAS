# Phase 12 Implementation Plan: GPU Interface & Evaluator Abstraction

In this phase, we develop **`crates/mfas-gpu`**, implementing the extensible **`Evaluator` abstraction**, **`CpuEvaluator`**, **`GpuEvaluator`**, **`SimdEvaluator`**, batch page generation pipelines, hardware detection with graceful CPU fallback, and the **`mfas gpu`** CLI subcommand.

---

## Proposed Changes

### Core GPU Crate: `crates/mfas-gpu`

#### [MODIFY] [Cargo.toml](file:///d:/Development/TAURI/MFAS/crates/mfas-gpu/Cargo.toml)
- Add dependencies: `serde`, `thiserror`.

#### [MODIFY] [src/lib.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-gpu/src/lib.rs)
- Modular architecture:
  - `pub mod error;`
  - `pub mod evaluator;`
  - `pub mod cpu;`
  - `pub mod gpu;`
- Expose key types:
  - `Evaluator` trait
  - `EvaluatorBackend` enum (`Cpu`, `Gpu`, `Simd`, `Mock`)
  - `CpuEvaluator`, `GpuEvaluator`, `SimdEvaluator`
  - `PageEvalRequest`, `GpuDeviceInfo`, `GpuError`

#### [NEW] [src/error.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-gpu/src/error.rs)
- Structured error handling:
  - `AdapterNotFound`
  - `DeviceUnavailable(String)`
  - `UnsupportedPattern(String)`
  - `ExecutionFailed(String)`
  - `Core(CoreError)`

#### [NEW] [src/evaluator.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-gpu/src/evaluator.rs)
- Common `Evaluator` interface:
  - `name(&self) -> &str`
  - `backend(&self) -> EvaluatorBackend`
  - `is_available(&self) -> bool`
  - `generate_bytes(&self, pattern: &str, length: usize, seed: u64) -> Result<Vec<u8>, GpuError>`
  - `batch_evaluate_pages(&self, requests: &[PageEvalRequest]) -> Result<Vec<Vec<u8>>, GpuError>`
- `PageEvalRequest`:
  - `page_index: u64`
  - `logical_len: usize`
  - `pattern: String`
  - `seed: u64`

#### [NEW] [src/cpu.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-gpu/src/cpu.rs)
- `CpuEvaluator`: High-speed baseline CPU generator and batch parallel evaluator.
- `SimdEvaluator`: Vectorized SIMD byte generator.

#### [NEW] [src/gpu.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-gpu/src/gpu.rs)
- `GpuEvaluator`: Compute-device abstraction with device detection:
  - Identifies available graphics adapters, vendor, driver backend (DirectX 12, Vulkan, Metal).
  - Graceful fallback: when dedicated compute hardware is not active, cleanly routes to `CpuEvaluator` with notification.

---

### CLI Interface: `crates/mfas-cli`

#### [MODIFY] [Cargo.toml](file:///d:/Development/TAURI/MFAS/crates/mfas-cli/Cargo.toml)
- Add dependency `mfas-gpu = { path = "../mfas-gpu" }`.

#### [NEW] [commands/gpu.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-cli/src/commands/gpu.rs)
- Subcommand `mfas gpu`:
  - `mfas gpu info`: Displays detected GPU compute adapters, driver backends, device capabilities, and fallback status.
  - `mfas gpu bench [--size <bytes>]`: Compares CPU vs SIMD vs GPU evaluator throughput (MB/s).

#### [MODIFY] [commands/mod.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-cli/src/commands/mod.rs)
- Register `pub mod gpu;`.

#### [MODIFY] [main.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-cli/src/main.rs)
- Add `Commands::Gpu(GpuArgs)` variant and routing.

---

### Verification & Testing

#### [NEW] [crates/mfas-gpu/tests/evaluator_tests.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-gpu/tests/evaluator_tests.rs)
- Unit tests:
  - `test_cpu_evaluator_generation`: Verifies byte generation and batch evaluation.
  - `test_simd_evaluator_generation`: Verifies SIMD output equality with CPU output.
  - `test_gpu_evaluator_fallback`: Verifies graceful fallback to CPU on unavailable hardware.
  - `test_batch_page_evaluation`: Tests concurrent multi-page evaluation.

#### [NEW] [crates/mfas-cli/tests/cli_gpu_tests.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-cli/tests/cli_gpu_tests.rs)
- Integration tests:
  - `test_cli_gpu_info_json`: Verifies `mfas gpu info --json`.
  - `test_cli_gpu_bench_json`: Verifies `mfas gpu bench --json`.

---

## Verification Plan

### Automated Tests
Run:
```bash
cargo test --workspace
```
Strictly complying with:
- **Never run `cargo check`**.
- **Never delete files automatically**.
