# Phase 11 Implementation Plan: Benchmarks & Performance Metrics

In this phase, we implement the **MFAS Benchmarking Suite** measuring CPU operations/sec, memory efficiency, I/O streaming throughput, and DAG optimization performance across the mathematical pipeline.

---

## Proposed Changes

### Core Library: `crates/mfas-core`

#### [NEW] [bench/mod.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-core/src/bench/mod.rs)
- Modular performance benchmarking functions:
  - `bench_address(iterations: u64) -> AddressBenchResult`: Measures URI parsing, URI formatting, and LEB128 binary roundtrips in operations/sec and nanoseconds/op.
  - `bench_page(iterations: u64) -> PageBenchResult`: Measures Radix-16 spatial hierarchy decomposition and page instantiation throughput.
  - `bench_codec(size_bytes: u64) -> CodecBenchResult`: Measures constant-memory streaming encoding and decoding throughput (MB/s) and time elapsed.
  - `bench_dag(nodes: usize) -> DagBenchResult`: Measures DAG construction, topological metric calculation, and CSE optimization throughput (nodes/sec).

#### [MODIFY] [lib.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-core/src/lib.rs)
- Expose `pub mod bench;` and re-export benchmark types.

---

### CLI Interface: `crates/mfas-cli`

#### [NEW] [commands/bench.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-cli/src/commands/bench.rs)
- Subcommand `mfas bench`:
  - `mfas bench [all | address | page | codec | dag]`
  - `--iterations <N>`: Iterations for micro-benchmarks (default: 50,000).
  - `--size <bytes>`: Data stream size for codec throughput benchmarks (default: 10 MiB).
  - Terminal table output with styled metric summaries and full `--json` support.

#### [MODIFY] [commands/mod.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-cli/src/commands/mod.rs)
- Register `pub mod bench;`.

#### [MODIFY] [main.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-cli/src/main.rs)
- Add `Commands::Bench(BenchArgs)` variant and routing.

---

### Verification & Testing

#### [NEW] [crates/mfas-cli/tests/cli_bench_tests.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-cli/tests/cli_bench_tests.rs)
- Automated CLI integration tests:
  - `test_cli_bench_address_json`: Verifies JSON metric schemas (ops/sec, ns/op).
  - `test_cli_bench_codec_json`: Verifies throughput calculation on a 1MB stream.
  - `test_cli_bench_all_human`: Verifies human-readable summary table output.

#### Core Unit Tests
- In `crates/mfas-core/src/bench/mod.rs`: Unit tests verifying benchmark runner execution and reporting.

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
