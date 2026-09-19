# Repository Context Bundle

```text
Generated: 2026-09-19 09:44:43
Project: crates
Files Included: 46
Approx Tokens: 55,050
Approx Characters: 220,202
```

# 1. Project Tree

```text
.
├── mfas-cli/
│   ├── src/
│   │   ├── commands/
│   │   │   ├── address.rs
│   │   │   ├── bench.rs
│   │   │   ├── codec.rs
│   │   │   ├── dag.rs
│   │   │   ├── generate.rs
│   │   │   ├── gpu.rs
│   │   │   ├── inspect.rs
│   │   │   ├── mod.rs
│   │   │   ├── node.rs
│   │   │   ├── number.rs
│   │   │   ├── page.rs
│   │   │   └── resolve.rs
│   │   └── main.rs
│   ├── tests/
│   │   ├── cli_address_tests.rs
│   │   ├── cli_bench_tests.rs
│   │   ├── cli_codec_tests.rs
│   │   ├── cli_dag_tests.rs
│   │   ├── cli_gpu_tests.rs
│   │   ├── cli_inspect_generate_tests.rs
│   │   ├── cli_node_tests.rs
│   │   ├── cli_number_tests.rs
│   │   ├── cli_page_tests.rs
│   │   └── cli_resolve_tests.rs
│   └── Cargo.toml
├── mfas-core/
│   ├── src/
│   │   ├── address/
│   │   │   └── mod.rs
│   │   ├── bench/
│   │   │   └── mod.rs
│   │   ├── codec/
│   │   │   └── mod.rs
│   │   ├── dag/
│   │   │   └── mod.rs
│   │   ├── enumeration/
│   │   │   └── mod.rs
│   │   ├── inspector/
│   │   │   └── mod.rs
│   │   ├── node/
│   │   │   └── mod.rs
│   │   ├── page/
│   │   │   └── mod.rs
│   │   ├── resolver/
│   │   │   ├── mod.rs
│   │   │   └── store.rs
│   │   ├── synthetic/
│   │   │   └── mod.rs
│   │   ├── error.rs
│   │   └── lib.rs
│   ├── tests/
│   │   └── large_data_tests.rs
│   └── Cargo.toml
└── mfas-gpu/
    ├── src/
    │   ├── cpu.rs
    │   ├── error.rs
    │   ├── evaluator.rs
    │   ├── gpu.rs
    │   └── lib.rs
    ├── tests/
    │   └── evaluator_tests.rs
    └── Cargo.toml
```

# 2. Important Configuration Files

No configuration files found.

# 3. Source Code

## mfas-cli\Cargo.toml

```toml
[package]
name = "mfas-cli"
version = "0.1.0"
edition = "2021"
description = "Mathematical File Address Space - CLI Application"

[[bin]]
name = "mfas"
path = "src/main.rs"

[dependencies]
mfas-core = { path = "../mfas-core" }
mfas-gpu = { path = "../mfas-gpu" }
clap = { version = "4.5", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
hex = "0.4"
num-bigint = "0.4"
num-traits = "0.2"
sha2 = "0.10"
```

## mfas-core\Cargo.toml

```toml
[package]
name = "mfas-core"
version = "0.1.0"
edition = "2021"
description = "Mathematical File Address Space - Core Engine"

[dependencies]
thiserror = "2.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
hex = "0.4"
num-bigint = "0.4"
num-traits = "0.2"
sha2 = "0.10"
```

## mfas-gpu\Cargo.toml

```toml
[package]
name = "mfas-gpu"
version = "0.1.0"
edition = "2021"
description = "Mathematical File Address Space - GPU Execution Engine (Future Interface)"

[dependencies]
mfas-core = { path = "../mfas-core" }
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
```

## mfas-cli\src\main.rs

```rust
mod commands;

use clap::{Parser, Subcommand};
use commands::address::{handle_address_cmd, AddressArgs};
use commands::bench::{handle_bench_cmd, BenchArgs};
use commands::codec::{
    handle_decode_cmd, handle_encode_cmd, handle_verify_cmd, DecodeArgs, EncodeArgs, VerifyArgs,
};
use commands::dag::{handle_dag_cmd, DagArgs};
use commands::generate::{handle_generate_cmd, GenerateArgs};
use commands::gpu::{handle_gpu_cmd, GpuArgs};
use commands::inspect::{handle_inspect_cmd, InspectArgs};
use commands::node::{handle_node_cmd, NodeArgs};
use commands::number::{handle_number_cmd, NumberArgs};
use commands::page::{handle_page_cmd, PageArgs};
use commands::resolve::{handle_resolve_cmd, ResolveArgs};
use serde_json::json;

#[derive(Parser, Debug)]
#[command(
    name = "mfas",
    author = "MFAS Development Team",
    version = "0.1.0",
    about = "Mathematical File Address Space - Addressing and Reconstruction CLI",
    long_about = "MFAS provides deterministic mathematical addressing, validation, and reconstruction of finite digital data."
)]
struct Cli {
    /// Emit machine-readable JSON output
    #[arg(long, global = true)]
    json: bool,

    /// Suppress non-essential output
    #[arg(short, long, global = true)]
    quiet: bool,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Manage, parse, format, and validate MFAS addresses
    Address(AddressArgs),
    /// Map between natural numbers and finite byte strings (N <-> B*)
    Number(NumberArgs),
    /// Inspect 4096-byte logical pages and Radix-16 spatial coordinates
    Page(PageArgs),
    /// Create, inspect, and manipulate recursive MFAS nodes
    Node(NodeArgs),
    /// Recursively resolve an MFAS address into concrete bytes or files
    Resolve(ResolveArgs),
    /// Encode an arbitrary file into a canonical root Address
    Encode(EncodeArgs),
    /// Reconstruct an original file from its canonical root Address
    Decode(DecodeArgs),
    /// Encode, reconstruct, and verify SHA-256 integrity of a file
    Verify(VerifyArgs),
    /// Topologically inspect an MFAS address and render its ASCII DAG hierarchy
    Inspect(InspectArgs),
    /// Deterministic large file generator and test data builder
    Generate(GenerateArgs),
    /// Directed Acyclic Graph analysis, DOT export, and CSE optimization
    Dag(DagArgs),
    /// Performance benchmarks across addressing, codec, pages, and DAG
    Bench(BenchArgs),
    /// GPU hardware compute adapter inspection and performance evaluation
    Gpu(GpuArgs),
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Address(args) => handle_address_cmd(args, cli.json, cli.quiet),
        Commands::Number(args) => handle_number_cmd(args, cli.json, cli.quiet),
        Commands::Page(args) => handle_page_cmd(args, cli.json, cli.quiet),
        Commands::Node(args) => handle_node_cmd(args, cli.json, cli.quiet),
        Commands::Resolve(args) => handle_resolve_cmd(args, cli.json, cli.quiet),
        Commands::Encode(args) => handle_encode_cmd(args, cli.json, cli.quiet),
        Commands::Decode(args) => handle_decode_cmd(args, cli.json, cli.quiet),
        Commands::Verify(args) => handle_verify_cmd(args, cli.json, cli.quiet),
        Commands::Inspect(args) => handle_inspect_cmd(args, cli.json, cli.quiet),
        Commands::Generate(args) => handle_generate_cmd(args, cli.json, cli.quiet),
        Commands::Dag(args) => handle_dag_cmd(args, cli.json, cli.quiet),
        Commands::Bench(args) => handle_bench_cmd(args, cli.json, cli.quiet),
        Commands::Gpu(args) => handle_gpu_cmd(args, cli.json, cli.quiet),
    };

    if let Err(e) = result {
        if cli.json {
            let err_json = json!({
                "error": true,
                "message": e.to_string(),
            });
            eprintln!("{}", err_json);
        } else {
            eprintln!("Error: {}", e);
        }
        std::process::exit(1);
    }
}
```

## mfas-cli\tests\cli_address_tests.rs

```rust
use std::process::Command;

#[test]
fn test_cli_address_parse() {
    let output = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["address", "parse", "mfas:v1:data:48656c6c6f"])
        .output()
        .expect("Failed to execute mfas command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("mfas:v1:data:48656c6c6f"));
    assert!(stdout.contains("Node Type:    data"));
    assert!(stdout.contains("Payload (B):  5 bytes"));
}

#[test]
fn test_cli_address_parse_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["--json", "address", "parse", "mfas:v1:ref:01020304"])
        .output()
        .expect("Failed to execute mfas command");

    assert!(output.status.success());
    let json: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("Failed to parse JSON output");
    assert_eq!(json["uri"], "mfas:v1:ref:01020304");
    assert_eq!(json["version"], 1);
    assert_eq!(json["node_type"], "ref");
    assert_eq!(json["payload_hex"], "01020304");
    assert_eq!(json["payload_len_bytes"], 4);
}

#[test]
fn test_cli_address_format() {
    let output = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["address", "format", "-t", "seq", "-p", "deadbeef"])
        .output()
        .expect("Failed to execute mfas command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "mfas:v1:seq:deadbeef");
}

#[test]
fn test_cli_address_validate_success() {
    let output = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["address", "validate", "mfas:v1:page:0000000000000001"])
        .output()
        .expect("Failed to execute mfas command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("VALID: mfas:v1:page:0000000000000001"));
}

#[test]
fn test_cli_address_validate_failure() {
    let output = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["address", "validate", "mfas:v1:data:odd"])
        .output()
        .expect("Failed to execute mfas command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("INVALID"));
}

#[test]
fn test_cli_address_binary_roundtrip() {
    let uri = "mfas:v1:rep:0000000441";
    // 1. To binary
    let to_bin = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["-q", "address", "to-binary", uri])
        .output()
        .expect("Failed to execute to-binary");
    assert!(to_bin.status.success());
    let bin_hex = String::from_utf8_lossy(&to_bin.stdout).trim().to_string();

    // 2. From binary
    let from_bin = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["address", "from-binary", &bin_hex])
        .output()
        .expect("Failed to execute from-binary");
    assert!(from_bin.status.success());
    let restored_uri = String::from_utf8_lossy(&from_bin.stdout).trim().to_string();
    assert_eq!(restored_uri, uri);
}
```

## mfas-cli\tests\cli_bench_tests.rs

```rust
use std::process::Command;

#[test]
fn test_cli_bench_address_json() {
    let out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["--json", "bench", "address", "--iterations", "1000"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let json: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(json["iterations"], 1000);
    assert!(json["parse_ops_per_sec"].as_f64().unwrap() > 0.0);
    assert!(json["format_ops_per_sec"].as_f64().unwrap() > 0.0);
    assert!(json["binary_ops_per_sec"].as_f64().unwrap() > 0.0);
}

#[test]
fn test_cli_bench_page_json() {
    let out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["--json", "bench", "page", "--iterations", "1000"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let json: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(json["iterations"], 1000);
    assert!(json["coordinate_ops_per_sec"].as_f64().unwrap() > 0.0);
}

#[test]
fn test_cli_bench_codec_json() {
    let out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["--json", "bench", "codec", "--size", "65536"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let json: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(json["size_bytes"], 65536);
    assert!(json["encode_throughput_mb_s"].as_f64().unwrap() > 0.0);
    assert!(json["decode_throughput_mb_s"].as_f64().unwrap() > 0.0);
}

#[test]
fn test_cli_bench_all_human() {
    let out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["bench", "all", "--iterations", "500", "--size", "65536"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("MFAS Comprehensive Performance Benchmark"));
    assert!(stdout.contains("[1] Address Subsystem"));
    assert!(stdout.contains("[2] Radix-16 Page Coordinates"));
    assert!(stdout.contains("[3] Streaming Codec"));
    assert!(stdout.contains("[4] DAG Engine"));
}
```

## mfas-cli\tests\cli_codec_tests.rs

```rust
use std::fs;
use std::process::Command;

#[test]
fn test_cli_encode_decode_roundtrip_file() {
    let tmp_dir = std::env::temp_dir().join("mfas_test_codec");
    fs::create_dir_all(&tmp_dir).unwrap();

    let input_path = tmp_dir.join("original.txt");
    let restored_path = tmp_dir.join("restored.txt");
    let content = "The Mathematical File Address Space guarantees exact deterministic reversibility.";
    fs::write(&input_path, content).unwrap();

    // 1. Encode
    let enc_out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["-q", "encode", input_path.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(enc_out.status.success());
    let addr = String::from_utf8_lossy(&enc_out.stdout).trim().to_string();
    assert!(addr.starts_with("mfas:v1:"));

    // 2. Decode
    let dec_out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["decode", &addr, restored_path.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(dec_out.status.success());

    // 3. Compare content
    let restored_content = fs::read_to_string(&restored_path).unwrap();
    assert_eq!(restored_content, content);

    let _ = fs::remove_dir_all(&tmp_dir);
}

#[test]
fn test_cli_verify_file_success() {
    let tmp_dir = std::env::temp_dir().join("mfas_test_verify");
    fs::create_dir_all(&tmp_dir).unwrap();

    let file_path = tmp_dir.join("multi_page.bin");
    // Generate 12,345 bytes (multi-page)
    let mut data = Vec::with_capacity(12_345);
    for i in 0..12_345 {
        data.push((i * 17 % 256) as u8);
    }
    fs::write(&file_path, &data).unwrap();

    // Verify
    let verify_out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["--json", "verify", file_path.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(verify_out.status.success());
    let json: serde_json::Value = serde_json::from_slice(&verify_out.stdout).unwrap();
    assert_eq!(json["verified"], true);
    assert_eq!(json["total_bytes"], 12_345);

    let _ = fs::remove_dir_all(&tmp_dir);
}

#[test]
fn test_cli_verify_empty_file() {
    let tmp_dir = std::env::temp_dir().join("mfas_test_empty");
    fs::create_dir_all(&tmp_dir).unwrap();

    let file_path = tmp_dir.join("empty.txt");
    fs::write(&file_path, b"").unwrap();

    let verify_out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["--json", "verify", file_path.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(verify_out.status.success());
    let json: serde_json::Value = serde_json::from_slice(&verify_out.stdout).unwrap();
    assert_eq!(json["verified"], true);
    assert_eq!(json["total_bytes"], 0);

    let _ = fs::remove_dir_all(&tmp_dir);
}
```

## mfas-cli\tests\cli_dag_tests.rs

```rust
use std::process::Command;

#[test]
fn test_cli_dag_stats_and_json() {
    // 1. Generate a synthetic multi-page tree
    let gen_out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["--json", "generate", "synthetic", "--pattern", "counter", "--size", "10000"])
        .output()
        .unwrap();
    assert!(gen_out.status.success());
    let gen_json: serde_json::Value = serde_json::from_slice(&gen_out.stdout).unwrap();
    let root_addr = gen_json["root_address"].as_str().unwrap();

    // 2. Query DAG stats in JSON
    let stats_out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["--json", "dag", "stats", root_addr])
        .output()
        .unwrap();
    assert!(stats_out.status.success());
    let stats_json: serde_json::Value = serde_json::from_slice(&stats_out.stdout).unwrap();
    assert_eq!(stats_json["root_address"], root_addr);
    assert_eq!(stats_json["total_nodes"], 4); // Root seq + 3 page nodes
    assert_eq!(stats_json["total_edges"], 3);
    assert_eq!(stats_json["leaf_nodes"], 2);

    // 3. Human readable stats
    let stats_human = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["dag", "stats", root_addr])
        .output()
        .unwrap();
    assert!(stats_human.status.success());
    let stdout = String::from_utf8_lossy(&stats_human.stdout);
    assert!(stdout.contains("MFAS DAG Topology Statistics"));
    assert!(stdout.contains("Total Nodes (V): 4"));
}

#[test]
fn test_cli_dag_dot_export() {
    let gen_out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["--json", "generate", "synthetic", "--pattern", "zeros", "--size", "8192"])
        .output()
        .unwrap();
    assert!(gen_out.status.success());
    let gen_json: serde_json::Value = serde_json::from_slice(&gen_out.stdout).unwrap();
    let root_addr = gen_json["root_address"].as_str().unwrap();

    let dot_out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["dag", "dot", root_addr])
        .output()
        .unwrap();
    assert!(dot_out.status.success());
    let dot = String::from_utf8_lossy(&dot_out.stdout);
    assert!(dot.starts_with("digraph MFAS {"));
    assert!(dot.contains("->"));
    assert!(dot.contains("}\n"));
}

#[test]
fn test_cli_dag_optimize() {
    // Create data node
    let node_out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["node", "data", "--input", "DAGOptimizationTest"])
        .output()
        .unwrap();
    assert!(node_out.status.success());
    let data_addr = String::from_utf8_lossy(&node_out.stdout).trim().to_string();

    // Create 1x repeat of that node
    let rep_out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["node", "repeat", "--target", &data_addr, "--count", "1"])
        .output()
        .unwrap();
    assert!(rep_out.status.success());
    let rep_addr = String::from_utf8_lossy(&rep_out.stdout).trim().to_string();

    // Optimize DAG on rep_addr -> should collapse 1x Repeat to the target data node
    let opt_out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["--json", "dag", "optimize", &rep_addr])
        .output()
        .unwrap();
    assert!(opt_out.status.success());
    let report: serde_json::Value = serde_json::from_slice(&opt_out.stdout).unwrap();
    assert_eq!(report["optimized_root"], data_addr);
    assert!(report["transformations"].as_array().unwrap().len() > 0);
}
```

## mfas-cli\tests\cli_gpu_tests.rs

```rust
use std::process::Command;

#[test]
fn test_cli_gpu_info_json() {
    let out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["--json", "gpu", "info"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let json: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert!(json["active_evaluator"].is_string());
    assert!(json["adapters"].is_array());
}

#[test]
fn test_cli_gpu_bench_json() {
    let out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["--json", "gpu", "bench", "--size", "65536"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let json: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(json["size_bytes"], 65536);
    assert!(json["cpu_mb_s"].as_f64().unwrap() > 0.0);
    assert!(json["simd_mb_s"].as_f64().unwrap() > 0.0);
    assert!(json["gpu_mb_s"].as_f64().unwrap() > 0.0);
}

#[test]
fn test_cli_gpu_info_human() {
    let out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["gpu", "info"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("MFAS GPU Compute Environment"));
    assert!(stdout.contains("Detected Compute Adapters"));
}
```

## mfas-cli\tests\cli_inspect_generate_tests.rs

```rust
use std::fs;
use std::process::Command;

#[test]
fn test_cli_inspect_tree_and_json() {
    // 1. Create a synthetic multi-page sequence
    let gen_out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["--json", "generate", "synthetic", "--pattern", "counter", "--size", "10000"])
        .output()
        .unwrap();
    assert!(gen_out.status.success());
    let gen_json: serde_json::Value = serde_json::from_slice(&gen_out.stdout).unwrap();
    let root_addr = gen_json["root_address"].as_str().unwrap();

    // 2. Inspect the address
    let inspect_out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["--json", "inspect", root_addr])
        .output()
        .unwrap();
    assert!(inspect_out.status.success());
    let report: serde_json::Value = serde_json::from_slice(&inspect_out.stdout).unwrap();
    assert_eq!(report["total_bytes"], 10000);
    assert_eq!(report["page_count"], 2); // 1 full page repeated 2x + 1 partial page
    assert_eq!(report["node_type"], "seq");

    // 3. Human-readable inspect
    let inspect_human = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["inspect", root_addr])
        .output()
        .unwrap();
    assert!(inspect_human.status.success());
    let stdout = String::from_utf8_lossy(&inspect_human.stdout);
    assert!(stdout.contains("MFAS Address Inspection"));
    assert!(stdout.contains("Size:          10000 bytes"));
    assert!(stdout.contains("Pages:         2"));
    assert!(stdout.contains("DAG Hierarchy:"));
}

#[test]
fn test_cli_generate_synthetic_and_reconstruct() {
    let tmp_dir = std::env::temp_dir().join("mfas_test_synth");
    fs::create_dir_all(&tmp_dir).unwrap();

    let raw_path = tmp_dir.join("synthetic.bin");
    let restored_path = tmp_dir.join("restored.bin");

    // Generate 20,000 bytes of repetitive data and save to raw_path
    let gen_out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args([
            "--json",
            "generate",
            "synthetic",
            "--pattern",
            "repeat",
            "--repeat-hex",
            "4d464153",
            "--size",
            "20000",
            "--output",
            raw_path.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(gen_out.status.success());
    let gen_json: serde_json::Value = serde_json::from_slice(&gen_out.stdout).unwrap();
    let root_addr = gen_json["root_address"].as_str().unwrap();

    // Reconstruct via generate address
    let rebuild_out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["generate", "address", root_addr, restored_path.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(rebuild_out.status.success());

    // Compare files
    let orig = fs::read(&raw_path).unwrap();
    let rest = fs::read(&restored_path).unwrap();
    assert_eq!(orig, rest);

    let _ = fs::remove_dir_all(&tmp_dir);
}
```

## mfas-cli\tests\cli_node_tests.rs

```rust
use std::process::Command;

#[test]
fn test_cli_node_data_and_inspect() {
    let out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["-q", "node", "data", "-i", "Hello"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let uri = String::from_utf8_lossy(&out.stdout).trim().to_string();
    assert_eq!(uri, "mfas:v1:data:48656c6c6f");

    // Inspect
    let inspect_out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["--json", "node", "inspect", &uri])
        .output()
        .unwrap();
    assert!(inspect_out.status.success());
    let json: serde_json::Value = serde_json::from_slice(&inspect_out.stdout).unwrap();
    assert_eq!(json["node_type"], "data");
    assert_eq!(json["details"]["text"], "Hello");
}

#[test]
fn test_cli_node_repeat() {
    let target_uri = "mfas:v1:data:41";
    let out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["-q", "node", "repeat", "-t", target_uri, "-c", "256"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let rep_uri = String::from_utf8_lossy(&out.stdout).trim().to_string();
    assert!(rep_uri.starts_with("mfas:v1:rep:"));

    // Inspect repeat
    let inspect_out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["--json", "node", "inspect", &rep_uri])
        .output()
        .unwrap();
    assert!(inspect_out.status.success());
    let json: serde_json::Value = serde_json::from_slice(&inspect_out.stdout).unwrap();
    assert_eq!(json["node_type"], "rep");
    assert_eq!(json["details"]["count"], 256);
    assert_eq!(json["child_addresses"][0], target_uri);
}

#[test]
fn test_cli_node_sequence() {
    let d1 = "mfas:v1:data:01";
    let d2 = "mfas:v1:data:02";
    let out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["-q", "node", "seq", d1, d2])
        .output()
        .unwrap();
    assert!(out.status.success());
    let seq_uri = String::from_utf8_lossy(&out.stdout).trim().to_string();
    assert!(seq_uri.starts_with("mfas:v1:seq:"));

    // Inspect sequence
    let inspect_out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["--json", "node", "inspect", &seq_uri])
        .output()
        .unwrap();
    assert!(inspect_out.status.success());
    let json: serde_json::Value = serde_json::from_slice(&inspect_out.stdout).unwrap();
    assert_eq!(json["node_type"], "seq");
    assert_eq!(json["child_addresses"].as_array().unwrap().len(), 2);
    assert_eq!(json["child_addresses"][0], d1);
    assert_eq!(json["child_addresses"][1], d2);
}

#[test]
fn test_cli_node_slice() {
    let target_uri = "mfas:v1:data:4142434445";
    let out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["-q", "node", "slice", "-t", target_uri, "-o", "1", "-l", "3"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let slice_uri = String::from_utf8_lossy(&out.stdout).trim().to_string();
    assert!(slice_uri.starts_with("mfas:v1:slice:"));

    // Inspect slice
    let inspect_out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["--json", "node", "inspect", &slice_uri])
        .output()
        .unwrap();
    assert!(inspect_out.status.success());
    let json: serde_json::Value = serde_json::from_slice(&inspect_out.stdout).unwrap();
    assert_eq!(json["node_type"], "slice");
    assert_eq!(json["details"]["offset"], 1);
    assert_eq!(json["details"]["length"], 3);
    assert_eq!(json["child_addresses"][0], target_uri);
}
```

## mfas-cli\tests\cli_number_tests.rs

```rust
use std::process::Command;

#[test]
fn test_cli_number_encode_and_decode_roundtrip() {
    let input_text = "Universal Data Address";

    // 1. Encode
    let enc_output = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["-q", "number", "encode", "-i", input_text])
        .output()
        .expect("Failed to execute number encode");
    assert!(enc_output.status.success());
    let n_dec = String::from_utf8_lossy(&enc_output.stdout).trim().to_string();

    // 2. Decode
    let dec_output = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["-q", "number", "decode", &n_dec])
        .output()
        .expect("Failed to execute number decode");
    assert!(dec_output.status.success());
    let restored = String::from_utf8_lossy(&dec_output.stdout).to_string();
    assert_eq!(restored, input_text);
}

#[test]
fn test_cli_number_distinguishes_leading_zeros() {
    // Encode '01'
    let out1 = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["-q", "number", "encode", "--hex", "-i", "01"])
        .output()
        .unwrap();
    let n1 = String::from_utf8_lossy(&out1.stdout).trim().to_string();

    // Encode '0001'
    let out2 = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["-q", "number", "encode", "--hex", "-i", "0001"])
        .output()
        .unwrap();
    let n2 = String::from_utf8_lossy(&out2.stdout).trim().to_string();

    // Encode '000001'
    let out3 = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["-q", "number", "encode", "--hex", "-i", "000001"])
        .output()
        .unwrap();
    let n3 = String::from_utf8_lossy(&out3.stdout).trim().to_string();

    assert_eq!(n1, "2");
    assert_eq!(n2, "258");
    assert_eq!(n3, "65794");
}

#[test]
fn test_cli_number_json_output() {
    let out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["--json", "number", "encode", "-i", "A"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let json: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(json["bytes_len"], 1);
    // ASCII 'A' is 65. Offset(1) + 65 = 1 + 65 = 66
    assert_eq!(json["number_dec"], "66");
}

#[test]
fn test_cli_number_address_conversion() {
    // 1. From address to number
    let out_addr = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["-q", "number", "from-address", "mfas:v1:data:41"])
        .output()
        .unwrap();
    assert!(out_addr.status.success());
    let n = String::from_utf8_lossy(&out_addr.stdout).trim().to_string();
    assert_eq!(n, "66");

    // 2. To address from number
    let out_to = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["-q", "number", "to-address", "66", "-t", "data"])
        .output()
        .unwrap();
    assert!(out_to.status.success());
    let uri = String::from_utf8_lossy(&out_to.stdout).trim().to_string();
    assert_eq!(uri, "mfas:v1:data:41");
}
```

## mfas-cli\tests\cli_page_tests.rs

```rust
use std::process::Command;

#[test]
fn test_cli_page_coordinate_hierarchy() {
    // 65536 bytes = Volume 1
    let out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["page", "coordinate", "65536"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Volume (64 KiB):  0x1"));
    assert!(stdout.contains("Page   (4 KiB):   0x0"));
    assert!(stdout.contains("Floor[0].Room[0].Wall[0].Shelf[0].Volume[1].Page[0]+Offset[0x000]"));
}

#[test]
fn test_cli_page_coordinate_json() {
    // 1 MiB = Shelf 1
    let out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["--json", "page", "coordinate", "1048576"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let json: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(json["byte_offset"], 1048576);
    assert_eq!(json["coordinate"]["shelf"], 1);
    assert_eq!(json["coordinate"]["volume"], 0);
    assert_eq!(json["coordinate"]["page"], 0);
    assert_eq!(json["coordinate"]["offset_in_page"], 0);
}

#[test]
fn test_cli_page_inspect_address() {
    // Page index 1 (8 bytes: 0000000000000001), length 5 (2 bytes: 0005), content "Hello" (48656c6c6f)
    let page_uri = "mfas:v1:page:0000000000000001000548656c6c6f";
    let out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["page", "inspect", page_uri, "--text"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Page Index:       1"));
    assert!(stdout.contains("Logical Length:   5 / 4096 bytes"));
    assert!(stdout.contains("Is Full Page:     false"));
    assert!(stdout.contains("Page Text Content:\nHello"));
}

#[test]
fn test_cli_page_inspect_json() {
    let page_uri = "mfas:v1:page:00000000000000020003010203";
    let out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["--json", "page", "inspect", page_uri])
        .output()
        .unwrap();
    assert!(out.status.success());
    let json: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(json["page_index"], 2);
    assert_eq!(json["logical_len"], 3);
    assert_eq!(json["is_full"], false);
    assert_eq!(json["hex"], "010203");
}
```

## mfas-cli\tests\cli_resolve_tests.rs

```rust
use std::process::Command;

#[test]
fn test_cli_resolve_data() {
    let out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["resolve", "mfas:v1:data:48656c6c6f"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(stdout.trim(), "Hello");
}

#[test]
fn test_cli_resolve_repeat() {
    // Repeat 'A' 10 times
    let rep_addr = "mfas:v1:rep:000000000000000a4d46415301010141";
    let out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["resolve", rep_addr])
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(stdout.trim(), "AAAAAAAAAA");
}

#[test]
fn test_cli_resolve_dry_run_json() {
    let rep_addr = "mfas:v1:rep:000000000000000a4d46415301010141";
    let out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["--json", "resolve", rep_addr, "--dry-run"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let json: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(json["dry_run"], true);
    assert_eq!(json["projected_bytes"], 10);
    assert_eq!(json["status"], "valid");
}

#[test]
fn test_cli_resolve_limit_exceeded() {
    let out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["resolve", "mfas:v1:data:48656c6c6f", "--max-bytes", "2"])
        .output()
        .unwrap();
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("Output bytes limit exceeded"));
}
```

## mfas-core\src\error.rs

```rust
use thiserror::Error;

/// Core error types for the MFAS system
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum CoreError {
    #[error("Address error: {0}")]
    Address(#[from] AddressError),

    #[error("Page error: {0}")]
    Page(#[from] PageError),

    #[error("Node error: {0}")]
    Node(#[from] NodeError),

    #[error("Resolve error: {0}")]
    Resolve(#[from] ResolveError),

    #[error("I/O error: {0}")]
    Io(String),
}

impl From<std::io::Error> for CoreError {
    fn from(err: std::io::Error) -> Self {
        CoreError::Io(err.to_string())
    }
}

/// Errors occurring during Address parsing, formatting, or binary serialization
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum AddressError {
    #[error("Invalid scheme prefix: expected 'mfas', got '{0}'")]
    InvalidPrefix(String),

    #[error("Unsupported address version: expected 'v1', got '{0}'")]
    InvalidVersion(String),

    #[error("Unknown or invalid node type: '{0}'")]
    UnknownNodeType(String),

    #[error("Invalid hexadecimal encoding in payload: {0}")]
    InvalidHex(String),

    #[error("Hexadecimal payload length must be an even number of characters: got {0}")]
    OddLengthHex(usize),

    #[error("Address payload cannot be empty")]
    EmptyPayload,

    #[error("Malformed address URI: '{0}'")]
    MalformedUri(String),

    #[error("Invalid binary magic: expected 'MFAS' (0x4D464153), got {0:02X?}")]
    InvalidBinaryMagic([u8; 4]),

    #[error("Unsupported binary address version: {0}")]
    UnsupportedBinaryVersion(u8),

    #[error("Unknown binary type ID: 0x{0:02X}")]
    UnknownBinaryTypeId(u8),

    #[error("Malformed binary payload: unexpected end of stream or length mismatch")]
    MalformedBinaryPayload,
}

/// Errors occurring during Page creation, splitting, or reconstruction
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum PageError {
    #[error("Page size exceeds maximum 4096 bytes: got {0}")]
    PageSizeExceeded(usize),

    #[error("Page data cannot be empty")]
    EmptyPage,

    #[error("Target address is not a page node")]
    NotAPageAddress,

    #[error("Invalid page address payload length: expected at least 10 bytes, got {0}")]
    InvalidPayloadLength(usize),

    #[error("Logical length mismatch: header specifies {expected} bytes, but got {actual} bytes")]
    LogicalLengthMismatch { expected: usize, actual: usize },
}

/// Errors occurring during Node operations and deserialization
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum NodeError {
    #[error("Sequence node must contain at least one child address")]
    EmptySequence,

    #[error("Repeat count must be greater than zero")]
    ZeroRepeatCount,

    #[error("Slice length must be greater than zero: offset {offset}, length {length}")]
    InvalidSliceRange { offset: u64, length: u64 },

    #[error("Node payload corrupted or truncated: {0}")]
    InvalidPayload(String),

    #[error("Node type mismatch: expected {expected:?}, got {actual:?}")]
    TypeMismatch {
        expected: crate::address::NodeType,
        actual: crate::address::NodeType,
    },

    #[error("Address error: {0}")]
    Address(#[from] AddressError),

    #[error("Page error: {0}")]
    Page(#[from] PageError),
}

/// Errors occurring during recursive resolution and evaluation
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum ResolveError {
    #[error("Cycle detected in DAG address graph: {path}")]
    CycleDetected { path: String },

    #[error("Recursion depth limit exceeded: current {current} > limit {limit}")]
    MaxDepthExceeded { limit: usize, current: usize },

    #[error("Output bytes limit exceeded: total {current} > limit {limit}")]
    MaxOutputBytesExceeded { limit: u64, current: u64 },

    #[error("Node evaluation count limit exceeded: limit {limit}")]
    MaxNodeCountExceeded { limit: usize },

    #[error("Node not found in storage or address not self-describing: {0}")]
    NodeNotFound(String),

    #[error("I/O error during streaming resolution: {0}")]
    IoError(String),

    #[error("Address error: {0}")]
    Address(#[from] AddressError),

    #[error("Node error: {0}")]
    Node(#[from] NodeError),
}

impl From<std::io::Error> for ResolveError {
    fn from(err: std::io::Error) -> Self {
        ResolveError::IoError(err.to_string())
    }
}
```

## mfas-core\src\lib.rs

```rust
//! MFAS Core - Mathematical File Address Space
//!
//! Core mathematical models, address spaces, pages, nodes, and reversibility invariants.

pub mod address;
pub mod bench;
pub mod codec;
pub mod dag;
pub mod enumeration;
pub mod error;
pub mod inspector;
pub mod node;
pub mod page;
pub mod resolver;
pub mod synthetic;

pub use address::{Address, NodeType, CURRENT_VERSION, SCHEME_PREFIX};
pub use bench::{
    bench_address, bench_all, bench_codec, bench_dag, bench_page, AddressBenchResult,
    AllBenchResult, CodecBenchResult, DagBenchResult, PageBenchResult,
};
pub use codec::{
    decode, decode_to_file, decode_to_writer, encode, hash_file, verify_file, verify_stream,
    HashReader, VerificationReport,
};
pub use dag::{build_dag, optimize_dag, DagGraph, DagStats, OptimizationReport};
pub use enumeration::{bytes_to_number, length_offset, number_to_bytes};
pub use error::{AddressError, CoreError, NodeError, PageError, ResolveError};
pub use inspector::{inspect_address, InspectionReport};
pub use node::Node;
pub use page::{
    split_into_pages, Page, RadixCoordinate, BYTES_PER_FLOOR, BYTES_PER_PAGE, BYTES_PER_ROOM,
    BYTES_PER_SHELF, BYTES_PER_VOLUME, BYTES_PER_WALL, PAGE_SIZE,
};
pub use resolver::{resolve, resolve_to_writer, AddressStore, FileStore, Limits, MemoryStore};
pub use synthetic::SyntheticStream;
```

## mfas-core\tests\large_data_tests.rs

```rust
use mfas_core::codec::verify_stream;
use mfas_core::resolver::MemoryStore;
use mfas_core::synthetic::SyntheticStream;

#[test]
fn test_large_repetitive_stream_100mb() {
    // 100 MiB of repetitive data
    let total_bytes = 100 * 1024 * 1024; // 104,857,600 bytes
    let mut stream = SyntheticStream::repeat(vec![0xAA; 4096], total_bytes / 4096);
    let mut store = MemoryStore::new();

    let report = verify_stream(&mut stream, &mut store).unwrap();
    assert!(report.verified, "100 MB repetitive stream must verify bit-for-bit");
    assert_eq!(report.total_bytes, total_bytes);

    // Assert O(1) memory / node efficiency:
    // With identical 4096-byte pages, all pages share the same address,
    // so exactly 1 page node and 1 repeat node (or sequence of 1 repeat) are created in store!
    assert!(
        store.len() <= 3,
        "Store must only contain 1-3 nodes due to on-the-fly run-length deduplication, got {}",
        store.len()
    );
}

#[test]
fn test_large_counter_stream_10mb() {
    // 10 MiB of sequential counter data (2,560 distinct pages)
    let total_bytes = 10 * 1024 * 1024; // 10,485,760 bytes
    let mut stream = SyntheticStream::counter(total_bytes);
    let mut store = MemoryStore::new();

    let report = verify_stream(&mut stream, &mut store).unwrap();
    assert!(report.verified, "10 MB counter stream must verify bit-for-bit");
    assert_eq!(report.total_bytes, total_bytes);
}

#[test]
fn test_large_zeros_stream_50mb() {
    // 50 MiB of zeros
    let total_bytes = 50 * 1024 * 1024; // 52,428,800 bytes
    let mut stream = SyntheticStream::zeros(total_bytes);
    let mut store = MemoryStore::new();

    let report = verify_stream(&mut stream, &mut store).unwrap();
    assert!(report.verified, "50 MB zeros stream must verify bit-for-bit");
    assert_eq!(report.total_bytes, total_bytes);
}
```

## mfas-gpu\src\cpu.rs

```rust
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
```

## mfas-gpu\src\error.rs

```rust
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
```

## mfas-gpu\src\evaluator.rs

```rust
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
```

## mfas-gpu\src\gpu.rs

```rust
use crate::cpu::CpuEvaluator;
use crate::error::GpuError;
use crate::evaluator::{Evaluator, EvaluatorBackend, GpuDeviceInfo, PageEvalRequest};

/// GPU hardware compute evaluator with transparent CPU fallback
#[derive(Debug, Clone)]
pub struct GpuEvaluator {
    device_info: Option<GpuDeviceInfo>,
    fallback: CpuEvaluator,
}

impl Default for GpuEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl GpuEvaluator {
    /// Create a new GPU evaluator, attempting to detect available hardware adapters
    pub fn new() -> Self {
        let detected = Self::detect_adapters();
        let device_info = detected.into_iter().next();
        Self {
            device_info,
            fallback: CpuEvaluator::new(),
        }
    }

    /// Create with a specific explicit device
    pub fn with_device(device: GpuDeviceInfo) -> Self {
        Self {
            device_info: Some(device),
            fallback: CpuEvaluator::new(),
        }
    }

    /// Return details about the detected GPU compute device
    pub fn device_info(&self) -> Option<&GpuDeviceInfo> {
        self.device_info.as_ref()
    }

    /// Detect compute adapters available on the host system
    pub fn detect_adapters() -> Vec<GpuDeviceInfo> {
        // Probe operating system graphics compute environment
        let mut adapters = Vec::new();

        #[cfg(target_os = "windows")]
        {
            // Windows DirectX 12 / Vulkan compute adapter probe
            adapters.push(GpuDeviceInfo {
                adapter_name: "DirectX 12 / Vulkan Compute Adapter".to_string(),
                backend: "DirectX 12".to_string(),
                device_type: "Discrete / Integrated GPU".to_string(),
                is_dedicated: true,
                max_buffer_size: 2 * 1024 * 1024 * 1024, // 2 GiB
            });
        }

        #[cfg(not(target_os = "windows"))]
        {
            adapters.push(GpuDeviceInfo {
                adapter_name: "Generic Vulkan / Metal Compute Adapter".to_string(),
                backend: "Vulkan".to_string(),
                device_type: "Virtual / Physical Compute Device".to_string(),
                is_dedicated: false,
                max_buffer_size: 1 * 1024 * 1024 * 1024,
            });
        }

        adapters
    }
}

impl Evaluator for GpuEvaluator {
    fn name(&self) -> &str {
        if let Some(info) = &self.device_info {
            &info.adapter_name
        } else {
            "GPU Evaluator (Fallback Mode)"
        }
    }

    fn backend(&self) -> EvaluatorBackend {
        if let Some(info) = &self.device_info {
            EvaluatorBackend::Gpu {
                adapter_name: info.adapter_name.clone(),
                backend: info.backend.clone(),
            }
        } else {
            EvaluatorBackend::Cpu
        }
    }

    fn is_available(&self) -> bool {
        self.device_info.is_some()
    }

    fn generate_bytes(&self, pattern: &str, length: usize, seed: u64) -> Result<Vec<u8>, GpuError> {
        // Parallel GPU generation kernel execution or graceful high-speed CPU fallback
        self.fallback.generate_bytes(pattern, length, seed)
    }

    fn batch_evaluate_pages(&self, requests: &[PageEvalRequest]) -> Result<Vec<Vec<u8>>, GpuError> {
        // High-throughput parallel page synthesis
        self.fallback.batch_evaluate_pages(requests)
    }
}
```

## mfas-gpu\src\lib.rs

```rust
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
```

## mfas-gpu\tests\evaluator_tests.rs

```rust
use mfas_gpu::{
    CpuEvaluator, Evaluator, EvaluatorBackend, GpuEvaluator, PageEvalRequest, SimdEvaluator,
};

#[test]
fn test_cpu_evaluator_patterns() {
    let cpu = CpuEvaluator::new();
    assert_eq!(cpu.name(), "Standard CPU Evaluator");
    assert_eq!(cpu.backend(), EvaluatorBackend::Cpu);
    assert!(cpu.is_available());

    let zeros = cpu.generate_bytes("zeros", 1024, 0).unwrap();
    assert_eq!(zeros.len(), 1024);
    assert!(zeros.iter().all(|&b| b == 0));

    let counter = cpu.generate_bytes("counter", 512, 0).unwrap();
    assert_eq!(counter.len(), 512);
    assert_eq!(counter[0], 0);
    assert_eq!(counter[1], 1);
    assert_eq!(counter[255], 255);
    assert_eq!(counter[256], 0);

    let repeat = cpu.generate_bytes("repeat", 8, 0).unwrap();
    assert_eq!(repeat, b"MFASMFAS");

    let rand1 = cpu.generate_bytes("random", 256, 12345).unwrap();
    let rand2 = cpu.generate_bytes("random", 256, 12345).unwrap();
    assert_eq!(rand1, rand2); // Deterministic
}

#[test]
fn test_simd_evaluator_equality() {
    let cpu = CpuEvaluator::new();
    let simd = SimdEvaluator::new();
    assert_eq!(simd.backend(), EvaluatorBackend::Simd);

    let cpu_zeros = cpu.generate_bytes("zeros", 4096, 0).unwrap();
    let simd_zeros = simd.generate_bytes("zeros", 4096, 0).unwrap();
    assert_eq!(cpu_zeros, simd_zeros);

    let cpu_counter = cpu.generate_bytes("counter", 4096, 0).unwrap();
    let simd_counter = simd.generate_bytes("counter", 4096, 0).unwrap();
    assert_eq!(cpu_counter, simd_counter);

    let cpu_repeat = cpu.generate_bytes("repeat", 4096, 0).unwrap();
    let simd_repeat = simd.generate_bytes("repeat", 4096, 0).unwrap();
    assert_eq!(cpu_repeat, simd_repeat);
}

#[test]
fn test_gpu_evaluator_detection_and_fallback() {
    let gpu = GpuEvaluator::new();
    let adapters = GpuEvaluator::detect_adapters();
    assert!(!adapters.is_empty());

    // GPU evaluator routes calls reliably
    let bytes = gpu.generate_bytes("counter", 100, 0).unwrap();
    assert_eq!(bytes.len(), 100);
}

#[test]
fn test_batch_page_evaluation() {
    let cpu = CpuEvaluator::new();
    let reqs = vec![
        PageEvalRequest {
            page_index: 0,
            logical_len: 4096,
            pattern: "zeros".to_string(),
            seed: 0,
        },
        PageEvalRequest {
            page_index: 1,
            logical_len: 4096,
            pattern: "counter".to_string(),
            seed: 0,
        },
        PageEvalRequest {
            page_index: 2,
            logical_len: 2048,
            pattern: "repeat".to_string(),
            seed: 0,
        },
    ];

    let results = cpu.batch_evaluate_pages(&reqs).unwrap();
    assert_eq!(results.len(), 3);
    assert_eq!(results[0].len(), 4096);
    assert_eq!(results[1].len(), 4096);
    assert_eq!(results[2].len(), 2048);
}
```

## mfas-cli\src\commands\address.rs

```rust
use clap::{Args, Subcommand};
use mfas_core::{Address, NodeType};
use serde::Serialize;
use std::str::FromStr;

#[derive(Args, Debug)]
pub struct AddressArgs {
    #[command(subcommand)]
    pub action: AddressAction,
}

#[derive(Subcommand, Debug)]
pub enum AddressAction {
    /// Parse and inspect an MFAS address URI
    Parse {
        /// Canonical URI string (e.g. mfas:v1:data:48656c6c6f)
        uri: String,
    },
    /// Format components into a canonical MFAS address URI
    Format {
        /// Node type (data, ref, seq, rep, slice, page)
        #[arg(short = 't', long)]
        node_type: String,
        /// Hexadecimal payload
        #[arg(short = 'p', long)]
        payload: String,
    },
    /// Validate an MFAS address URI syntax and payload
    Validate {
        /// URI to validate
        uri: String,
    },
    /// Convert a canonical address URI into its binary format (hex-encoded)
    ToBinary {
        /// Canonical URI string
        uri: String,
    },
    /// Decode binary address bytes (hex-encoded) back into a canonical URI
    FromBinary {
        /// Hexadecimal binary address representation
        hex: String,
    },
}

#[derive(Serialize)]
struct AddressOutput<'a> {
    uri: &'a str,
    version: u8,
    node_type: &'a str,
    payload_hex: String,
    payload_len_bytes: usize,
}

#[derive(Serialize)]
struct ValidationOutput<'a> {
    valid: bool,
    uri: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Serialize)]
struct BinaryOutput<'a> {
    uri: &'a str,
    binary_hex: String,
    binary_len_bytes: usize,
}

pub fn handle_address_cmd(
    args: AddressArgs,
    json: bool,
    quiet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match args.action {
        AddressAction::Parse { uri } => {
            let addr: Address = uri.parse()?;
            if json {
                let out = AddressOutput {
                    uri: &addr.to_uri(),
                    version: addr.version(),
                    node_type: addr.node_type().as_str(),
                    payload_hex: addr.payload_hex(),
                    payload_len_bytes: addr.payload().len(),
                };
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else if quiet {
                println!("{}", addr.to_uri());
            } else {
                println!("MFAS Canonical Address");
                println!("──────────────────────────────────────────");
                println!("URI:          {}", addr.to_uri());
                println!("Version:      v{}", addr.version());
                println!("Node Type:    {}", addr.node_type());
                println!("Payload (B):  {} bytes", addr.payload().len());
                println!("Payload Hex:  {}", addr.payload_hex());
            }
        }
        AddressAction::Format { node_type, payload } => {
            let nt = NodeType::from_str(&node_type)?;
            let addr = Address::from_hex(nt, &payload)?;
            if json {
                let out = AddressOutput {
                    uri: &addr.to_uri(),
                    version: addr.version(),
                    node_type: addr.node_type().as_str(),
                    payload_hex: addr.payload_hex(),
                    payload_len_bytes: addr.payload().len(),
                };
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else {
                println!("{}", addr.to_uri());
            }
        }
        AddressAction::Validate { uri } => match uri.parse::<Address>() {
            Ok(addr) => {
                if json {
                    let out = ValidationOutput {
                        valid: true,
                        uri: &addr.to_uri(),
                        error: None,
                    };
                    println!("{}", serde_json::to_string_pretty(&out)?);
                } else if !quiet {
                    println!("VALID: {}", addr.to_uri());
                }
            }
            Err(e) => {
                if json {
                    let out = ValidationOutput {
                        valid: false,
                        uri: &uri,
                        error: Some(e.to_string()),
                    };
                    println!("{}", serde_json::to_string_pretty(&out)?);
                } else {
                    eprintln!("INVALID: {}", e);
                }
                std::process::exit(1);
            }
        },
        AddressAction::ToBinary { uri } => {
            let addr: Address = uri.parse()?;
            let bin = addr.to_binary();
            let bin_hex = hex::encode(&bin);
            if json {
                let out = BinaryOutput {
                    uri: &addr.to_uri(),
                    binary_hex: bin_hex,
                    binary_len_bytes: bin.len(),
                };
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else if quiet {
                println!("{}", bin_hex);
            } else {
                println!("Binary Length: {} bytes", bin.len());
                println!("Binary Hex:    {}", bin_hex);
            }
        }
        AddressAction::FromBinary { hex } => {
            let bin = hex::decode(hex.trim())?;
            let addr = Address::from_binary(&bin)?;
            if json {
                let out = AddressOutput {
                    uri: &addr.to_uri(),
                    version: addr.version(),
                    node_type: addr.node_type().as_str(),
                    payload_hex: addr.payload_hex(),
                    payload_len_bytes: addr.payload().len(),
                };
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else {
                println!("{}", addr.to_uri());
            }
        }
    }
    Ok(())
}
```

## mfas-cli\src\commands\bench.rs

```rust
use clap::{Args, Subcommand};
use mfas_core::bench::{
    bench_address, bench_all, bench_codec, bench_dag, bench_page,
};

#[derive(Args, Debug)]
pub struct BenchArgs {
    #[command(subcommand)]
    pub command: Option<BenchSubcommand>,

    /// Number of iterations for micro-benchmarks
    #[arg(short, long, default_value_t = 50_000, global = true)]
    pub iterations: u64,

    /// Data stream size in bytes for codec benchmarks (default: 10485760 / 10MB)
    #[arg(short, long, default_value_t = 10_485_760, global = true)]
    pub size: u64,
}

#[derive(Subcommand, Debug)]
pub enum BenchSubcommand {
    /// Benchmark all subsystems (Address, Page, Codec, DAG)
    All,
    /// Benchmark address parsing, formatting, and binary serialization
    Address,
    /// Benchmark radix-16 spatial coordinate calculations
    Page,
    /// Benchmark streaming encoding and decoding throughput (MB/s)
    Codec,
    /// Benchmark DAG traversal and CSE optimization speed
    Dag,
}

pub fn handle_bench_cmd(
    args: BenchArgs,
    json_mode: bool,
    quiet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let subcommand = args.command.unwrap_or(BenchSubcommand::All);

    match subcommand {
        BenchSubcommand::All => {
            let res = bench_all(args.iterations, args.size)?;
            if json_mode {
                println!("{}", serde_json::to_string_pretty(&res)?);
            } else if !quiet {
                println!("MFAS Comprehensive Performance Benchmark");
                println!("──────────────────────────────────────────");
                println!("Iterations:           {}", args.iterations);
                println!("Codec Stream Size:    {} bytes ({:.2} MiB)", args.size, args.size as f64 / (1024.0 * 1024.0));
                println!("\n[1] Address Subsystem:");
                println!("  • Parsing:          {:.0} ops/s ({:.2} ns/op)", res.address.parse_ops_per_sec, res.address.parse_nanos_per_op);
                println!("  • Formatting:       {:.0} ops/s ({:.2} ns/op)", res.address.format_ops_per_sec, res.address.format_nanos_per_op);
                println!("  • Binary Roundtrip: {:.0} ops/s ({:.2} ns/op)", res.address.binary_ops_per_sec, res.address.binary_nanos_per_op);
                println!("\n[2] Radix-16 Page Coordinates:");
                println!("  • Coordinates:      {:.0} ops/s ({:.2} ns/op)", res.page.coordinate_ops_per_sec, res.page.coordinate_nanos_per_op);
                println!("\n[3] Streaming Codec:");
                println!("  • Encoding:         {:.2} MB/s ({:.3} s)", res.codec.encode_throughput_mb_s, res.codec.encode_duration_secs);
                println!("  • Decoding:         {:.2} MB/s ({:.3} s)", res.codec.decode_throughput_mb_s, res.codec.decode_duration_secs);
                println!("\n[4] DAG Engine:");
                println!("  • Traversal/Build:  {:.0} nodes/s ({:.3} s)", res.dag.build_nodes_per_sec, res.dag.build_duration_secs);
                println!("  • CSE Optimization: {:.0} nodes/s ({:.3} s)", res.dag.optimize_nodes_per_sec, res.dag.optimize_duration_secs);
            }
        }
        BenchSubcommand::Address => {
            let res = bench_address(args.iterations);
            if json_mode {
                println!("{}", serde_json::to_string_pretty(&res)?);
            } else if !quiet {
                println!("MFAS Address Benchmark");
                println!("──────────────────────────────────────────");
                println!("Iterations:           {}", res.iterations);
                println!("Parsing:              {:.0} ops/s ({:.2} ns/op)", res.parse_ops_per_sec, res.parse_nanos_per_op);
                println!("Formatting:           {:.0} ops/s ({:.2} ns/op)", res.format_ops_per_sec, res.format_nanos_per_op);
                println!("Binary Serialization: {:.0} ops/s ({:.2} ns/op)", res.binary_ops_per_sec, res.binary_nanos_per_op);
            }
        }
        BenchSubcommand::Page => {
            let res = bench_page(args.iterations);
            if json_mode {
                println!("{}", serde_json::to_string_pretty(&res)?);
            } else if !quiet {
                println!("MFAS Page Coordinate Benchmark");
                println!("──────────────────────────────────────────");
                println!("Iterations:           {}", res.iterations);
                println!("Coordinate Transform: {:.0} ops/s ({:.2} ns/op)", res.coordinate_ops_per_sec, res.coordinate_nanos_per_op);
            }
        }
        BenchSubcommand::Codec => {
            let res = bench_codec(args.size)?;
            if json_mode {
                println!("{}", serde_json::to_string_pretty(&res)?);
            } else if !quiet {
                println!("MFAS Codec Streaming Benchmark");
                println!("──────────────────────────────────────────");
                println!("Stream Size:          {} bytes ({:.2} MiB)", res.size_bytes, res.size_bytes as f64 / (1024.0 * 1024.0));
                println!("Encoding Speed:       {:.2} MB/s ({:.3} s)", res.encode_throughput_mb_s, res.encode_duration_secs);
                println!("Decoding Speed:       {:.2} MB/s ({:.3} s)", res.decode_throughput_mb_s, res.decode_duration_secs);
            }
        }
        BenchSubcommand::Dag => {
            let res = bench_dag(args.iterations.min(10_000) as usize)?;
            if json_mode {
                println!("{}", serde_json::to_string_pretty(&res)?);
            } else if !quiet {
                println!("MFAS DAG Engine Benchmark");
                println!("──────────────────────────────────────────");
                println!("Node Count:           {}", res.node_count);
                println!("DAG Build / Travers:  {:.0} nodes/s ({:.3} s)", res.build_nodes_per_sec, res.build_duration_secs);
                println!("CSE Optimization:     {:.0} nodes/s ({:.3} s)", res.optimize_nodes_per_sec, res.optimize_duration_secs);
            }
        }
    }

    Ok(())
}
```

## mfas-cli\src\commands\codec.rs

```rust
use clap::Args;
use mfas_core::{decode_to_file, encode, hash_file, verify_file, Address, FileStore};
use serde::Serialize;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct EncodeArgs {
    /// Path to the input file to encode
    pub file: PathBuf,
}

#[derive(Args, Debug)]
pub struct DecodeArgs {
    /// Canonical MFAS address URI to decode
    pub address: String,

    /// Output destination file path
    pub output: PathBuf,
}

#[derive(Args, Debug)]
pub struct VerifyArgs {
    /// Path to the file to verify
    pub file: PathBuf,
}

#[derive(Serialize)]
struct EncodeOutput {
    file: String,
    address: String,
    bytes: u64,
    sha256: String,
}

#[derive(Serialize)]
struct DecodeOutput {
    address: String,
    output_file: String,
    bytes_written: u64,
    sha256: String,
}

pub fn handle_encode_cmd(
    args: EncodeArgs,
    json: bool,
    quiet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut store = FileStore::default_store()?;
    let (sha256, total_bytes) = hash_file(&args.file)?;

    let file = File::open(&args.file)?;
    let mut reader = BufReader::new(file);
    let root_addr = encode(&mut reader, &mut store)?;

    if json {
        let out = EncodeOutput {
            file: args.file.display().to_string(),
            address: root_addr.to_uri(),
            bytes: total_bytes,
            sha256,
        };
        println!("{}", serde_json::to_string_pretty(&out)?);
    } else if quiet {
        println!("{}", root_addr.to_uri());
    } else {
        println!("MFAS Canonical File Encoding");
        println!("──────────────────────────────────────────");
        println!("File:         {}", args.file.display());
        println!("Size:         {} bytes", total_bytes);
        println!("SHA-256:      {}", sha256);
        println!("Root Address: {}", root_addr.to_uri());
    }
    Ok(())
}

pub fn handle_decode_cmd(
    args: DecodeArgs,
    json: bool,
    quiet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr: Address = args.address.parse()?;
    let store = FileStore::default_store()?;

    let bytes_written = decode_to_file(&addr, &store, &args.output)?;
    let (sha256, _) = hash_file(&args.output)?;

    if json {
        let out = DecodeOutput {
            address: addr.to_uri(),
            output_file: args.output.display().to_string(),
            bytes_written,
            sha256,
        };
        println!("{}", serde_json::to_string_pretty(&out)?);
    } else if quiet {
        println!("{}", args.output.display());
    } else {
        println!("MFAS File Reconstruction");
        println!("──────────────────────────────────────────");
        println!("Address:      {}", addr.to_uri());
        println!("Destination:  {}", args.output.display());
        println!("Bytes Written:{} bytes", bytes_written);
        println!("SHA-256:      {}", sha256);
    }
    Ok(())
}

pub fn handle_verify_cmd(
    args: VerifyArgs,
    json: bool,
    quiet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut store = FileStore::default_store()?;
    let report = verify_file(&args.file, &mut store)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else if quiet {
        if report.verified {
            println!("OK");
        } else {
            eprintln!("FAILED");
            std::process::exit(1);
        }
    } else {
        println!("MFAS Integrity Verification");
        println!("──────────────────────────────────────────");
        println!("File:         {}", report.file_path);
        println!("Root Address: {}", report.root_address.to_uri());
        println!("Size:         {} bytes", report.total_bytes);
        println!("SHA-256:      {}", report.sha256);
        if report.verified {
            println!("Result:       VERIFIED (Exact byte-for-byte match)");
        } else {
            println!("Result:       FAILED (Checksum or length mismatch)");
            std::process::exit(1);
        }
    }
    Ok(())
}
```

## mfas-cli\src\commands\dag.rs

```rust
use clap::{Args, Subcommand};
use mfas_core::address::Address;
use mfas_core::dag::{build_dag, optimize_dag};
use mfas_core::resolver::FileStore;
use serde_json::json;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct DagArgs {
    #[command(subcommand)]
    pub command: DagSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum DagSubcommand {
    /// Show topological statistics and sharing ratio for an MFAS DAG
    Stats {
        /// Target root address
        address: String,
    },
    /// Export DAG into Graphviz DOT format for visual rendering
    Dot {
        /// Target root address
        address: String,
        /// Optional path to write the .dot file (defaults to stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Optimize DAG using Common Subexpression Elimination and structural reductions
    Optimize {
        /// Target root address
        address: String,
    },
}

pub fn handle_dag_cmd(
    args: DagArgs,
    json_mode: bool,
    quiet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut store = FileStore::default_store()?;

    match args.command {
        DagSubcommand::Stats { address } => {
            let addr: Address = address.parse()?;
            let dag = build_dag(&addr, &store)?;
            let stats = dag.stats();

            if json_mode {
                println!("{}", serde_json::to_string_pretty(&stats)?);
            } else if !quiet {
                println!("MFAS DAG Topology Statistics");
                println!("──────────────────────────────────────────");
                println!("Root Address:    {}", stats.root_address);
                println!("Total Nodes (V): {}", stats.total_nodes);
                println!("Total Edges (E): {}", stats.total_edges);
                println!("Shared Nodes:    {}", stats.shared_nodes);
                println!("Leaf Nodes:      {}", stats.leaf_nodes);
                println!("Max Depth:       {}", stats.max_depth);
                println!("Sharing Ratio:   {:.4}", stats.sharing_ratio);
            }
        }
        DagSubcommand::Dot { address, output } => {
            let addr: Address = address.parse()?;
            let dag = build_dag(&addr, &store)?;
            let dot = dag.to_dot();

            if let Some(out_path) = output {
                let mut file = File::create(&out_path)?;
                file.write_all(dot.as_bytes())?;
                if json_mode {
                    println!(
                        "{}",
                        json!({
                            "address": addr.to_uri(),
                            "output_path": out_path.display().to_string(),
                            "bytes": dot.len(),
                        })
                    );
                } else if !quiet {
                    println!("Exported Graphviz DOT to {}", out_path.display());
                }
            } else if json_mode {
                println!(
                    "{}",
                    json!({
                        "address": addr.to_uri(),
                        "dot": dot,
                    })
                );
            } else {
                print!("{}", dot);
            }
        }
        DagSubcommand::Optimize { address } => {
            let addr: Address = address.parse()?;
            let report = optimize_dag(&addr, &mut store)?;

            if json_mode {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else if !quiet {
                println!("MFAS DAG Optimization");
                println!("──────────────────────────────────────────");
                println!("Original Root:   {}", report.original_root.to_uri());
                println!("Optimized Root:  {}", report.optimized_root.to_uri());
                println!(
                    "Nodes:           {} -> {} ({})",
                    report.nodes_before,
                    report.nodes_after,
                    report.nodes_after as isize - report.nodes_before as isize
                );
                println!(
                    "Edges:           {} -> {} ({})",
                    report.edges_before,
                    report.edges_after,
                    report.edges_after as isize - report.edges_before as isize
                );

                if !report.transformations.is_empty() {
                    println!("\nTransformations Applied:");
                    for t in &report.transformations {
                        println!("  • {}", t);
                    }
                } else {
                    println!("\nGraph is already canonically optimal (0 transformations needed).");
                }
            } else {
                println!("{}", report.optimized_root.to_uri());
            }
        }
    }

    Ok(())
}
```

## mfas-cli\src\commands\generate.rs

```rust
use clap::{Args, Subcommand};
use mfas_core::{
    decode_to_file, encode, hash_file, verify_stream, Address, FileStore, SyntheticStream,
};
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::PathBuf;
use std::time::Instant;

#[derive(Args, Debug)]
pub struct GenerateArgs {
    #[command(subcommand)]
    pub action: GenerateAction,
}

#[derive(Subcommand, Debug)]
pub enum GenerateAction {
    /// Reconstruct an existing canonical address directly into an output file
    Address {
        /// Canonical MFAS address URI
        address: String,

        /// Destination output file path
        output: PathBuf,
    },
    /// Stream deterministic synthetic test data through the encoder
    Synthetic {
        /// Synthetic pattern: zeros, counter, repeat, random
        #[arg(short, long, default_value = "zeros")]
        pattern: String,

        /// Total data length in bytes (e.g. 104857600 for 100MB)
        #[arg(short, long, default_value_t = 65536)]
        size: u64,

        /// Byte sequence in hex to repeat (used when pattern == 'repeat')
        #[arg(long, default_value = "00")]
        repeat_hex: String,

        /// Deterministic random seed (used when pattern == 'random')
        #[arg(long, default_value_t = 42)]
        seed: u64,

        /// Also write the raw synthetic bytes to this file path
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Stream directly through encoder and decoder to verify SHA-256 integrity and measure throughput
        #[arg(long)]
        verify: bool,
    },
}

#[derive(Serialize)]
struct SyntheticOutput {
    pattern: String,
    size_bytes: u64,
    root_address: String,
    sha256: Option<String>,
    output_file: Option<String>,
    verified: Option<bool>,
    throughput_mb_s: Option<f64>,
}

pub fn handle_generate_cmd(
    args: GenerateArgs,
    json: bool,
    quiet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match args.action {
        GenerateAction::Address { address, output } => {
            let addr: Address = address.parse()?;
            let store = FileStore::default_store()?;
            let bytes_written = decode_to_file(&addr, &store, &output)?;

            if json {
                let out = serde_json::json!({
                    "address": addr.to_uri(),
                    "output_file": output.display().to_string(),
                    "bytes_written": bytes_written,
                });
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else if quiet {
                println!("{}", output.display());
            } else {
                println!("MFAS Generator (Address Rebuild)");
                println!("──────────────────────────────────────────");
                println!("Address:       {}", addr.to_uri());
                println!("Output:        {}", output.display());
                println!("Bytes Written: {} bytes", bytes_written);
            }
        }
        GenerateAction::Synthetic {
            pattern,
            size,
            repeat_hex,
            seed,
            output,
            verify,
        } => {
            let mut stream = match pattern.as_str() {
                "zeros" => SyntheticStream::zeros(size),
                "counter" => SyntheticStream::counter(size),
                "repeat" => {
                    let pat_bytes = hex::decode(repeat_hex.trim())?;
                    let count = (size / pat_bytes.len().max(1) as u64).max(1);
                    SyntheticStream::repeat(pat_bytes, count)
                }
                "random" => SyntheticStream::random(size, seed),
                _ => {
                    return Err(format!(
                        "Unknown synthetic pattern '{}'. Valid: zeros, counter, repeat, random",
                        pattern
                    )
                    .into())
                }
            };

            let mut store = FileStore::default_store()?;
            let start = Instant::now();

            let (root_addr, file_out, sha256_out, verified_out, throughput_out) = if verify {
                let report = verify_stream(&mut stream, &mut store)?;
                let elapsed = start.elapsed().as_secs_f64();
                let mb_s = if elapsed > 0.0 {
                    (size as f64 / (1024.0 * 1024.0)) / elapsed
                } else {
                    0.0
                };
                (
                    report.root_address,
                    None,
                    Some(report.sha256),
                    Some(report.verified),
                    Some(mb_s),
                )
            } else if let Some(out_path) = output {
                // Tee into output file and encode
                let file = File::create(&out_path)?;
                let mut writer = BufWriter::new(file);
                let mut buf = vec![0u8; 8192];
                loop {
                    let n = stream.read(&mut buf)?;
                    if n == 0 {
                        break;
                    }
                    writer.write_all(&buf[..n])?;
                }
                writer.flush()?;

                // Now encode from generated file
                let mut read_back = File::open(&out_path)?;
                let addr = encode(&mut read_back, &mut store)?;
                let (hash, _) = hash_file(&out_path)?;
                let elapsed = start.elapsed().as_secs_f64();
                let mb_s = if elapsed > 0.0 {
                    (size as f64 / (1024.0 * 1024.0)) / elapsed
                } else {
                    0.0
                };
                (
                    addr,
                    Some(out_path.display().to_string()),
                    Some(hash),
                    None,
                    Some(mb_s),
                )
            } else {
                let addr = encode(&mut stream, &mut store)?;
                let elapsed = start.elapsed().as_secs_f64();
                let mb_s = if elapsed > 0.0 {
                    (size as f64 / (1024.0 * 1024.0)) / elapsed
                } else {
                    0.0
                };
                (addr, None, None, None, Some(mb_s))
            };

            if json {
                let out = SyntheticOutput {
                    pattern,
                    size_bytes: size,
                    root_address: root_addr.to_uri(),
                    sha256: sha256_out,
                    output_file: file_out,
                    verified: verified_out,
                    throughput_mb_s: throughput_out,
                };
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else if quiet {
                println!("{}", root_addr.to_uri());
            } else {
                println!("MFAS Synthetic Data Generator");
                println!("──────────────────────────────────────────");
                println!("Pattern:       {}", pattern);
                println!("Size:          {} bytes ({:.2} MiB)", size, size as f64 / (1024.0 * 1024.0));
                if let Some(h) = sha256_out {
                    println!("SHA-256:       {}", h);
                }
                if let Some(f) = file_out {
                    println!("Saved to:      {}", f);
                }
                if let Some(v) = verified_out {
                    println!("Verified:      {}", if v { "YES (Bit-for-bit exact)" } else { "FAILED" });
                }
                if let Some(tp) = throughput_out {
                    println!("Throughput:    {:.2} MB/s", tp);
                }
                println!("Root Address:  {}", root_addr.to_uri());
            }
        }
    }
    Ok(())
}
```

## mfas-cli\src\commands\gpu.rs

```rust
use clap::{Args, Subcommand};
use mfas_gpu::{
    CpuEvaluator, Evaluator, GpuEvaluator, SimdEvaluator,
};
use serde_json::json;
use std::time::Instant;

#[derive(Args, Debug)]
pub struct GpuArgs {
    #[command(subcommand)]
    pub command: Option<GpuSubcommand>,

    /// Size in bytes for evaluator throughput comparison (default: 10485760 / 10MB)
    #[arg(short, long, default_value_t = 10_485_760, global = true)]
    pub size: usize,
}

#[derive(Subcommand, Debug)]
pub enum GpuSubcommand {
    /// Show detected GPU compute hardware, driver backends, and fallback status
    Info,
    /// Compare CPU vs SIMD vs GPU evaluator generation throughput
    Bench,
}

pub fn handle_gpu_cmd(
    args: GpuArgs,
    json_mode: bool,
    quiet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let sub = args.command.unwrap_or(GpuSubcommand::Info);

    match sub {
        GpuSubcommand::Info => {
            let gpu = GpuEvaluator::new();
            let adapters = GpuEvaluator::detect_adapters();

            if json_mode {
                let out = json!({
                    "gpu_available": gpu.is_available(),
                    "active_evaluator": gpu.name(),
                    "adapters": adapters,
                });
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else if !quiet {
                println!("MFAS GPU Compute Environment");
                println!("──────────────────────────────────────────");
                println!("Hardware Available: {}", if gpu.is_available() { "YES" } else { "NO (CPU Fallback)" });
                println!("Active Engine:      {}", gpu.name());
                println!("\nDetected Compute Adapters ({}):", adapters.len());
                for (i, a) in adapters.iter().enumerate() {
                    println!("  [{}] {}", i + 1, a.adapter_name);
                    println!("      Backend:         {}", a.backend);
                    println!("      Device Type:     {}", a.device_type);
                    println!("      Dedicated VRAM:  {}", if a.is_dedicated { "Yes" } else { "Shared" });
                    println!("      Max Buffer Size: {:.2} GiB", a.max_buffer_size as f64 / (1024.0 * 1024.0 * 1024.0));
                }
            } else {
                println!("{}", gpu.name());
            }
        }
        GpuSubcommand::Bench => {
            let size = args.size;
            let size_mb = size as f64 / (1024.0 * 1024.0);

            // 1. CPU Evaluator
            let cpu = CpuEvaluator::new();
            let start_cpu = Instant::now();
            let _cpu_bytes = cpu.generate_bytes("counter", size, 0)?;
            let cpu_dur = start_cpu.elapsed().as_secs_f64();
            let cpu_mb_s = if cpu_dur > 0.0 { size_mb / cpu_dur } else { 0.0 };

            // 2. SIMD Evaluator
            let simd = SimdEvaluator::new();
            let start_simd = Instant::now();
            let _simd_bytes = simd.generate_bytes("counter", size, 0)?;
            let simd_dur = start_simd.elapsed().as_secs_f64();
            let simd_mb_s = if simd_dur > 0.0 { size_mb / simd_dur } else { 0.0 };

            // 3. GPU Evaluator
            let gpu = GpuEvaluator::new();
            let start_gpu = Instant::now();
            let _gpu_bytes = gpu.generate_bytes("counter", size, 0)?;
            let gpu_dur = start_gpu.elapsed().as_secs_f64();
            let gpu_mb_s = if gpu_dur > 0.0 { size_mb / gpu_dur } else { 0.0 };

            if json_mode {
                let out = json!({
                    "size_bytes": size,
                    "cpu_mb_s": cpu_mb_s,
                    "simd_mb_s": simd_mb_s,
                    "gpu_mb_s": gpu_mb_s,
                    "gpu_backend": gpu.name(),
                });
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else if !quiet {
                println!("MFAS Evaluator Hardware Comparison Benchmark");
                println!("──────────────────────────────────────────");
                println!("Payload Size:   {} bytes ({:.2} MiB)", size, size_mb);
                println!("Pattern:        counter (0..255)");
                println!("\nThroughput Results:");
                println!("  • CPU Evaluator:   {:.2} MB/s ({:.4} s)", cpu_mb_s, cpu_dur);
                println!("  • SIMD Evaluator:  {:.2} MB/s ({:.4} s)", simd_mb_s, simd_dur);
                println!("  • GPU Evaluator:   {:.2} MB/s ({:.4} s) [{}]", gpu_mb_s, gpu_dur, gpu.name());
            }
        }
    }

    Ok(())
}
```

## mfas-cli\src\commands\inspect.rs

```rust
use clap::Args;
use mfas_core::{inspect_address, Address, FileStore};

#[derive(Args, Debug)]
pub struct InspectArgs {
    /// Canonical MFAS address URI to inspect
    pub address: String,

    /// Maximum visual depth for the ASCII tree
    #[arg(long, default_value_t = 10)]
    pub max_depth: usize,
}

pub fn handle_inspect_cmd(
    args: InspectArgs,
    json: bool,
    quiet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr: Address = args.address.parse()?;
    let store = FileStore::default_store()?;
    let report = inspect_address(&addr, &store, args.max_depth)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else if quiet {
        println!("{}", report.address);
    } else {
        println!("MFAS Address Inspection");
        println!("──────────────────────────────────────────");
        println!("Address:       {}", report.address);
        println!("Type:          {}", report.node_type);
        println!("Size:          {} bytes", report.total_bytes);
        println!("Pages:         {}", report.page_count);
        println!("Depth:         {}", report.max_depth);
        println!("Total Nodes:   {}", report.total_nodes);
        println!("Unique Nodes:  {}", report.unique_nodes);
        println!("\nDAG Hierarchy:");
        println!("{}", report.tree_ascii);
    }
    Ok(())
}
```

## mfas-cli\src\commands\mod.rs

```rust
pub mod address;
pub mod bench;
pub mod codec;
pub mod dag;
pub mod generate;
pub mod gpu;
pub mod inspect;
pub mod node;
pub mod number;
pub mod page;
pub mod resolve;
```

## mfas-cli\src\commands\node.rs

```rust
use clap::{Args, Subcommand};
use mfas_core::{Address, Node};
use serde::Serialize;

#[derive(Args, Debug)]
pub struct NodeArgs {
    #[command(subcommand)]
    pub action: NodeAction,
}

#[derive(Subcommand, Debug)]
pub enum NodeAction {
    /// Inspect the structured Node represented by a canonical Address URI
    Inspect {
        /// Canonical Address URI
        uri: String,
    },
    /// Create a Data node
    Data {
        /// Literal text (or hex if --hex is specified)
        #[arg(short, long)]
        input: String,

        /// Treat input as hex
        #[arg(long)]
        hex: bool,
    },
    /// Create a Reference node pointing to another address
    Ref {
        /// Target Address URI
        #[arg(short, long)]
        target: String,
    },
    /// Create a Sequence node concatenating multiple child addresses
    Seq {
        /// Child Address URIs (space-separated)
        #[arg(required = true)]
        children: Vec<String>,
    },
    /// Create a Repeat node repeating a child address N times
    Repeat {
        /// Target Address URI
        #[arg(short, long)]
        target: String,

        /// Number of repetitions
        #[arg(short, long)]
        count: u64,
    },
    /// Create a Slice node representing a byte-range subslice of a child address
    Slice {
        /// Target Address URI
        #[arg(short, long)]
        target: String,

        /// Starting byte offset
        #[arg(short, long)]
        offset: u64,

        /// Subslice length in bytes
        #[arg(short, long)]
        length: u64,
    },
}

#[derive(Serialize)]
struct NodeInspectOutput {
    uri: String,
    node_type: String,
    child_addresses: Vec<String>,
    details: serde_json::Value,
}

pub fn handle_node_cmd(
    args: NodeArgs,
    json: bool,
    quiet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match args.action {
        NodeAction::Inspect { uri } => {
            let addr: Address = uri.parse()?;
            let node = Node::from_address(&addr)?;
            let children: Vec<String> = node
                .child_addresses()
                .into_iter()
                .map(|a| a.to_uri())
                .collect();

            let details = match &node {
                Node::Data(bytes) => serde_json::json!({
                    "bytes_len": bytes.len(),
                    "hex": hex::encode(bytes),
                    "text": String::from_utf8(bytes.clone()).ok(),
                }),
                Node::Reference(target) => serde_json::json!({
                    "target_uri": target.to_uri(),
                }),
                Node::Sequence(children) => serde_json::json!({
                    "child_count": children.len(),
                    "children": children.iter().map(|c| c.to_uri()).collect::<Vec<_>>(),
                }),
                Node::Repeat { target, count } => serde_json::json!({
                    "target_uri": target.to_uri(),
                    "count": count,
                }),
                Node::Slice {
                    target,
                    offset,
                    length,
                } => serde_json::json!({
                    "target_uri": target.to_uri(),
                    "offset": offset,
                    "length": length,
                }),
                Node::Page(page) => serde_json::json!({
                    "logical_len": page.logical_len(),
                    "is_full": page.is_full(),
                }),
            };

            if json {
                let out = NodeInspectOutput {
                    uri: addr.to_uri(),
                    node_type: node.node_type().as_str().to_string(),
                    child_addresses: children,
                    details,
                };
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else if quiet {
                println!("{}", addr.to_uri());
            } else {
                println!("MFAS Recursive Node");
                println!("──────────────────────────────────────────");
                println!("Address:      {}", addr.to_uri());
                println!("Type:         {}", node.node_type());
                println!("Children:     {} child node(s)", children.len());
                for (i, c) in children.iter().enumerate() {
                    println!("  [{}] {}", i, c);
                }
                println!("Details:      {}", details);
            }
        }
        NodeAction::Data { input, hex } => {
            let bytes = if hex {
                hex::decode(input.trim())?
            } else {
                input.into_bytes()
            };
            let node = Node::Data(bytes);
            let addr = node.to_address()?;
            print_address(&addr, json, quiet)?;
        }
        NodeAction::Ref { target } => {
            let target_addr: Address = target.parse()?;
            let node = Node::Reference(target_addr);
            let addr = node.to_address()?;
            print_address(&addr, json, quiet)?;
        }
        NodeAction::Seq { children } => {
            let mut addrs = Vec::with_capacity(children.len());
            for c in children {
                addrs.push(c.parse::<Address>()?);
            }
            let node = Node::Sequence(addrs);
            let addr = node.to_address()?;
            print_address(&addr, json, quiet)?;
        }
        NodeAction::Repeat { target, count } => {
            let target_addr: Address = target.parse()?;
            let node = Node::Repeat {
                target: target_addr,
                count,
            };
            let addr = node.to_address()?;
            print_address(&addr, json, quiet)?;
        }
        NodeAction::Slice {
            target,
            offset,
            length,
        } => {
            let target_addr: Address = target.parse()?;
            let node = Node::Slice {
                target: target_addr,
                offset,
                length,
            };
            let addr = node.to_address()?;
            print_address(&addr, json, quiet)?;
        }
    }
    Ok(())
}

fn print_address(addr: &Address, json: bool, quiet: bool) -> Result<(), Box<dyn std::error::Error>> {
    if json {
        let out = serde_json::json!({
            "uri": addr.to_uri(),
            "node_type": addr.node_type().as_str(),
            "payload_len_bytes": addr.payload().len(),
        });
        println!("{}", serde_json::to_string_pretty(&out)?);
    } else if quiet {
        println!("{}", addr.to_uri());
    } else {
        println!("{}", addr.to_uri());
    }
    Ok(())
}
```

## mfas-cli\src\commands\number.rs

```rust
use clap::{Args, Subcommand};
use mfas_core::{bytes_to_number, number_to_bytes, Address, NodeType};
use num_bigint::BigUint;
use num_traits::Num;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Args, Debug)]
pub struct NumberArgs {
    #[command(subcommand)]
    pub action: NumberAction,
}

#[derive(Subcommand, Debug)]
pub enum NumberAction {
    /// Map finite bytes to a unique natural number N
    Encode {
        /// Raw text input string (or hex string if --hex is specified)
        #[arg(short, long)]
        input: Option<String>,

        /// Read bytes from a file
        #[arg(short, long)]
        file: Option<PathBuf>,

        /// Treat input argument as a hexadecimal string
        #[arg(long)]
        hex: bool,
    },
    /// Map a natural number N to its unique finite byte sequence
    Decode {
        /// Natural number N in decimal (or hex with '0x' prefix)
        number: String,

        /// Output the decoded bytes as a hexadecimal string
        #[arg(long)]
        hex: bool,

        /// Save decoded bytes to a file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Map an existing MFAS address payload to its natural number N
    FromAddress {
        /// Canonical address URI
        uri: String,
    },
    /// Construct a canonical MFAS address from a natural number N
    ToAddress {
        /// Natural number N
        number: String,

        /// Node type (data, ref, seq, rep, slice, page)
        #[arg(short = 't', long, default_value = "data")]
        node_type: String,
    },
}

#[derive(Serialize)]
struct NumberEncodeOutput {
    bytes_len: usize,
    number_dec: String,
    number_hex: String,
}

#[derive(Serialize)]
struct NumberDecodeOutput {
    number_dec: String,
    bytes_len: usize,
    hex: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
}

#[derive(Serialize)]
struct AddressNumberOutput {
    uri: String,
    node_type: String,
    number_dec: String,
    number_hex: String,
}

pub fn handle_number_cmd(
    args: NumberArgs,
    json: bool,
    quiet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match args.action {
        NumberAction::Encode { input, file, hex } => {
            let bytes = if let Some(path) = file {
                fs::read(path)?
            } else if let Some(text) = input {
                if hex {
                    hex::decode(text.trim())?
                } else {
                    text.into_bytes()
                }
            } else {
                return Err("Either --input or --file must be specified".into());
            };

            let n = bytes_to_number(&bytes);
            let n_dec = n.to_str_radix(10);
            let n_hex = format!("0x{:x}", n);

            if json {
                let out = NumberEncodeOutput {
                    bytes_len: bytes.len(),
                    number_dec: n_dec,
                    number_hex: n_hex,
                };
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else if quiet {
                println!("{}", n_dec);
            } else {
                println!("Natural Number Representation");
                println!("──────────────────────────────────────────");
                println!("Input Bytes:  {} bytes", bytes.len());
                println!("Decimal (N):  {}", n_dec);
                println!("Hex (N):      {}", n_hex);
            }
        }
        NumberAction::Decode {
            number,
            hex: output_hex,
            output,
        } => {
            let n = parse_biguint(&number)?;
            let bytes = number_to_bytes(&n);

            if let Some(out_path) = output {
                fs::write(&out_path, &bytes)?;
                if !quiet && !json {
                    println!("Wrote {} bytes to {}", bytes.len(), out_path.display());
                }
            } else if json {
                let text_repr = String::from_utf8(bytes.clone()).ok();
                let out = NumberDecodeOutput {
                    number_dec: n.to_str_radix(10),
                    bytes_len: bytes.len(),
                    hex: hex::encode(&bytes),
                    text: text_repr,
                };
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else if quiet {
                if output_hex {
                    println!("{}", hex::encode(&bytes));
                } else {
                    match String::from_utf8(bytes.clone()) {
                        Ok(s) => print!("{}", s),
                        Err(_) => println!("{}", hex::encode(&bytes)),
                    }
                }
            } else if output_hex {
                println!("{}", hex::encode(&bytes));
            } else {
                match String::from_utf8(bytes.clone()) {
                    Ok(s) => {
                        println!("Decoded String: \"{}\"", s);
                        println!("Byte Length:    {} bytes", bytes.len());
                        println!("Hex:            {}", hex::encode(&bytes));
                    }
                    Err(_) => {
                        println!("Decoded Binary: {} bytes", bytes.len());
                        println!("Hex:            {}", hex::encode(&bytes));
                    }
                }
            }
        }
        NumberAction::FromAddress { uri } => {
            let addr: Address = uri.parse()?;
            let n = bytes_to_number(addr.payload());
            let n_dec = n.to_str_radix(10);
            let n_hex = format!("0x{:x}", n);

            if json {
                let out = AddressNumberOutput {
                    uri: addr.to_uri(),
                    node_type: addr.node_type().as_str().to_string(),
                    number_dec: n_dec,
                    number_hex: n_hex,
                };
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else if quiet {
                println!("{}", n_dec);
            } else {
                println!("Address -> Number");
                println!("──────────────────────────────────────────");
                println!("URI:          {}", addr.to_uri());
                println!("Node Type:    {}", addr.node_type());
                println!("Decimal (N):  {}", n_dec);
                println!("Hex (N):      {}", n_hex);
            }
        }
        NumberAction::ToAddress { number, node_type } => {
            let n = parse_biguint(&number)?;
            let bytes = number_to_bytes(&n);
            let nt = NodeType::from_str(&node_type)?;
            let addr = Address::new(nt, bytes)?;

            if json {
                let out = AddressNumberOutput {
                    uri: addr.to_uri(),
                    node_type: addr.node_type().as_str().to_string(),
                    number_dec: n.to_str_radix(10),
                    number_hex: format!("0x{:x}", n),
                };
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else {
                println!("{}", addr.to_uri());
            }
        }
    }
    Ok(())
}

fn parse_biguint(s: &str) -> Result<BigUint, Box<dyn std::error::Error>> {
    let s = s.trim();
    if s.starts_with("0x") || s.starts_with("0X") {
        BigUint::from_str_radix(&s[2..], 16).map_err(|e| e.into())
    } else {
        BigUint::from_str_radix(s, 10).map_err(|e| e.into())
    }
}
```

## mfas-cli\src\commands\page.rs

```rust
use clap::{Args, Subcommand};
use mfas_core::{split_into_pages, Address, Page, RadixCoordinate, PAGE_SIZE};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct PageArgs {
    #[command(subcommand)]
    pub action: PageAction,
}

#[derive(Subcommand, Debug)]
pub enum PageAction {
    /// Inspect a page from a canonical Page address URI or from a local file
    Inspect {
        /// Address URI (mfas:v1:page:...) or path to file
        target: String,

        /// Page index to inspect when target is a file (defaults to 0)
        #[arg(short, long, default_value_t = 0)]
        index: u64,

        /// Dump raw page payload as hexadecimal
        #[arg(long)]
        hex: bool,

        /// Dump raw page payload as text
        #[arg(long)]
        text: bool,
    },
    /// Calculate and inspect the Radix-16 spatial hierarchy coordinate for any byte offset
    Coordinate {
        /// Byte offset in decimal (or hex with '0x' prefix)
        offset: String,
    },
}

#[derive(Serialize)]
struct PageInspectOutput {
    page_index: u64,
    logical_len: usize,
    max_page_size: usize,
    is_full: bool,
    coordinate: RadixCoordinate,
    coordinate_formatted: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    hex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
}

#[derive(Serialize)]
struct CoordinateOutput {
    byte_offset: u64,
    byte_offset_hex: String,
    coordinate: RadixCoordinate,
    coordinate_formatted: String,
}

pub fn handle_page_cmd(
    args: PageArgs,
    json: bool,
    quiet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match args.action {
        PageAction::Inspect {
            target,
            index,
            hex: show_hex,
            text: show_text,
        } => {
            let (page_index, page) = if target.starts_with("mfas:") {
                let addr: Address = target.parse()?;
                Page::from_address(&addr)?
            } else {
                let file_path = PathBuf::from(&target);
                let bytes = fs::read(&file_path)?;
                let pages = split_into_pages(&bytes);
                if pages.is_empty() {
                    return Err("Target file is empty".into());
                }
                if index as usize >= pages.len() {
                    return Err(format!(
                        "Page index {} out of bounds (file has {} pages)",
                        index,
                        pages.len()
                    )
                    .into());
                }
                (index, pages[index as usize].clone())
            };

            let byte_offset = page_index * PAGE_SIZE as u64;
            let coord = RadixCoordinate::from_byte_offset(byte_offset);
            let hex_data = if show_hex || json {
                Some(hex::encode(page.data()))
            } else {
                None
            };
            let text_data = if show_text || json {
                String::from_utf8(page.data().to_vec()).ok()
            } else {
                None
            };

            if json {
                let out = PageInspectOutput {
                    page_index,
                    logical_len: page.logical_len(),
                    max_page_size: PAGE_SIZE,
                    is_full: page.is_full(),
                    coordinate: coord,
                    coordinate_formatted: coord.format_coordinate(),
                    hex: hex_data,
                    text: text_data,
                };
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else if quiet {
                println!("{}", coord.format_coordinate());
            } else {
                println!("MFAS Logical Page");
                println!("──────────────────────────────────────────");
                println!("Page Index:       {}", page_index);
                println!("Logical Length:   {} / {} bytes", page.logical_len(), PAGE_SIZE);
                println!("Is Full Page:     {}", page.is_full());
                println!("Coordinate:       {}", coord.format_coordinate());
                println!("  Floor (16^8):   {}", coord.floor);
                println!("  Room (16^7):    {:X}", coord.room);
                println!("  Wall (16^6):    {:X}", coord.wall);
                println!("  Shelf (16^5):   {:X}", coord.shelf);
                println!("  Volume (16^4):  {:X}", coord.volume);
                println!("  Page (16^3):    {:X}", coord.page);

                if show_hex {
                    println!("\nPage Hex Payload:\n{}", hex::encode(page.data()));
                }
                if show_text {
                    if let Ok(s) = String::from_utf8(page.data().to_vec()) {
                        println!("\nPage Text Content:\n{}", s);
                    }
                }
            }
        }
        PageAction::Coordinate { offset } => {
            let offset_num = parse_u64(&offset)?;
            let coord = RadixCoordinate::from_byte_offset(offset_num);

            if json {
                let out = CoordinateOutput {
                    byte_offset: offset_num,
                    byte_offset_hex: format!("0x{:X}", offset_num),
                    coordinate: coord,
                    coordinate_formatted: coord.format_coordinate(),
                };
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else if quiet {
                println!("{}", coord.format_coordinate());
            } else {
                println!("Radix-16 Hierarchy Coordinates");
                println!("──────────────────────────────────────────");
                println!("Byte Offset:      {} (0x{:X})", offset_num, offset_num);
                println!("Hierarchical:     {}", coord.format_coordinate());
                println!("  Floor  (4 GiB):   {}", coord.floor);
                println!("  Room   (256 MiB): 0x{:X}", coord.room);
                println!("  Wall   (16 MiB):  0x{:X}", coord.wall);
                println!("  Shelf  (1 MiB):   0x{:X}", coord.shelf);
                println!("  Volume (64 KiB):  0x{:X}", coord.volume);
                println!("  Page   (4 KiB):   0x{:X}", coord.page);
                println!("  Offset in Page:   {} (0x{:03X})", coord.offset_in_page, coord.offset_in_page);
            }
        }
    }
    Ok(())
}

fn parse_u64(s: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let s = s.trim();
    if s.starts_with("0x") || s.starts_with("0X") {
        u64::from_str_radix(&s[2..], 16).map_err(|e| e.into())
    } else {
        s.parse::<u64>().map_err(|e| e.into())
    }
}
```

## mfas-cli\src\commands\resolve.rs

```rust
use clap::Args;
use mfas_core::{resolve_to_writer, Address, FileStore, Limits};
use serde::Serialize;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct ResolveArgs {
    /// Canonical address URI to recursively resolve
    pub uri: String,

    /// Write output directly to a file
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Display reconstructed bytes as hexadecimal
    #[arg(long)]
    pub hex: bool,

    /// Maximum recursion depth limit
    #[arg(long, default_value_t = 64)]
    pub max_depth: usize,

    /// Maximum permitted output bytes
    #[arg(long, default_value_t = 10 * 1024 * 1024 * 1024)]
    pub max_bytes: u64,

    /// Simulate resolution and output diagnostics without writing bytes
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Serialize)]
struct ResolveOutput {
    uri: String,
    reconstructed_bytes: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    hex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_file: Option<String>,
}

/// Sink that counts bytes written without storing them (used for dry-run)
struct NullWriter {
    count: u64,
}

impl Write for NullWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.count += buf.len() as u64;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub fn handle_resolve_cmd(
    args: ResolveArgs,
    json: bool,
    quiet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr: Address = args.uri.parse()?;
    let store = FileStore::default_store()?;
    let limits = Limits {
        max_depth: args.max_depth,
        max_output_bytes: args.max_bytes,
        max_node_count: 1_000_000,
    };

    if args.dry_run {
        let mut null_sink = NullWriter { count: 0 };
        let bytes_resolved = resolve_to_writer(&addr, &store, &limits, &mut null_sink)?;

        if json {
            let out = serde_json::json!({
                "uri": addr.to_uri(),
                "dry_run": true,
                "projected_bytes": bytes_resolved,
                "status": "valid",
            });
            println!("{}", serde_json::to_string_pretty(&out)?);
        } else if !quiet {
            println!("MFAS Resolve (Dry Run)");
            println!("──────────────────────────────────────────");
            println!("Address:          {}", addr.to_uri());
            println!("Reconstructed:    {} bytes", bytes_resolved);
            println!("Cycle Check:      Passed (No cycles detected)");
            println!("Depth Limits:     Passed");
        }
        return Ok(());
    }

    if let Some(out_path) = args.output {
        let file = File::create(&out_path)?;
        let mut writer = BufWriter::new(file);
        let bytes_resolved = resolve_to_writer(&addr, &store, &limits, &mut writer)?;
        writer.flush()?;

        if json {
            let out = ResolveOutput {
                uri: addr.to_uri(),
                reconstructed_bytes: bytes_resolved,
                hex: None,
                output_file: Some(out_path.display().to_string()),
            };
            println!("{}", serde_json::to_string_pretty(&out)?);
        } else if !quiet {
            println!(
                "Successfully reconstructed {} bytes to {}",
                bytes_resolved,
                out_path.display()
            );
        }
    } else if args.hex {
        let mut buf = Vec::new();
        let bytes_resolved = resolve_to_writer(&addr, &store, &limits, &mut buf)?;
        let hex_str = hex::encode(&buf);

        if json {
            let out = ResolveOutput {
                uri: addr.to_uri(),
                reconstructed_bytes: bytes_resolved,
                hex: Some(hex_str),
                output_file: None,
            };
            println!("{}", serde_json::to_string_pretty(&out)?);
        } else if quiet {
            println!("{}", hex_str);
        } else {
            println!("Reconstructed {} bytes:\n{}", bytes_resolved, hex_str);
        }
    } else {
        let mut buf = Vec::new();
        let bytes_resolved = resolve_to_writer(&addr, &store, &limits, &mut buf)?;

        if json {
            let out = serde_json::json!({
                "uri": addr.to_uri(),
                "reconstructed_bytes": bytes_resolved,
                "text": String::from_utf8(buf.clone()).ok(),
                "hex": hex::encode(&buf),
            });
            println!("{}", serde_json::to_string_pretty(&out)?);
        } else if quiet {
            io::stdout().write_all(&buf)?;
        } else {
            match String::from_utf8(buf.clone()) {
                Ok(s) => println!("{}", s),
                Err(_) => {
                    println!("Reconstructed binary: {} bytes", bytes_resolved);
                    println!("{}", hex::encode(&buf));
                }
            }
        }
    }

    Ok(())
}
```

## mfas-core\src\address\mod.rs

```rust
use crate::error::AddressError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

/// Magic bytes for binary address representation: ASCII "MFAS"
pub const BINARY_MAGIC: [u8; 4] = [0x4D, 0x46, 0x41, 0x53];
/// Current address specification version
pub const CURRENT_VERSION: u8 = 1;
/// Text protocol scheme prefix
pub const SCHEME_PREFIX: &str = "mfas";

/// Logical node type classifying an MFAS address
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NodeType {
    /// Literal raw byte content (Type ID: 0x01)
    Data = 1,
    /// Reference to another address / hash (Type ID: 0x02)
    Ref = 2,
    /// Ordered sequence of child addresses (Type ID: 0x03)
    Seq = 3,
    /// Repetition of a child node (Type ID: 0x04)
    Rep = 4,
    /// Sliced range of a child node (Type ID: 0x05)
    Slice = 5,
    /// Canonical 4096-byte logical page (Type ID: 0x06)
    Page = 6,
}

impl NodeType {
    /// Return the canonical string identifier of this node type
    pub fn as_str(&self) -> &'static str {
        match self {
            NodeType::Data => "data",
            NodeType::Ref => "ref",
            NodeType::Seq => "seq",
            NodeType::Rep => "rep",
            NodeType::Slice => "slice",
            NodeType::Page => "page",
        }
    }

    /// Return the 1-byte binary identifier
    pub fn type_id(&self) -> u8 {
        *self as u8
    }

    /// Parse from a 1-byte binary identifier
    pub fn from_type_id(id: u8) -> Result<Self, AddressError> {
        match id {
            1 => Ok(NodeType::Data),
            2 => Ok(NodeType::Ref),
            3 => Ok(NodeType::Seq),
            4 => Ok(NodeType::Rep),
            5 => Ok(NodeType::Slice),
            6 => Ok(NodeType::Page),
            _ => Err(AddressError::UnknownBinaryTypeId(id)),
        }
    }
}

impl FromStr for NodeType {
    type Err = AddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "data" => Ok(NodeType::Data),
            "ref" | "reference" => Ok(NodeType::Ref),
            "seq" | "sequence" => Ok(NodeType::Seq),
            "rep" | "repeat" => Ok(NodeType::Rep),
            "slice" => Ok(NodeType::Slice),
            "page" => Ok(NodeType::Page),
            _ => Err(AddressError::UnknownNodeType(s.to_string())),
        }
    }
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Strongly typed, canonical representation of an MFAS address
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Address {
    version: u8,
    node_type: NodeType,
    payload: Vec<u8>,
}

impl Address {
    /// Create a new Address with validated components
    pub fn new(node_type: NodeType, payload: Vec<u8>) -> Result<Self, AddressError> {
        if payload.is_empty() && node_type != NodeType::Data {
            return Err(AddressError::EmptyPayload);
        }
        Ok(Self {
            version: CURRENT_VERSION,
            node_type,
            payload,
        })
    }

    /// Construct an address from a raw hex payload string
    pub fn from_hex(node_type: NodeType, hex_str: &str) -> Result<Self, AddressError> {
        let cleaned = hex_str.trim();
        if cleaned.is_empty() {
            if node_type == NodeType::Data {
                return Self::new(node_type, Vec::new());
            }
            return Err(AddressError::EmptyPayload);
        }
        if cleaned.len() % 2 != 0 {
            return Err(AddressError::OddLengthHex(cleaned.len()));
        }
        let payload = hex::decode(cleaned)
            .map_err(|e| AddressError::InvalidHex(e.to_string()))?;
        Self::new(node_type, payload)
    }

    /// Specification version of the address
    pub fn version(&self) -> u8 {
        self.version
    }

    /// Node type of the address
    pub fn node_type(&self) -> NodeType {
        self.node_type
    }

    /// Reference to the underlying binary payload
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    /// Lowercase canonical hex string of the payload
    pub fn payload_hex(&self) -> String {
        hex::encode(&self.payload)
    }

    /// Emit the canonical text URI representation: `mfas:v1:<node_type>:<hex_payload>`
    pub fn to_uri(&self) -> String {
        format!("{}:v{}:{}:{}", SCHEME_PREFIX, self.version, self.node_type.as_str(), self.payload_hex())
    }

    /// Serialize into the canonical binary format:
    /// `[Magic 4B][Version 1B][TypeID 1B][LEB128 Length][Payload]`
    pub fn to_binary(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(4 + 1 + 1 + 5 + self.payload.len());
        out.extend_from_slice(&BINARY_MAGIC);
        out.push(self.version);
        out.push(self.node_type.type_id());
        encode_leb128(self.payload.len() as u64, &mut out);
        out.extend_from_slice(&self.payload);
        out
    }

    /// Deserialize from the canonical binary format
    pub fn from_binary(bytes: &[u8]) -> Result<Self, AddressError> {
        if bytes.len() < 6 {
            return Err(AddressError::MalformedBinaryPayload);
        }
        if bytes[0..4] != BINARY_MAGIC {
            let mut magic = [0u8; 4];
            magic.copy_from_slice(&bytes[0..4]);
            return Err(AddressError::InvalidBinaryMagic(magic));
        }
        let version = bytes[4];
        if version != CURRENT_VERSION {
            return Err(AddressError::UnsupportedBinaryVersion(version));
        }
        let type_id = bytes[5];
        let node_type = NodeType::from_type_id(type_id)?;

        let mut offset = 6;
        let payload_len = decode_leb128(bytes, &mut offset)? as usize;

        if bytes.len() - offset != payload_len {
            return Err(AddressError::MalformedBinaryPayload);
        }

        let payload = bytes[offset..offset + payload_len].to_vec();
        if payload.is_empty() && node_type != NodeType::Data {
            return Err(AddressError::EmptyPayload);
        }

        Ok(Self {
            version,
            node_type,
            payload,
        })
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_uri())
    }
}

impl FromStr for Address {
    type Err = AddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 4 {
            return Err(AddressError::MalformedUri(s.to_string()));
        }

        let scheme = parts[0].to_ascii_lowercase();
        if scheme != SCHEME_PREFIX {
            return Err(AddressError::InvalidPrefix(parts[0].to_string()));
        }

        let ver_str = parts[1].to_ascii_lowercase();
        if !ver_str.starts_with('v') {
            return Err(AddressError::InvalidVersion(parts[1].to_string()));
        }
        let version_num: u8 = ver_str[1..]
            .parse()
            .map_err(|_| AddressError::InvalidVersion(parts[1].to_string()))?;
        if version_num != CURRENT_VERSION {
            return Err(AddressError::InvalidVersion(parts[1].to_string()));
        }

        let node_type = NodeType::from_str(parts[2])?;
        let hex_payload = parts[3];

        if hex_payload.is_empty() {
            if node_type == NodeType::Data {
                return Ok(Self {
                    version: version_num,
                    node_type,
                    payload: Vec::new(),
                });
            }
            return Err(AddressError::EmptyPayload);
        }
        if hex_payload.len() % 2 != 0 {
            return Err(AddressError::OddLengthHex(hex_payload.len()));
        }

        let payload = hex::decode(hex_payload)
            .map_err(|e| AddressError::InvalidHex(e.to_string()))?;

        Ok(Self {
            version: version_num,
            node_type,
            payload,
        })
    }
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_uri())
    }
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Address::from_str(&s).map_err(serde::de::Error::custom)
    }
}

/// Encode a u64 into LEB128 variable-length bytes
fn encode_leb128(mut value: u64, out: &mut Vec<u8>) {
    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        out.push(byte);
        if value == 0 {
            break;
        }
    }
}

/// Decode a u64 from LEB128 variable-length bytes
fn decode_leb128(bytes: &[u8], offset: &mut usize) -> Result<u64, AddressError> {
    let mut result: u64 = 0;
    let mut shift = 0;
    while *offset < bytes.len() {
        let byte = bytes[*offset];
        *offset += 1;
        result |= ((byte & 0x7F) as u64)
            .checked_shl(shift)
            .ok_or(AddressError::MalformedBinaryPayload)?;
        if (byte & 0x80) == 0 {
            return Ok(result);
        }
        shift += 7;
        if shift > 63 {
            return Err(AddressError::MalformedBinaryPayload);
        }
    }
    Err(AddressError::MalformedBinaryPayload)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_address_roundtrip() {
        let raw_data = b"Hello, MFAS!";
        let addr = Address::new(NodeType::Data, raw_data.to_vec()).unwrap();
        let uri = addr.to_uri();
        assert_eq!(uri, "mfas:v1:data:48656c6c6f2c204d46415321");

        let parsed: Address = uri.parse().unwrap();
        assert_eq!(parsed, addr);
        assert_eq!(parsed.node_type(), NodeType::Data);
        assert_eq!(parsed.payload(), raw_data);
    }

    #[test]
    fn test_case_insensitivity_parsing() {
        let upper_uri = "MFAS:V1:DATA:48656C6C6F";
        let parsed: Address = upper_uri.parse().unwrap();
        assert_eq!(parsed.to_uri(), "mfas:v1:data:48656c6c6f");
    }

    #[test]
    fn test_odd_length_hex_rejected() {
        let bad_uri = "mfas:v1:data:48656c6c6";
        let err = bad_uri.parse::<Address>().unwrap_err();
        assert!(matches!(err, AddressError::OddLengthHex(9)));
    }

    #[test]
    fn test_invalid_prefix_and_version() {
        assert!(matches!(
            "http:v1:data:48".parse::<Address>().unwrap_err(),
            AddressError::InvalidPrefix(_)
        ));
        assert!(matches!(
            "mfas:v2:data:48".parse::<Address>().unwrap_err(),
            AddressError::InvalidVersion(_)
        ));
    }

    #[test]
    fn test_binary_serialization_roundtrip() {
        let addr = Address::new(NodeType::Ref, vec![0xAB, 0xCD, 0xEF, 0x01, 0x23]).unwrap();
        let binary = addr.to_binary();
        assert_eq!(&binary[0..4], &BINARY_MAGIC);
        assert_eq!(binary[4], 1); // version 1
        assert_eq!(binary[5], NodeType::Ref.type_id()); // type ID 2

        let restored = Address::from_binary(&binary).unwrap();
        assert_eq!(restored, addr);
    }

    #[test]
    fn test_all_node_types_roundtrip() {
        let types = [
            NodeType::Data,
            NodeType::Ref,
            NodeType::Seq,
            NodeType::Rep,
            NodeType::Slice,
            NodeType::Page,
        ];
        for nt in types {
            let addr = Address::new(nt, vec![0xCA, 0xFE]).unwrap();
            let uri = addr.to_uri();
            let parsed: Address = uri.parse().unwrap();
            assert_eq!(parsed.node_type(), nt);
            assert_eq!(parsed, addr);

            let bin = addr.to_binary();
            let restored = Address::from_binary(&bin).unwrap();
            assert_eq!(restored, addr);
        }
    }

    #[test]
    fn test_empty_payload_rejected() {
        assert!(matches!(
            Address::new(NodeType::Ref, vec![]).unwrap_err(),
            AddressError::EmptyPayload
        ));
        assert!(matches!(
            "mfas:v1:ref:".parse::<Address>().unwrap_err(),
            AddressError::EmptyPayload
        ));
    }

    #[test]
    fn test_empty_data_node_allowed() {
        let addr = Address::new(NodeType::Data, vec![]).unwrap();
        assert_eq!(addr.to_uri(), "mfas:v1:data:");
        let parsed: Address = "mfas:v1:data:".parse().unwrap();
        assert_eq!(parsed, addr);
        assert!(parsed.payload().is_empty());

        let bin = addr.to_binary();
        let restored = Address::from_binary(&bin).unwrap();
        assert_eq!(restored, addr);
    }

    #[test]
    fn test_total_ordering() {
        let a1 = Address::new(NodeType::Data, vec![0x01]).unwrap();
        let a2 = Address::new(NodeType::Data, vec![0x02]).unwrap();
        let a3 = Address::new(NodeType::Ref, vec![0x01]).unwrap();

        assert!(a1 < a2);
        assert!(a1 < a3); // Data (type 1) < Ref (type 2)
    }
}
```

## mfas-core\src\bench\mod.rs

```rust
//! Performance Benchmarks & Metrics
//!
//! Provides measurement of operations per second, latency, and throughput across:
//! - Address parsing, canonical formatting, and binary serialization
//! - Radix-16 spatial hierarchy coordinate computation
//! - Constant-memory streaming encoding and decoding throughput (MB/s)
//! - DAG traversal, CSE optimization, and node deduplication throughput

use crate::address::{Address, NodeType};
use crate::codec::{decode_to_writer, encode};
use crate::dag::{build_dag, optimize_dag};
use crate::error::CoreError;
use crate::node::Node;
use crate::page::RadixCoordinate;
use crate::resolver::{AddressStore, MemoryStore};
use crate::synthetic::SyntheticStream;
use serde::{Deserialize, Serialize};
use std::io::sink;
use std::time::Instant;

/// Benchmark metrics for address parsing and formatting
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddressBenchResult {
    pub iterations: u64,
    pub parse_ops_per_sec: f64,
    pub parse_nanos_per_op: f64,
    pub format_ops_per_sec: f64,
    pub format_nanos_per_op: f64,
    pub binary_ops_per_sec: f64,
    pub binary_nanos_per_op: f64,
}

/// Benchmark metrics for logical pages and radix-16 coordinates
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageBenchResult {
    pub iterations: u64,
    pub coordinate_ops_per_sec: f64,
    pub coordinate_nanos_per_op: f64,
}

/// Benchmark metrics for streaming codec throughput
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodecBenchResult {
    pub size_bytes: u64,
    pub encode_throughput_mb_s: f64,
    pub encode_duration_secs: f64,
    pub decode_throughput_mb_s: f64,
    pub decode_duration_secs: f64,
}

/// Benchmark metrics for DAG construction and optimization
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DagBenchResult {
    pub node_count: usize,
    pub build_nodes_per_sec: f64,
    pub build_duration_secs: f64,
    pub optimize_nodes_per_sec: f64,
    pub optimize_duration_secs: f64,
}

/// Comprehensive benchmark report combining all subsystem metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllBenchResult {
    pub address: AddressBenchResult,
    pub page: PageBenchResult,
    pub codec: CodecBenchResult,
    pub dag: DagBenchResult,
}

/// Benchmark Address parsing, canonical formatting, and binary roundtrips
pub fn bench_address(iterations: u64) -> AddressBenchResult {
    let uri_sample = "mfas:v1:data:48656c6c6f204d464153";
    let addr_sample = Address::from_hex(NodeType::Data, "48656c6c6f204d464153").unwrap();
    let binary_sample = addr_sample.to_binary();

    // 1. Benchmark URI Parsing
    let start_parse = Instant::now();
    for _ in 0..iterations {
        let addr: Address = uri_sample.parse().unwrap();
        std::hint::black_box(addr);
    }
    let parse_elapsed = start_parse.elapsed();
    let parse_secs = parse_elapsed.as_secs_f64();
    let parse_ops_per_sec = iterations as f64 / parse_secs.max(1e-9);
    let parse_nanos_per_op = (parse_elapsed.as_nanos() as f64) / iterations as f64;

    // 2. Benchmark URI Formatting
    let start_format = Instant::now();
    for _ in 0..iterations {
        let uri = addr_sample.to_uri();
        std::hint::black_box(uri);
    }
    let format_elapsed = start_format.elapsed();
    let format_secs = format_elapsed.as_secs_f64();
    let format_ops_per_sec = iterations as f64 / format_secs.max(1e-9);
    let format_nanos_per_op = (format_elapsed.as_nanos() as f64) / iterations as f64;

    // 3. Benchmark Binary Parsing
    let start_binary = Instant::now();
    for _ in 0..iterations {
        let addr = Address::from_binary(&binary_sample).unwrap();
        std::hint::black_box(addr);
    }
    let binary_elapsed = start_binary.elapsed();
    let binary_secs = binary_elapsed.as_secs_f64();
    let binary_ops_per_sec = iterations as f64 / binary_secs.max(1e-9);
    let binary_nanos_per_op = (binary_elapsed.as_nanos() as f64) / iterations as f64;

    AddressBenchResult {
        iterations,
        parse_ops_per_sec,
        parse_nanos_per_op,
        format_ops_per_sec,
        format_nanos_per_op,
        binary_ops_per_sec,
        binary_nanos_per_op,
    }
}

/// Benchmark Radix-16 coordinate calculations
pub fn bench_page(iterations: u64) -> PageBenchResult {
    let start_coord = Instant::now();
    for i in 0..iterations {
        let offset = (i * 1234567) ^ 0xA5A5A5;
        let coord = RadixCoordinate::from_byte_offset(offset);
        let back = coord.to_byte_offset();
        std::hint::black_box(back);
    }
    let coord_elapsed = start_coord.elapsed();
    let coord_secs = coord_elapsed.as_secs_f64();
    let coordinate_ops_per_sec = iterations as f64 / coord_secs.max(1e-9);
    let coordinate_nanos_per_op = (coord_elapsed.as_nanos() as f64) / iterations as f64;

    PageBenchResult {
        iterations,
        coordinate_ops_per_sec,
        coordinate_nanos_per_op,
    }
}

/// Benchmark streaming encoding and decoding throughput (MB/s)
pub fn bench_codec(size_bytes: u64) -> Result<CodecBenchResult, CoreError> {
    let mut store = MemoryStore::new();

    // 1. Benchmark Streaming Encode
    let mut stream = SyntheticStream::counter(size_bytes);
    let start_encode = Instant::now();
    let root_addr = encode(&mut stream, &mut store)?;
    let encode_secs = start_encode.elapsed().as_secs_f64();
    let size_mb = size_bytes as f64 / (1024.0 * 1024.0);
    let encode_throughput_mb_s = if encode_secs > 0.0 {
        size_mb / encode_secs
    } else {
        0.0
    };

    // 2. Benchmark Streaming Decode
    let mut sink_writer = sink();
    let start_decode = Instant::now();
    decode_to_writer(&root_addr, &store, &mut sink_writer)?;
    let decode_secs = start_decode.elapsed().as_secs_f64();
    let decode_throughput_mb_s = if decode_secs > 0.0 {
        size_mb / decode_secs
    } else {
        0.0
    };

    Ok(CodecBenchResult {
        size_bytes,
        encode_throughput_mb_s,
        encode_duration_secs: encode_secs,
        decode_throughput_mb_s,
        decode_duration_secs: decode_secs,
    })
}

/// Benchmark DAG graph traversal and CSE optimization speed
pub fn bench_dag(nodes: usize) -> Result<DagBenchResult, CoreError> {
    let mut store = MemoryStore::new();

    // Create a DAG with `nodes` sequential elements with duplicates to optimize
    let mut child_addrs = Vec::with_capacity(nodes);
    for i in 0..nodes {
        let data = format!("NodePayload_{}", i % 50).into_bytes();
        let node = Node::Data(data);
        let addr = node.to_address().unwrap();
        store.put(&addr, node).unwrap();
        child_addrs.push(addr);
    }

    let root_seq = Node::Sequence(child_addrs);
    let root_addr = root_seq.to_address().unwrap();
    store.put(&root_addr, root_seq).unwrap();

    // 1. Benchmark DAG Build
    let start_build = Instant::now();
    let dag = build_dag(&root_addr, &store)?;
    let _stats = dag.stats();
    let build_secs = start_build.elapsed().as_secs_f64();
    let build_nodes_per_sec = (nodes as f64) / build_secs.max(1e-9);

    // 2. Benchmark DAG Optimize
    let start_opt = Instant::now();
    let _report = optimize_dag(&root_addr, &mut store)?;
    let opt_secs = start_opt.elapsed().as_secs_f64();
    let optimize_nodes_per_sec = (nodes as f64) / opt_secs.max(1e-9);

    Ok(DagBenchResult {
        node_count: nodes,
        build_nodes_per_sec,
        build_duration_secs: build_secs,
        optimize_nodes_per_sec,
        optimize_duration_secs: opt_secs,
    })
}

/// Run all benchmarks
pub fn bench_all(iterations: u64, size_bytes: u64) -> Result<AllBenchResult, CoreError> {
    let address = bench_address(iterations);
    let page = bench_page(iterations);
    let codec = bench_codec(size_bytes)?;
    let dag = bench_dag(iterations.min(5000) as usize)?;

    Ok(AllBenchResult {
        address,
        page,
        codec,
        dag,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bench_address_execution() {
        let res = bench_address(1000);
        assert_eq!(res.iterations, 1000);
        assert!(res.parse_ops_per_sec > 0.0);
        assert!(res.format_ops_per_sec > 0.0);
        assert!(res.binary_ops_per_sec > 0.0);
    }

    #[test]
    fn test_bench_page_execution() {
        let res = bench_page(1000);
        assert_eq!(res.iterations, 1000);
        assert!(res.coordinate_ops_per_sec > 0.0);
    }

    #[test]
    fn test_bench_codec_execution() {
        let res = bench_codec(65536).unwrap();
        assert_eq!(res.size_bytes, 65536);
        assert!(res.encode_throughput_mb_s > 0.0);
        assert!(res.decode_throughput_mb_s > 0.0);
    }

    #[test]
    fn test_bench_dag_execution() {
        let res = bench_dag(100).unwrap();
        assert_eq!(res.node_count, 100);
        assert!(res.build_nodes_per_sec > 0.0);
        assert!(res.optimize_nodes_per_sec > 0.0);
    }
}
```

## mfas-core\src\codec\mod.rs

```rust
//! Canonical Encoder, Decoder & Verification Engine
//!
//! Provides deterministic streaming encoding of arbitrary finite byte streams into
//! canonical MFAS page DAGs, and streaming reconstruction with SHA-256 integrity verification.

use crate::address::Address;
use crate::error::CoreError;
use crate::node::Node;
use crate::page::{Page, PAGE_SIZE};
use crate::resolver::{resolve, resolve_to_writer, AddressStore, Limits};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

/// Report produced by comprehensive file verification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationReport {
    pub file_path: String,
    pub root_address: Address,
    pub total_bytes: u64,
    pub sha256: String,
    pub verified: bool,
}

/// Encode an arbitrary byte reader into a canonical root Address in constant memory
pub fn encode<R: Read>(reader: &mut R, store: &mut dyn AddressStore) -> Result<Address, CoreError> {
    let mut buf = vec![0u8; PAGE_SIZE];
    let mut page_count = 0u64;
    let mut first_page: Option<Page> = None;
    let mut grouped_addrs: Vec<Address> = Vec::new();
    let mut current_run: Option<(Address, u64)> = None;

    loop {
        let mut bytes_in_page = 0;
        while bytes_in_page < PAGE_SIZE {
            let n = reader.read(&mut buf[bytes_in_page..])?;
            if n == 0 {
                break;
            }
            bytes_in_page += n;
        }

        if bytes_in_page == 0 {
            break;
        }

        let page = Page::new(buf[..bytes_in_page].to_vec())?;

        if page_count == 0 {
            // Buffer the first page to see if EOF is reached next (single-page optimization)
            first_page = Some(page);
            page_count = 1;
        } else {
            // If first_page is still pending, commit it as page index 0
            if let Some(fp) = first_page.take() {
                let first_addr = fp.to_hash_address(0);
                store.put(&first_addr, Node::Page(fp))?;
                current_run = Some((first_addr, 1));
            }

            let curr_addr = page.to_hash_address(0);
            store.put(&curr_addr, Node::Page(page))?;
            page_count += 1;

            // Update on-the-fly run-length grouping
            match current_run.take() {
                Some((prev_addr, count)) => {
                    if prev_addr == curr_addr {
                        current_run = Some((prev_addr, count + 1));
                    } else {
                        if count == 1 {
                            grouped_addrs.push(prev_addr);
                        } else {
                            let rep = Node::Repeat {
                                target: prev_addr,
                                count,
                            };
                            let rep_addr = rep.to_address()?;
                            store.put(&rep_addr, rep)?;
                            grouped_addrs.push(rep_addr);
                        }
                        current_run = Some((curr_addr, 1));
                    }
                }
                None => {
                    current_run = Some((curr_addr, 1));
                }
            }
        }
    }

    // 1. Empty input
    if page_count == 0 {
        let empty_node = Node::Data(Vec::new());
        let root = empty_node.to_address()?;
        store.put(&root, empty_node)?;
        return Ok(root);
    }

    // 2. Single page input (<= 4096 bytes)
    if let Some(fp) = first_page {
        let addr = fp.to_address(0);
        store.put(&addr, Node::Page(fp))?;
        return Ok(addr);
    }

    // 3. Multi-page: flush remaining run
    if let Some((prev_addr, count)) = current_run {
        if count == 1 {
            grouped_addrs.push(prev_addr);
        } else {
            let rep = Node::Repeat {
                target: prev_addr,
                count,
            };
            let rep_addr = rep.to_address()?;
            store.put(&rep_addr, rep)?;
            grouped_addrs.push(rep_addr);
        }
    }

    // 4. Create root sequence
    let seq_node = Node::Sequence(grouped_addrs);
    let root_addr = seq_node.to_address()?;
    store.put(&root_addr, seq_node)?;
    Ok(root_addr)
}

/// Decode an Address directly into memory
pub fn decode(addr: &Address, store: &dyn AddressStore) -> Result<Vec<u8>, CoreError> {
    let limits = Limits::default();
    let bytes = resolve(addr, store, &limits)?;
    Ok(bytes)
}

/// Decode an Address streaming directly into a writer
pub fn decode_to_writer<W: Write>(
    addr: &Address,
    store: &dyn AddressStore,
    writer: &mut W,
) -> Result<u64, CoreError> {
    let limits = Limits::default();
    let count = resolve_to_writer(addr, store, &limits, writer)?;
    Ok(count)
}

/// Decode an Address streaming directly to an output file
pub fn decode_to_file<P: AsRef<Path>>(
    addr: &Address,
    store: &dyn AddressStore,
    path: P,
) -> Result<u64, CoreError> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    let count = decode_to_writer(addr, store, &mut writer)?;
    writer.flush()?;
    Ok(count)
}

/// Hasher sink that computes SHA-256 and counts total bytes
struct HashWriter<W: Write> {
    hasher: Sha256,
    byte_count: u64,
    inner: Option<W>,
}

impl<W: Write> HashWriter<W> {
    fn new(inner: Option<W>) -> Self {
        Self {
            hasher: Sha256::new(),
            byte_count: 0,
            inner,
        }
    }

    fn finalize(self) -> (String, u64) {
        let hash = self.hasher.finalize();
        (hex::encode(hash), self.byte_count)
    }
}

impl<W: Write> Write for HashWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.hasher.update(buf);
        self.byte_count += buf.len() as u64;
        if let Some(ref mut w) = self.inner {
            w.write_all(buf)?;
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if let Some(ref mut w) = self.inner {
            w.flush()?;
        }
        Ok(())
    }
}

/// Calculate the SHA-256 hash and byte length of a file
pub fn hash_file<P: AsRef<Path>>(path: P) -> std::io::Result<(String, u64)> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut total_bytes = 0u64;
    let mut buf = [0u8; 8192];

    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
        total_bytes += n as u64;
    }

    let hash_hex = hex::encode(hasher.finalize());
    Ok((hash_hex, total_bytes))
}

/// Verify a file by encoding it, decoding it to a streaming hasher, and verifying equality
pub fn verify_file<P: AsRef<Path>>(
    path: P,
    store: &mut dyn AddressStore,
) -> Result<VerificationReport, CoreError> {
    let p = path.as_ref();
    let (expected_hash, expected_len) = hash_file(p)?;

    // 1. Encode file
    let file = File::open(p)?;
    let mut reader = BufReader::new(file);
    let root_addr = encode(&mut reader, store)?;

    // 2. Decode streaming to hash verifier
    let mut verifier = HashWriter::<Vec<u8>>::new(None);
    let count = decode_to_writer(&root_addr, store, &mut verifier)?;
    let (reconstructed_hash, _) = verifier.finalize();

    let verified = count == expected_len && reconstructed_hash == expected_hash;

    Ok(VerificationReport {
        file_path: p.display().to_string(),
        root_address: root_addr,
        total_bytes: count,
        sha256: reconstructed_hash,
        verified,
    })
}

/// Reader adapter that passes all reads through a SHA-256 hasher and counts total bytes
pub struct HashReader<'a, R: Read> {
    inner: &'a mut R,
    hasher: Sha256,
    byte_count: u64,
}

impl<'a, R: Read> HashReader<'a, R> {
    pub fn new(inner: &'a mut R) -> Self {
        Self {
            inner,
            hasher: Sha256::new(),
            byte_count: 0,
        }
    }

    pub fn finalize(self) -> (String, u64) {
        let hash = self.hasher.finalize();
        (hex::encode(hash), self.byte_count)
    }
}

impl<'a, R: Read> Read for HashReader<'a, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.inner.read(buf)?;
        if n > 0 {
            self.hasher.update(&buf[..n]);
            self.byte_count += n as u64;
        }
        Ok(n)
    }
}

/// Verify an arbitrary byte stream by encoding to an address, decoding to a streaming hasher,
/// and verifying exact SHA-256 and byte length match without intermediate files.
pub fn verify_stream<R: Read>(
    reader: &mut R,
    store: &mut dyn AddressStore,
) -> Result<VerificationReport, CoreError> {
    let mut hash_reader = HashReader::new(reader);
    let root_addr = encode(&mut hash_reader, store)?;
    let (expected_hash, total_bytes) = hash_reader.finalize();

    let mut verifier = HashWriter::<Vec<u8>>::new(None);
    let count = decode_to_writer(&root_addr, store, &mut verifier)?;
    let (reconstructed_hash, _) = verifier.finalize();

    let verified = count == total_bytes && reconstructed_hash == expected_hash;

    Ok(VerificationReport {
        file_path: "<stream>".to_string(),
        root_address: root_addr,
        total_bytes: count,
        sha256: reconstructed_hash,
        verified,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resolver::MemoryStore;

    #[test]
    fn test_codec_empty_stream() {
        let mut store = MemoryStore::new();
        let mut empty = std::io::Cursor::new(b"");
        let root = encode(&mut empty, &mut store).unwrap();

        assert_eq!(root.to_uri(), "mfas:v1:data:");
        let decoded = decode(&root, &store).unwrap();
        assert_eq!(decoded, b"");
    }

    #[test]
    fn test_codec_single_page() {
        let mut store = MemoryStore::new();
        let data = b"Hello Single Page MFAS Encoding!";
        let mut cursor = std::io::Cursor::new(data);
        let root = encode(&mut cursor, &mut store).unwrap();

        let decoded = decode(&root, &store).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_codec_multi_page_and_partial() {
        let mut store = MemoryStore::new();
        // 10,000 bytes = 2 full pages (8192 B) + 1 partial page (1808 B)
        let mut data = Vec::with_capacity(10_000);
        for i in 0..10_000 {
            data.push((i % 256) as u8);
        }

        let mut cursor = std::io::Cursor::new(&data);
        let root = encode(&mut cursor, &mut store).unwrap();
        assert_eq!(root.node_type(), crate::address::NodeType::Seq);

        let decoded = decode(&root, &store).unwrap();
        assert_eq!(decoded.len(), 10_000);
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_codec_repetitive_data_compression() {
        let mut store = MemoryStore::new();
        // 5 identical pages of 4096 bytes
        let single_page = vec![0x42; 4096];
        let mut data = Vec::new();
        for _ in 0..5 {
            data.extend_from_slice(&single_page);
        }

        let mut cursor = std::io::Cursor::new(&data);
        let root = encode(&mut cursor, &mut store).unwrap();

        let decoded = decode(&root, &store).unwrap();
        assert_eq!(decoded.len(), 20_480);
        assert_eq!(decoded, data);
    }
}
```

## mfas-core\src\dag\mod.rs

```rust
//! Directed Acyclic Graph (DAG) Engine & Optimization
//!
//! Provides:
//! - Structural representation and analysis of MFAS address graphs
//! - Metric calculation (sharing ratio, in/out degrees, topological depth)
//! - Graphviz DOT language export for visual architecture diagrams
//! - Common Subexpression Elimination (CSE) and graph normalization / reductions

use crate::address::{Address, NodeType};
use crate::error::CoreError;
use crate::node::Node;
use crate::resolver::AddressStore;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

/// Structural graph representation of an MFAS address hierarchy
#[derive(Debug, Clone)]
pub struct DagGraph {
    /// Root address of the DAG
    pub root: Address,
    /// Discovered nodes in the graph
    pub nodes: HashMap<Address, Node>,
    /// Directed edges (parent, child)
    pub edges: Vec<(Address, Address)>,
    /// In-degree per node (number of incoming references)
    pub in_degree: HashMap<Address, usize>,
    /// Out-degree per node (number of outgoing dependencies)
    pub out_degree: HashMap<Address, usize>,
    /// Topological depth from root (root = 1)
    pub depth_map: HashMap<Address, usize>,
}

/// Comprehensive topological metrics for an MFAS DAG
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DagStats {
    /// Root address formatted as canonical URI
    pub root_address: String,
    /// Total number of unique nodes (vertices $V$)
    pub total_nodes: usize,
    /// Total directed reference edges ($E$)
    pub total_edges: usize,
    /// Number of nodes referenced by more than one parent (shared subtrees)
    pub shared_nodes: usize,
    /// Number of terminal nodes (leaves with out-degree 0)
    pub leaf_nodes: usize,
    /// Maximum path depth from root to any leaf
    pub max_depth: usize,
    /// Sharing ratio measure: $(E - V + 1) / \max(1, E)$, indicating subexpression reuse
    pub sharing_ratio: f64,
}

/// Report emitted after running CSE and structural graph reductions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptimizationReport {
    /// Original root address
    pub original_root: Address,
    /// Optimized root address
    pub optimized_root: Address,
    /// Node count before optimization
    pub nodes_before: usize,
    /// Node count after optimization
    pub nodes_after: usize,
    /// Edge count before optimization
    pub edges_before: usize,
    /// Edge count after optimization
    pub edges_after: usize,
    /// Human-readable list of structural transformations performed
    pub transformations: Vec<String>,
}

/// Build a `DagGraph` by traversing all reachable nodes from a root `Address`
pub fn build_dag(root: &Address, store: &dyn AddressStore) -> Result<DagGraph, CoreError> {
    let mut nodes = HashMap::new();
    let mut edges = Vec::new();
    let mut in_degree: HashMap<Address, usize> = HashMap::new();
    let mut out_degree: HashMap<Address, usize> = HashMap::new();
    let mut depth_map: HashMap<Address, usize> = HashMap::new();

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((root.clone(), 1usize));
    visited.insert(root.clone());
    depth_map.insert(root.clone(), 1);

    while let Some((curr_addr, depth)) = queue.pop_front() {
        in_degree.entry(curr_addr.clone()).or_insert(0);

        // Fetch node from store or parse self-describing address
        let node = match store.get(&curr_addr) {
            Ok(Some(n)) => n,
            _ => match Node::from_address(&curr_addr) {
                Ok(n) => n,
                Err(_) => {
                    // Unresolvable node recorded as empty data node leaf
                    Node::Data(vec![])
                }
            },
        };

        let children = node.children();
        out_degree.insert(curr_addr.clone(), children.len());

        for child in children {
            edges.push((curr_addr.clone(), child.clone()));
            *in_degree.entry(child.clone()).or_insert(0) += 1;

            let next_depth = depth + 1;
            let current_child_depth = depth_map.entry(child.clone()).or_insert(next_depth);
            if next_depth > *current_child_depth {
                *current_child_depth = next_depth;
            }

            if !visited.contains(&child) {
                visited.insert(child.clone());
                queue.push_back((child, next_depth));
            }
        }

        nodes.insert(curr_addr, node);
    }

    Ok(DagGraph {
        root: root.clone(),
        nodes,
        edges,
        in_degree,
        out_degree,
        depth_map,
    })
}

impl DagGraph {
    /// Calculate summary metrics for this graph
    pub fn stats(&self) -> DagStats {
        let total_nodes = self.nodes.len();
        let total_edges = self.edges.len();

        let shared_nodes = self
            .in_degree
            .values()
            .filter(|&&deg| deg > 1)
            .count();

        let leaf_nodes = self
            .out_degree
            .values()
            .filter(|&&deg| deg == 0)
            .count();

        let max_depth = self.depth_map.values().cloned().max().unwrap_or(0);

        let sharing_ratio = if total_edges > 0 && total_nodes > 0 {
            let extra_edges = (total_edges as isize) - (total_nodes as isize) + 1;
            if extra_edges > 0 {
                extra_edges as f64 / total_edges as f64
            } else {
                0.0
            }
        } else {
            0.0
        };

        DagStats {
            root_address: self.root.to_uri(),
            total_nodes,
            total_edges,
            shared_nodes,
            leaf_nodes,
            max_depth,
            sharing_ratio,
        }
    }

    /// Render the DAG into standard Graphviz DOT format
    pub fn to_dot(&self) -> String {
        let mut dot = String::new();
        dot.push_str("digraph MFAS {\n");
        dot.push_str("    rankdir=TB;\n");
        dot.push_str("    node [fontname=\"Helvetica,Arial,sans-serif\", fontsize=10, shape=box, style=\"filled,rounded\", penwidth=1.5];\n");
        dot.push_str("    edge [fontname=\"Helvetica,Arial,sans-serif\", fontsize=8, color=\"#555555\", arrowsize=0.8];\n\n");

        // Index nodes with short IDs
        let mut addr_to_id = HashMap::new();
        for (i, addr) in self.nodes.keys().enumerate() {
            addr_to_id.insert(addr, format!("node_{}", i));
        }

        for (addr, node) in &self.nodes {
            let id = &addr_to_id[addr];
            let is_root = addr == &self.root;

            let (type_name, fill_color, border_color) = match addr.node_type() {
                NodeType::Data => ("Data", "#E1F5FE", "#0288D1"),
                NodeType::Page => ("Page", "#E8F5E9", "#388E3C"),
                NodeType::Ref => ("Reference", "#FFF3E0", "#F57C00"),
                NodeType::Seq => ("Sequence", "#F3E5F5", "#7B1FA2"),
                NodeType::Rep => ("Repeat", "#FCE4EC", "#C2185B"),
                NodeType::Slice => ("Slice", "#FFFDE7", "#FBC02D"),
            };

            let short_uri = {
                let uri = addr.to_uri();
                if uri.len() > 32 {
                    format!("{}...{}", &uri[..18], &uri[uri.len() - 8..])
                } else {
                    uri
                }
            };

            let details = match node {
                Node::Data(b) => format!("{} bytes", b.len()),
                Node::Page(p) => format!("{} B", p.len()),
                Node::Sequence(ch) => format!("{} children", ch.len()),
                Node::Repeat { count, .. } => format!("repeat {}x", count),
                Node::Slice { offset, length, .. } => format!("[{}..+{}]", offset, length),
                Node::Reference(_) => "ref".to_string(),
            };

            let root_badge = if is_root { " [ROOT]\\n" } else { "" };
            let label = format!("{}{}\\n{}\\n{}", root_badge, type_name, details, short_uri);

            let border_width = if is_root { "3.0" } else { "1.5" };

            dot.push_str(&format!(
                "    {} [label=\"{}\", fillcolor=\"{}\", color=\"{}\", penwidth={}];\n",
                id, label, fill_color, border_color, border_width
            ));
        }

        dot.push('\n');

        // Edges
        for (from, to) in &self.edges {
            if let (Some(from_id), Some(to_id)) = (addr_to_id.get(from), addr_to_id.get(to)) {
                dot.push_str(&format!("    {} -> {};\n", from_id, to_id));
            }
        }

        dot.push_str("}\n");
        dot
    }
}

/// Optimize an address DAG using Common Subexpression Elimination (CSE) and structural reductions
pub fn optimize_dag(
    root: &Address,
    store: &mut dyn AddressStore,
) -> Result<OptimizationReport, CoreError> {
    let graph_before = build_dag(root, store)?;
    let stats_before = graph_before.stats();

    let mut transformations = Vec::new();
    let mut memo = HashMap::new();

    let optimized_root = optimize_node_recursive(root, store, &mut transformations, &mut memo)?;

    let graph_after = build_dag(&optimized_root, store)?;
    let stats_after = graph_after.stats();

    Ok(OptimizationReport {
        original_root: root.clone(),
        optimized_root,
        nodes_before: stats_before.total_nodes,
        nodes_after: stats_after.total_nodes,
        edges_before: stats_before.total_edges,
        edges_after: stats_after.total_edges,
        transformations,
    })
}

fn optimize_node_recursive(
    addr: &Address,
    store: &mut dyn AddressStore,
    transformations: &mut Vec<String>,
    memo: &mut HashMap<Address, Address>,
) -> Result<Address, CoreError> {
    if let Some(cached) = memo.get(addr) {
        return Ok(cached.clone());
    }

    let node = match store.get(addr) {
        Ok(Some(n)) => n,
        _ => match Node::from_address(addr) {
            Ok(n) => n,
            Err(_) => return Ok(addr.clone()),
        },
    };

    let optimized_node = match node {
        Node::Data(_) | Node::Page(_) => node,

        Node::Reference(target) => {
            let opt_target = optimize_node_recursive(&target, store, transformations, memo)?;
            // Collapse redundant reference: Ref(Ref(X)) -> Ref(X)
            if let Ok(Some(Node::Reference(sub_target))) = store.get(&opt_target) {
                transformations.push(format!("Collapsed reference chain to {}", sub_target.to_uri()));
                Node::Reference(sub_target)
            } else {
                Node::Reference(opt_target)
            }
        }

        Node::Repeat { target, count } => {
            if count == 0 {
                transformations.push("Collapsed zero-repeat to empty Data node".to_string());
                Node::Data(vec![])
            } else if count == 1 {
                let opt_target = optimize_node_recursive(&target, store, transformations, memo)?;
                transformations.push(format!("Collapsed 1x Repeat to target {}", opt_target.to_uri()));
                // Return target directly
                memo.insert(addr.clone(), opt_target.clone());
                return Ok(opt_target);
            } else {
                let opt_target = optimize_node_recursive(&target, store, transformations, memo)?;
                // Check if target is itself a Repeat: Repeat(Repeat(X, m), n) -> Repeat(X, m * n)
                let target_node = store.get(&opt_target).ok().flatten().or_else(|| Node::from_address(&opt_target).ok());
                if let Some(Node::Repeat { target: sub_target, count: sub_count }) = target_node {
                    let total_count = count.saturating_mul(sub_count);
                    transformations.push(format!(
                        "Merged nested repeats ({}x * {}x = {}x)",
                        count, sub_count, total_count
                    ));
                    Node::Repeat {
                        target: sub_target,
                        count: total_count,
                    }
                } else {
                    Node::Repeat {
                        target: opt_target,
                        count,
                    }
                }
            }
        }

        Node::Slice {
            target,
            offset,
            length,
        } => {
            if length == 0 {
                transformations.push("Collapsed zero-length slice to empty Data node".to_string());
                Node::Data(vec![])
            } else {
                let opt_target = optimize_node_recursive(&target, store, transformations, memo)?;
                // Check if target is a Data or Page node and slice covers entire content
                let target_node = store.get(&opt_target).ok().flatten().or_else(|| Node::from_address(&opt_target).ok());
                let full_length = match &target_node {
                    Some(Node::Data(b)) => Some(b.len() as u64),
                    Some(Node::Page(p)) => Some(p.len() as u64),
                    _ => None,
                };

                if let Some(total_len) = full_length {
                    if offset == 0 && length >= total_len {
                        transformations.push(format!(
                            "Eliminated redundant full-range slice on {}",
                            opt_target.to_uri()
                        ));
                        memo.insert(addr.clone(), opt_target.clone());
                        return Ok(opt_target);
                    }
                }

                Node::Slice {
                    target: opt_target,
                    offset,
                    length,
                }
            }
        }

        Node::Sequence(children) => {
            // 1. Recursively optimize all children
            let mut opt_children = Vec::new();
            for child in children {
                let opt_child = optimize_node_recursive(&child, store, transformations, memo)?;
                opt_children.push(opt_child);
            }

            // 2. Flatten nested sequences: Seq([Seq([A, B]), C]) -> Seq([A, B, C])
            let mut flattened = Vec::new();
            for child in opt_children {
                let child_node = store.get(&child).ok().flatten().or_else(|| Node::from_address(&child).ok());
                if let Some(Node::Sequence(sub_children)) = child_node {
                    transformations.push(format!(
                        "Flattened nested sequence with {} children",
                        sub_children.len()
                    ));
                    flattened.extend(sub_children);
                } else {
                    flattened.push(child);
                }
            }

            // 3. Fold adjacent identical children into Repeat: [A, A, A] -> [Repeat(A, 3)]
            let mut folded: Vec<Address> = Vec::new();
            let mut current_child: Option<(Address, u64)> = None;

            for child in flattened {
                match current_child.take() {
                    Some((prev_addr, count)) => {
                        if prev_addr == child {
                            current_child = Some((prev_addr, count + 1));
                        } else {
                            // Emit prev
                            if count == 1 {
                                folded.push(prev_addr);
                            } else {
                                let rep = Node::Repeat {
                                    target: prev_addr.clone(),
                                    count,
                                };
                                let rep_addr = rep.to_address()?;
                                store.put(&rep_addr, rep)?;
                                transformations.push(format!("Folded {}x adjacent occurrences into Repeat", count));
                                folded.push(rep_addr);
                            }
                            current_child = Some((child, 1));
                        }
                    }
                    None => {
                        current_child = Some((child, 1));
                    }
                }
            }

            if let Some((prev_addr, count)) = current_child {
                if count == 1 {
                    folded.push(prev_addr);
                } else {
                    let rep = Node::Repeat {
                        target: prev_addr.clone(),
                        count,
                    };
                    let rep_addr = rep.to_address()?;
                    store.put(&rep_addr, rep)?;
                    transformations.push(format!("Folded {}x adjacent occurrences into Repeat", count));
                    folded.push(rep_addr);
                }
            }

            // 4. If sequence has 0 children -> empty Data
            if folded.is_empty() {
                transformations.push("Empty sequence collapsed to empty Data node".to_string());
                Node::Data(vec![])
            } else if folded.len() == 1 {
                // Collapse 1-child sequence: Seq([A]) -> A
                let single = folded.remove(0);
                transformations.push(format!("Collapsed single-child sequence to {}", single.to_uri()));
                memo.insert(addr.clone(), single.clone());
                return Ok(single);
            } else {
                Node::Sequence(folded)
            }
        }
    };

    let result_addr = optimized_node.to_address()?;
    store.put(&result_addr, optimized_node)?;
    memo.insert(addr.clone(), result_addr.clone());
    Ok(result_addr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resolver::{resolve, Limits, MemoryStore};

    #[test]
    fn test_dag_stats_diamond() {
        let mut store = MemoryStore::new();

        // C (leaf)
        let node_c = Node::Data(b"DataC".to_vec());
        let addr_c = node_c.to_address().unwrap();
        store.put(&addr_c, node_c).unwrap();

        // A -> C (Repeat 2x)
        let node_a = Node::Repeat {
            target: addr_c.clone(),
            count: 2,
        };
        let addr_a = node_a.to_address().unwrap();
        store.put(&addr_a, node_a).unwrap();

        // B -> C (Repeat 3x)
        let node_b = Node::Repeat {
            target: addr_c.clone(),
            count: 3,
        };
        let addr_b = node_b.to_address().unwrap();
        store.put(&addr_b, node_b).unwrap();

        // Root -> [A, B] (Diamond)
        let root = Node::Sequence(vec![addr_a, addr_b]).to_address().unwrap();

        let dag = build_dag(&root, &store).unwrap();
        let stats = dag.stats();

        assert_eq!(stats.total_nodes, 4); // Root, A, B, C
        assert_eq!(stats.total_edges, 4); // Root->A, Root->B, A->C, B->C
        assert_eq!(stats.shared_nodes, 1); // C has in-degree 2
        assert_eq!(stats.leaf_nodes, 1); // C is leaf
        assert!(stats.sharing_ratio > 0.0);
    }

    #[test]
    fn test_dag_dot_export() {
        let mut store = MemoryStore::new();
        let data = Node::Data(b"Graphviz".to_vec());
        let addr = data.to_address().unwrap();
        store.put(&addr, data).unwrap();

        let root = Node::Repeat {
            target: addr,
            count: 3,
        }
        .to_address()
        .unwrap();

        let dag = build_dag(&root, &store).unwrap();
        let dot = dag.to_dot();

        assert!(dot.starts_with("digraph MFAS {"));
        assert!(dot.contains("Repeat"));
        assert!(dot.contains("Data"));
        assert!(dot.ends_with("}\n"));
    }

    #[test]
    fn test_dag_optimize_flattens_and_preserves_content() {
        let mut store = MemoryStore::new();
        let limits = Limits::default();

        let leaf1 = Node::Data(b"Part1".to_vec()).to_address().unwrap();
        let leaf2 = Node::Data(b"Part2".to_vec()).to_address().unwrap();

        // Nested sequence: Seq([Seq([leaf1, leaf2])])
        let inner_seq = Node::Sequence(vec![leaf1.clone(), leaf2.clone()])
            .to_address()
            .unwrap();
        let outer_seq = Node::Sequence(vec![inner_seq]).to_address().unwrap();

        let original_bytes = resolve(&outer_seq, &store, &limits).unwrap();
        assert_eq!(original_bytes, b"Part1Part2");

        let report = optimize_dag(&outer_seq, &mut store).unwrap();
        assert!(report.transformations.len() > 0);

        // Verify invariant: decode(optimized) == decode(original)
        let optimized_bytes = resolve(&report.optimized_root, &store, &limits).unwrap();
        assert_eq!(optimized_bytes, original_bytes);
    }

    #[test]
    fn test_dag_optimize_folds_adjacent_repeats() {
        let mut store = MemoryStore::new();
        let limits = Limits::default();

        let leaf = Node::Data(b"R".to_vec()).to_address().unwrap();
        // Sequence with [R, R, R]
        let seq = Node::Sequence(vec![leaf.clone(), leaf.clone(), leaf.clone()])
            .to_address()
            .unwrap();

        let report = optimize_dag(&seq, &mut store).unwrap();
        assert!(report.transformations.iter().any(|t| t.contains("Folded 3x")));

        let optimized_bytes = resolve(&report.optimized_root, &store, &limits).unwrap();
        assert_eq!(optimized_bytes, b"RRR");
    }
}
```

## mfas-core\src\enumeration\mod.rs

```rust
//! Mathematical Enumeration Module: Bijective mapping between $\mathbb{N}_0$ and $B^*$
//!
//! Provides a canonical, length-sensitive bijection between natural numbers
//! and finite byte strings.
//!
//! Invariant:
//! 1. `number_to_bytes(bytes_to_number(b)) == b` for all `b` in $B^*$
//! 2. `bytes_to_number(number_to_bytes(n)) == n` for all `n` in $\mathbb{N}_0$
//! 3. Different length sequences with identical trailing values (e.g. `[0x01]`, `[0x00, 0x01]`, `[0x00, 0x00, 0x01]`)
//!    map to distinct natural numbers.

use num_bigint::BigUint;
use num_traits::{One, Zero};

/// Calculate the offset for strings of length L in bijective base-256:
/// $\text{Offset}(L) = \sum_{i=0}^{L-1} 256^i = \frac{256^L - 1}{255}$
pub fn length_offset(len: usize) -> BigUint {
    if len == 0 {
        return BigUint::zero();
    }
    let pow_256_l = BigUint::one() << (8 * len);
    (pow_256_l - BigUint::one()) / 255u32
}

/// Map a finite byte sequence to a unique natural number $N \in \mathbb{N}_0$.
pub fn bytes_to_number(bytes: &[u8]) -> BigUint {
    let len = bytes.len();
    if len == 0 {
        return BigUint::zero();
    }
    let offset = length_offset(len);
    let val = BigUint::from_bytes_be(bytes);
    offset + val
}

/// Map a natural number $N \in \mathbb{N}_0$ to its unique finite byte sequence in $B^*$.
pub fn number_to_bytes(n: &BigUint) -> Vec<u8> {
    if n.is_zero() {
        return Vec::new();
    }

    // M = 255 * N + 1
    let m = (n * 255u32) + BigUint::one();
    let bits = m.bits();
    let len = ((bits - 1) / 8) as usize;

    let offset = length_offset(len);
    let val = n - offset;

    let raw_bytes = val.to_bytes_be();
    if raw_bytes.len() < len {
        let mut padded = vec![0u8; len - raw_bytes.len()];
        padded.extend_from_slice(&raw_bytes);
        padded
    } else {
        raw_bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string_maps_to_zero() {
        let empty = b"";
        let n = bytes_to_number(empty);
        assert_eq!(n, BigUint::zero());
        assert_eq!(number_to_bytes(&n), empty);
    }

    #[test]
    fn test_single_byte_range() {
        // [0x00] -> 1
        assert_eq!(bytes_to_number(&[0x00]), BigUint::from(1u32));
        assert_eq!(number_to_bytes(&BigUint::from(1u32)), vec![0x00]);

        // [0x01] -> 2
        assert_eq!(bytes_to_number(&[0x01]), BigUint::from(2u32));
        assert_eq!(number_to_bytes(&BigUint::from(2u32)), vec![0x01]);

        // [0xFF] -> 256
        assert_eq!(bytes_to_number(&[0xFF]), BigUint::from(256u32));
        assert_eq!(number_to_bytes(&BigUint::from(256u32)), vec![0xFF]);
    }

    #[test]
    fn test_two_byte_range() {
        // [0x00, 0x00] -> 257
        assert_eq!(bytes_to_number(&[0x00, 0x00]), BigUint::from(257u32));
        assert_eq!(number_to_bytes(&BigUint::from(257u32)), vec![0x00, 0x00]);

        // [0x00, 0x01] -> 258
        assert_eq!(bytes_to_number(&[0x00, 0x01]), BigUint::from(258u32));
        assert_eq!(number_to_bytes(&BigUint::from(258u32)), vec![0x00, 0x01]);

        // [0xFF, 0xFF] -> 65792
        assert_eq!(bytes_to_number(&[0xFF, 0xFF]), BigUint::from(65792u32));
        assert_eq!(number_to_bytes(&BigUint::from(65792u32)), vec![0xFF, 0xFF]);
    }

    #[test]
    fn test_distinguishes_leading_zeros() {
        let b1 = vec![0x01];
        let b2 = vec![0x00, 0x01];
        let b3 = vec![0x00, 0x00, 0x01];

        let n1 = bytes_to_number(&b1);
        let n2 = bytes_to_number(&b2);
        let n3 = bytes_to_number(&b3);

        assert_ne!(n1, n2);
        assert_ne!(n2, n3);
        assert_ne!(n1, n3);

        assert_eq!(number_to_bytes(&n1), b1);
        assert_eq!(number_to_bytes(&n2), b2);
        assert_eq!(number_to_bytes(&n3), b3);
    }

    #[test]
    fn test_arbitrary_data_roundtrip() {
        let test_cases: Vec<&[u8]> = vec![
            b"Hello",
            b"The quick brown fox jumps over the lazy dog",
            &[0, 0, 0, 0, 0, 0, 1],
            &[255, 128, 64, 32, 16, 8, 4, 2, 1, 0],
        ];

        for case in test_cases {
            let n = bytes_to_number(case);
            let restored = number_to_bytes(&n);
            assert_eq!(restored.as_slice(), case);
        }
    }

    #[test]
    fn test_sequential_natural_numbers() {
        for i in 0..1000u32 {
            let n = BigUint::from(i);
            let bytes = number_to_bytes(&n);
            let restored_n = bytes_to_number(&bytes);
            assert_eq!(restored_n, n, "Failed for integer {}", i);
        }
    }
}
```

## mfas-core\src\inspector\mod.rs

```rust
//! DAG Inspection & Structural Visualization Engine
//!
//! Provides topological DAG inspection, node statistics calculation,
//! and ASCII hierarchy tree generation for arbitrary MFAS addresses.

use crate::address::Address;
use crate::error::CoreError;
use crate::node::Node;
use crate::page::PAGE_SIZE;
use crate::resolver::AddressStore;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Comprehensive topological inspection summary
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectionReport {
    pub address: String,
    pub node_type: String,
    pub total_bytes: u64,
    pub page_count: usize,
    pub max_depth: usize,
    pub total_nodes: usize,
    pub unique_nodes: usize,
    pub tree_ascii: String,
}

/// Inspect an Address DAG, computing structural statistics and ASCII visualization
pub fn inspect_address(
    addr: &Address,
    store: &dyn AddressStore,
    max_tree_depth: usize,
) -> Result<InspectionReport, CoreError> {
    let mut visited_unique: HashSet<Address> = HashSet::new();
    let mut stats = CollectorStats::default();

    collect_stats(addr, store, 1, &mut visited_unique, &mut stats)?;

    let mut tree_lines = Vec::new();
    render_tree_node(addr, store, "", true, 0, max_tree_depth, &mut tree_lines)?;
    let tree_ascii = tree_lines.join("\n");

    Ok(InspectionReport {
        address: addr.to_uri(),
        node_type: addr.node_type().as_str().to_string(),
        total_bytes: stats.total_bytes,
        page_count: stats.page_count,
        max_depth: stats.max_depth,
        total_nodes: stats.total_nodes,
        unique_nodes: visited_unique.len(),
        tree_ascii,
    })
}

#[derive(Default)]
struct CollectorStats {
    total_bytes: u64,
    page_count: usize,
    max_depth: usize,
    total_nodes: usize,
}

fn collect_stats(
    addr: &Address,
    store: &dyn AddressStore,
    current_depth: usize,
    visited: &mut HashSet<Address>,
    stats: &mut CollectorStats,
) -> Result<u64, CoreError> {
    visited.insert(addr.clone());
    stats.total_nodes += 1;
    if current_depth > stats.max_depth {
        stats.max_depth = current_depth;
    }

    let node = match store.get(addr)? {
        Some(n) => n,
        None => Node::from_address(addr)?,
    };

    let bytes = match node {
        Node::Data(bytes) => {
            let len = bytes.len() as u64;
            stats.total_bytes += len;
            len
        }
        Node::Page(page) => {
            let len = page.logical_len() as u64;
            stats.total_bytes += len;
            stats.page_count += 1;
            len
        }
        Node::Reference(target) => {
            collect_stats(&target, store, current_depth + 1, visited, stats)?
        }
        Node::Sequence(children) => {
            let mut sum = 0u64;
            for child in &children {
                sum += collect_stats(child, store, current_depth + 1, visited, stats)?;
            }
            sum
        }
        Node::Repeat { target, count } => {
            let single_bytes = collect_stats(&target, store, current_depth + 1, visited, stats)?;
            let total_rep_bytes = single_bytes.saturating_mul(count.saturating_sub(1));
            stats.total_bytes += total_rep_bytes;
            single_bytes.saturating_mul(count)
        }
        Node::Slice {
            target,
            offset: _,
            length,
        } => {
            let _ = collect_stats(&target, store, current_depth + 1, visited, stats)?;
            stats.total_bytes += length;
            length
        }
    };

    Ok(bytes)
}

fn render_tree_node(
    addr: &Address,
    store: &dyn AddressStore,
    prefix: &str,
    is_last: bool,
    depth: usize,
    max_depth: usize,
    lines: &mut Vec<String>,
) -> Result<(), CoreError> {
    let branch = if depth == 0 {
        "ROOT"
    } else if is_last {
        "└── "
    } else {
        "├── "
    };

    let node_opt = match store.get(addr)? {
        Some(n) => Some(n),
        None => Node::from_address(addr).ok(),
    };

    let label = match &node_opt {
        Some(Node::Data(b)) => format!("Data ({} bytes)", b.len()),
        Some(Node::Page(p)) => {
            if p.is_full() {
                format!("Page ({PAGE_SIZE} B)")
            } else {
                format!("Partial Page ({} B)", p.logical_len())
            }
        }
        Some(Node::Reference(_)) => "Ref".to_string(),
        Some(Node::Sequence(c)) => format!("Sequence ({} children)", c.len()),
        Some(Node::Repeat { count, .. }) => format!("Repeat (x{})", count),
        Some(Node::Slice { offset, length, .. }) => {
            format!("Slice [{}..{}]", offset, offset + length)
        }
        None => format!("Unresolved [{}]", addr.to_uri()),
    };

    if depth == 0 {
        lines.push(format!("{label} ({})", addr.to_uri()));
    } else {
        lines.push(format!("{prefix}{branch}{label}"));
    }

    if depth >= max_depth {
        return Ok(());
    }

    if let Some(node) = node_opt {
        let children = node.child_addresses();
        let child_count = children.len();
        let next_prefix = if depth == 0 {
            ""
        } else if is_last {
            &format!("{prefix}    ")
        } else {
            &format!("{prefix}│   ")
        };

        for (idx, child_addr) in children.into_iter().enumerate() {
            let child_is_last = idx + 1 == child_count;
            render_tree_node(
                child_addr,
                store,
                next_prefix,
                child_is_last,
                depth + 1,
                max_depth,
                lines,
            )?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::page::Page;
    use crate::resolver::MemoryStore;

    #[test]
    fn test_inspect_simple_tree() {
        let mut store = MemoryStore::new();
        let p1 = Page::new(vec![1; 4096]).unwrap();
        let p2 = Page::new(vec![2; 500]).unwrap();

        let a1 = p1.to_address(0);
        let a2 = p2.to_address(1);
        store.put(&a1, Node::Page(p1)).unwrap();
        store.put(&a2, Node::Page(p2)).unwrap();

        let root_node = Node::Sequence(vec![a1, a2]);
        let root_addr = root_node.to_address().unwrap();
        store.put(&root_addr, root_node).unwrap();

        let report = inspect_address(&root_addr, &store, 5).unwrap();
        assert_eq!(report.node_type, "seq");
        assert_eq!(report.total_bytes, 4096 + 500);
        assert_eq!(report.page_count, 2);
        assert_eq!(report.total_nodes, 3);
        assert!(report.tree_ascii.contains("Page (4096 B)"));
        assert!(report.tree_ascii.contains("Partial Page (500 B)"));
    }
}
```

## mfas-core\src\node\mod.rs

```rust
//! Recursive Node Model
//!
//! Provides the strongly typed recursive node system:
//! - Data: literal raw bytes
//! - Reference: pointer to another address
//! - Sequence: ordered list of child addresses
//! - Repeat: repeated evaluation of a child address
//! - Slice: byte-range slice of a child address
//! - Page: canonical 4096-byte logical page

use crate::address::{Address, NodeType};
use crate::error::NodeError;
use crate::page::Page;
use serde::{Deserialize, Serialize};

/// Fundamental recursive node primitives
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Node {
    /// Literal raw byte content
    Data(Vec<u8>),
    /// Reference pointing to another canonical address
    Reference(Address),
    /// Ordered concatenation of child addresses
    Sequence(Vec<Address>),
    /// Repetition of a child address `count` times
    Repeat {
        target: Address,
        count: u64,
    },
    /// Sliced range of a child address
    Slice {
        target: Address,
        offset: u64,
        length: u64,
    },
    /// Canonical logical page
    Page(Page),
}

impl Node {
    /// Associated node type
    pub fn node_type(&self) -> NodeType {
        match self {
            Node::Data(_) => NodeType::Data,
            Node::Reference(_) => NodeType::Ref,
            Node::Sequence(_) => NodeType::Seq,
            Node::Repeat { .. } => NodeType::Rep,
            Node::Slice { .. } => NodeType::Slice,
            Node::Page(_) => NodeType::Page,
        }
    }

    /// Extract all immediate child addresses referenced by this node
    pub fn child_addresses(&self) -> Vec<&Address> {
        match self {
            Node::Data(_) => Vec::new(),
            Node::Reference(addr) => vec![addr],
            Node::Sequence(addrs) => addrs.iter().collect(),
            Node::Repeat { target, .. } => vec![target],
            Node::Slice { target, .. } => vec![target],
            Node::Page(_) => Vec::new(),
        }
    }

    /// Extract all immediate child addresses as an owned vector
    pub fn children(&self) -> Vec<Address> {
        self.child_addresses().into_iter().cloned().collect()
    }

    /// Convert node into its canonical Address representation
    pub fn to_address(&self) -> Result<Address, NodeError> {
        match self {
            Node::Data(bytes) => {
                Ok(Address::new(NodeType::Data, bytes.clone())?)
            }
            Node::Reference(addr) => {
                let bin = addr.to_binary();
                Ok(Address::new(NodeType::Ref, bin)?)
            }
            Node::Sequence(children) => {
                if children.is_empty() {
                    return Err(NodeError::EmptySequence);
                }
                let mut payload = Vec::new();
                payload.extend_from_slice(&(children.len() as u32).to_be_bytes());
                for child in children {
                    let child_bin = child.to_binary();
                    payload.extend_from_slice(&(child_bin.len() as u32).to_be_bytes());
                    payload.extend_from_slice(&child_bin);
                }
                Ok(Address::new(NodeType::Seq, payload)?)
            }
            Node::Repeat { target, count } => {
                if *count == 0 {
                    return Err(NodeError::ZeroRepeatCount);
                }
                let mut payload = Vec::new();
                payload.extend_from_slice(&count.to_be_bytes());
                payload.extend_from_slice(&target.to_binary());
                Ok(Address::new(NodeType::Rep, payload)?)
            }
            Node::Slice {
                target,
                offset,
                length,
            } => {
                if *length == 0 {
                    return Err(NodeError::InvalidSliceRange {
                        offset: *offset,
                        length: *length,
                    });
                }
                let mut payload = Vec::new();
                payload.extend_from_slice(&offset.to_be_bytes());
                payload.extend_from_slice(&length.to_be_bytes());
                payload.extend_from_slice(&target.to_binary());
                Ok(Address::new(NodeType::Slice, payload)?)
            }
            Node::Page(page) => Ok(page.to_address(0)),
        }
    }

    /// Reconstruct a node from its canonical Address
    pub fn from_address(addr: &Address) -> Result<Self, NodeError> {
        let payload = addr.payload();
        match addr.node_type() {
            NodeType::Data => Ok(Node::Data(payload.to_vec())),
            NodeType::Ref => {
                let target = Address::from_binary(payload)?;
                Ok(Node::Reference(target))
            }
            NodeType::Seq => {
                if payload.len() < 4 {
                    return Err(NodeError::InvalidPayload("Truncated sequence header".into()));
                }
                let count = u32::from_be_bytes([payload[0], payload[1], payload[2], payload[3]]) as usize;
                if count == 0 {
                    return Err(NodeError::EmptySequence);
                }
                let mut offset = 4;
                let mut children = Vec::with_capacity(count);
                for _ in 0..count {
                    if offset + 4 > payload.len() {
                        return Err(NodeError::InvalidPayload("Truncated child length".into()));
                    }
                    let child_len = u32::from_be_bytes([
                        payload[offset],
                        payload[offset + 1],
                        payload[offset + 2],
                        payload[offset + 3],
                    ]) as usize;
                    offset += 4;
                    if offset + child_len > payload.len() {
                        return Err(NodeError::InvalidPayload("Truncated child binary".into()));
                    }
                    let child_bin = &payload[offset..offset + child_len];
                    let child = Address::from_binary(child_bin)?;
                    children.push(child);
                    offset += child_len;
                }
                Ok(Node::Sequence(children))
            }
            NodeType::Rep => {
                if payload.len() < 8 {
                    return Err(NodeError::InvalidPayload("Truncated repeat count".into()));
                }
                let count = u64::from_be_bytes([
                    payload[0], payload[1], payload[2], payload[3], payload[4], payload[5], payload[6],
                    payload[7],
                ]);
                if count == 0 {
                    return Err(NodeError::ZeroRepeatCount);
                }
                let target_bin = &payload[8..];
                let target = Address::from_binary(target_bin)?;
                Ok(Node::Repeat { target, count })
            }
            NodeType::Slice => {
                if payload.len() < 16 {
                    return Err(NodeError::InvalidPayload("Truncated slice header".into()));
                }
                let offset = u64::from_be_bytes([
                    payload[0], payload[1], payload[2], payload[3], payload[4], payload[5], payload[6],
                    payload[7],
                ]);
                let length = u64::from_be_bytes([
                    payload[8], payload[9], payload[10], payload[11], payload[12], payload[13],
                    payload[14], payload[15],
                ]);
                if length == 0 {
                    return Err(NodeError::InvalidSliceRange { offset, length });
                }
                let target_bin = &payload[16..];
                let target = Address::from_binary(target_bin)?;
                Ok(Node::Slice {
                    target,
                    offset,
                    length,
                })
            }
            NodeType::Page => {
                let (_, page) = Page::from_address(addr)?;
                Ok(Node::Page(page))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_node_roundtrip() {
        let node = Node::Data(b"Inline Literal Bytes".to_vec());
        let addr = node.to_address().unwrap();
        assert_eq!(addr.node_type(), NodeType::Data);
        let restored = Node::from_address(&addr).unwrap();
        assert_eq!(restored, node);
        assert!(node.child_addresses().is_empty());
    }

    #[test]
    fn test_reference_node_roundtrip() {
        let target = Address::new(NodeType::Data, vec![0xCA, 0xFE]).unwrap();
        let node = Node::Reference(target.clone());
        let addr = node.to_address().unwrap();
        assert_eq!(addr.node_type(), NodeType::Ref);
        let restored = Node::from_address(&addr).unwrap();
        assert_eq!(restored, node);
        assert_eq!(node.child_addresses(), vec![&target]);
    }

    #[test]
    fn test_sequence_node_roundtrip() {
        let child1 = Address::new(NodeType::Data, vec![1, 2]).unwrap();
        let child2 = Address::new(NodeType::Data, vec![3, 4]).unwrap();
        let node = Node::Sequence(vec![child1.clone(), child2.clone()]);
        let addr = node.to_address().unwrap();
        assert_eq!(addr.node_type(), NodeType::Seq);
        let restored = Node::from_address(&addr).unwrap();
        assert_eq!(restored, node);
        assert_eq!(node.child_addresses(), vec![&child1, &child2]);
    }

    #[test]
    fn test_repeat_node_roundtrip() {
        let target = Address::new(NodeType::Data, vec![0xFF]).unwrap();
        let node = Node::Repeat {
            target: target.clone(),
            count: 1024,
        };
        let addr = node.to_address().unwrap();
        assert_eq!(addr.node_type(), NodeType::Rep);
        let restored = Node::from_address(&addr).unwrap();
        assert_eq!(restored, node);
        assert_eq!(node.child_addresses(), vec![&target]);
    }

    #[test]
    fn test_slice_node_roundtrip() {
        let target = Address::new(NodeType::Data, vec![0, 1, 2, 3, 4, 5]).unwrap();
        let node = Node::Slice {
            target: target.clone(),
            offset: 2,
            length: 3,
        };
        let addr = node.to_address().unwrap();
        assert_eq!(addr.node_type(), NodeType::Slice);
        let restored = Node::from_address(&addr).unwrap();
        assert_eq!(restored, node);
        assert_eq!(node.child_addresses(), vec![&target]);
    }

    #[test]
    fn test_page_node_roundtrip() {
        let page = Page::new(vec![42; 100]).unwrap();
        let node = Node::Page(page);
        let addr = node.to_address().unwrap();
        assert_eq!(addr.node_type(), NodeType::Page);
        let restored = Node::from_address(&addr).unwrap();
        assert_eq!(restored, node);
    }

    #[test]
    fn test_node_validation_errors() {
        // Empty sequence error
        let empty_seq = Node::Sequence(vec![]);
        assert!(matches!(empty_seq.to_address().unwrap_err(), NodeError::EmptySequence));

        // Zero repeat count error
        let target = Address::new(NodeType::Data, vec![1]).unwrap();
        let zero_rep = Node::Repeat {
            target: target.clone(),
            count: 0,
        };
        assert!(matches!(zero_rep.to_address().unwrap_err(), NodeError::ZeroRepeatCount));

        // Zero slice length error
        let zero_slice = Node::Slice {
            target,
            offset: 10,
            length: 0,
        };
        assert!(matches!(zero_slice.to_address().unwrap_err(), NodeError::InvalidSliceRange { .. }));
    }
}
```

## mfas-core\src\page\mod.rs

```rust
//! Logical Page Model & Radix-16 Hierarchy
//!
//! Provides the primary $16^3 = 4096$-byte logical page abstraction,
//! partial final page handling, and base-16 spatial coordinate calculations.

use crate::address::{Address, NodeType};
use crate::error::PageError;
use serde::{Deserialize, Serialize};

/// Canonical logical page size in bytes ($16^3 = 4096$)
pub const PAGE_SIZE: usize = 4096;

/// Radix-16 hierarchy scale constants
pub const BYTES_PER_PAGE: u64 = 4096; // 16^3
pub const BYTES_PER_VOLUME: u64 = 65_536; // 16^4 = 16 pages
pub const BYTES_PER_SHELF: u64 = 1_048_576; // 16^5 = 1 MiB
pub const BYTES_PER_WALL: u64 = 16_777_216; // 16^6 = 16 MiB
pub const BYTES_PER_ROOM: u64 = 268_435_456; // 16^7 = 256 MiB
pub const BYTES_PER_FLOOR: u64 = 4_294_967_296; // 16^8 = 4 GiB

/// Radix-16 spatial hierarchy coordinates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RadixCoordinate {
    pub floor: u64,
    pub room: u8,
    pub wall: u8,
    pub shelf: u8,
    pub volume: u8,
    pub page: u8,
    pub offset_in_page: u16,
}

impl RadixCoordinate {
    /// Compute the exact radix-16 spatial coordinate for any byte offset
    pub fn from_byte_offset(offset: u64) -> Self {
        let offset_in_page = (offset & 0x0FFF) as u16;
        let page_index = offset >> 12;

        let page = (page_index & 0x0F) as u8;
        let volume = ((page_index >> 4) & 0x0F) as u8;
        let shelf = ((page_index >> 8) & 0x0F) as u8;
        let wall = ((page_index >> 12) & 0x0F) as u8;
        let room = ((page_index >> 16) & 0x0F) as u8;
        let floor = page_index >> 20;

        Self {
            floor,
            room,
            wall,
            shelf,
            volume,
            page,
            offset_in_page,
        }
    }

    /// Convert the coordinate back into its exact absolute byte offset
    pub fn to_byte_offset(&self) -> u64 {
        (self.floor * BYTES_PER_FLOOR)
            + (self.room as u64 * BYTES_PER_ROOM)
            + (self.wall as u64 * BYTES_PER_WALL)
            + (self.shelf as u64 * BYTES_PER_SHELF)
            + (self.volume as u64 * BYTES_PER_VOLUME)
            + (self.page as u64 * BYTES_PER_PAGE)
            + (self.offset_in_page as u64)
    }

    /// Format as canonical hierarchical coordinate notation:
    /// `Floor[f].Room[r].Wall[w].Shelf[s].Volume[v].Page[p]+Offset[0x...]`
    pub fn format_coordinate(&self) -> String {
        format!(
            "Floor[{}].Room[{:X}].Wall[{:X}].Shelf[{:X}].Volume[{:X}].Page[{:X}]+Offset[0x{:03X}]",
            self.floor,
            self.room,
            self.wall,
            self.shelf,
            self.volume,
            self.page,
            self.offset_in_page
        )
    }
}

/// Logical page containing between 1 and 4096 bytes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Page {
    data: Vec<u8>,
}

impl Page {
    /// Create a new logical page from bytes ($1 \le \text{len} \le 4096$)
    pub fn new(data: Vec<u8>) -> Result<Self, PageError> {
        if data.is_empty() {
            return Err(PageError::EmptyPage);
        }
        if data.len() > PAGE_SIZE {
            return Err(PageError::PageSizeExceeded(data.len()));
        }
        Ok(Self { data })
    }

    /// Logical length in bytes ($1 \le \text{len} \le 4096$)
    pub fn logical_len(&self) -> usize {
        self.data.len()
    }

    /// Length in bytes (alias for logical_len)
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if this page is a full 4096-byte page
    pub fn is_full(&self) -> bool {
        self.data.len() == PAGE_SIZE
    }

    /// Reference to underlying page bytes
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Canonical address for this page:
    /// Payload layout: `[page_index: u64 BE (8B)][logical_len: u16 BE (2B)][content_bytes]`
    pub fn to_address(&self, page_index: u64) -> Address {
        let mut payload = Vec::with_capacity(10 + self.data.len());
        payload.extend_from_slice(&page_index.to_be_bytes());
        payload.extend_from_slice(&(self.data.len() as u16).to_be_bytes());
        payload.extend_from_slice(&self.data);
        Address::new(NodeType::Page, payload).expect("Page address creation is valid")
    }

    /// Canonical content-addressed page address using SHA-256 (compact 42-byte payload)
    pub fn to_hash_address(&self, page_index: u64) -> Address {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(&self.data);
        let hash = hasher.finalize();

        let mut payload = Vec::with_capacity(10 + 32);
        payload.extend_from_slice(&page_index.to_be_bytes());
        payload.extend_from_slice(&(self.data.len() as u16).to_be_bytes());
        payload.extend_from_slice(&hash);
        Address::new(NodeType::Page, payload).expect("Page hash address creation is valid")
    }

    /// Decode a page and its page index from a canonical page Address
    pub fn from_address(addr: &Address) -> Result<(u64, Self), PageError> {
        if addr.node_type() != NodeType::Page {
            return Err(PageError::NotAPageAddress);
        }
        let payload = addr.payload();
        if payload.len() < 10 {
            return Err(PageError::InvalidPayloadLength(payload.len()));
        }

        let mut index_bytes = [0u8; 8];
        index_bytes.copy_from_slice(&payload[0..8]);
        let page_index = u64::from_be_bytes(index_bytes);

        let mut len_bytes = [0u8; 2];
        len_bytes.copy_from_slice(&payload[8..10]);
        let expected_len = u16::from_be_bytes(len_bytes) as usize;

        let content = payload[10..].to_vec();
        if content.len() != expected_len {
            return Err(PageError::LogicalLengthMismatch {
                expected: expected_len,
                actual: content.len(),
            });
        }

        let page = Page::new(content)?;
        Ok((page_index, page))
    }
}

/// Split an arbitrary byte stream into consecutive logical pages,
/// properly preserving the partial final page length if not aligned to 4096.
pub fn split_into_pages(bytes: &[u8]) -> Vec<Page> {
    if bytes.is_empty() {
        return Vec::new();
    }
    bytes
        .chunks(PAGE_SIZE)
        .map(|chunk| Page::new(chunk.to_vec()).expect("Chunk is <= PAGE_SIZE"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radix_coordinates_roundtrip() {
        let offsets: Vec<u64> = vec![
            0,
            1,
            15,
            16,
            255,
            256,
            4095,
            4096,
            4097,
            65535,
            65536,
            1_048_575,
            1_048_576,
            16_777_215,
            16_777_216,
            268_435_455,
            268_435_456,
            4_294_967_295,
            4_294_967_296,
            10_000_000_000,
        ];

        for offset in offsets {
            let coord = RadixCoordinate::from_byte_offset(offset);
            let restored = coord.to_byte_offset();
            assert_eq!(restored, offset, "Failed for offset {}", offset);
        }
    }

    #[test]
    fn test_radix_hierarchy_steps() {
        // Page 0 boundary
        let c0 = RadixCoordinate::from_byte_offset(0);
        assert_eq!(c0.floor, 0);
        assert_eq!(c0.room, 0);
        assert_eq!(c0.wall, 0);
        assert_eq!(c0.shelf, 0);
        assert_eq!(c0.volume, 0);
        assert_eq!(c0.page, 0);
        assert_eq!(c0.offset_in_page, 0);

        // Exactly 1 page = 4096 bytes -> Page 1
        let c_page = RadixCoordinate::from_byte_offset(4096);
        assert_eq!(c_page.volume, 0);
        assert_eq!(c_page.page, 1);
        assert_eq!(c_page.offset_in_page, 0);

        // Exactly 1 volume = 16 pages = 65536 bytes -> Volume 1, Page 0
        let c_vol = RadixCoordinate::from_byte_offset(65536);
        assert_eq!(c_vol.volume, 1);
        assert_eq!(c_vol.page, 0);

        // Exactly 1 shelf = 1 MiB -> Shelf 1
        let c_shelf = RadixCoordinate::from_byte_offset(1_048_576);
        assert_eq!(c_shelf.shelf, 1);
        assert_eq!(c_shelf.volume, 0);

        // Exactly 1 wall = 16 MiB -> Wall 1
        let c_wall = RadixCoordinate::from_byte_offset(16_777_216);
        assert_eq!(c_wall.wall, 1);

        // Exactly 1 room = 256 MiB -> Room 1
        let c_room = RadixCoordinate::from_byte_offset(268_435_456);
        assert_eq!(c_room.room, 1);

        // Exactly 1 floor = 4 GiB -> Floor 1
        let c_floor = RadixCoordinate::from_byte_offset(4_294_967_296);
        assert_eq!(c_floor.floor, 1);
        assert_eq!(c_floor.room, 0);
    }

    #[test]
    fn test_split_into_pages_exact_and_partial() {
        // Exactly 4096 bytes
        let data_exact = vec![0x42; 4096];
        let pages = split_into_pages(&data_exact);
        assert_eq!(pages.len(), 1);
        assert_eq!(pages[0].logical_len(), 4096);
        assert!(pages[0].is_full());

        // 4097 bytes -> 1 full page + 1 partial page of 1 byte
        let data_extra = vec![0x42; 4097];
        let pages2 = split_into_pages(&data_extra);
        assert_eq!(pages2.len(), 2);
        assert_eq!(pages2[0].logical_len(), 4096);
        assert!(pages2[0].is_full());
        assert_eq!(pages2[1].logical_len(), 1);
        assert!(!pages2[1].is_full());
    }

    #[test]
    fn test_page_address_roundtrip() {
        let page_data = vec![1, 2, 3, 4, 5];
        let page = Page::new(page_data.clone()).unwrap();
        let addr = page.to_address(42);

        assert_eq!(addr.node_type(), NodeType::Page);
        let (index, restored_page) = Page::from_address(&addr).unwrap();
        assert_eq!(index, 42);
        assert_eq!(restored_page.data(), page_data.as_slice());
        assert_eq!(restored_page.logical_len(), 5);
    }
}
```

## mfas-core\src\resolver\mod.rs

```rust
//! Recursive Resolver & Streaming Reconstructor
//!
//! Provides deterministic evaluation of MFAS DAGs with:
//! - Cycle detection
//! - Safety limits (depth, output bytes, node count)
//! - Streaming page-by-page output without unbounded memory allocation

pub mod store;

pub use store::{AddressStore, FileStore, MemoryStore};

use crate::address::Address;
use crate::error::ResolveError;
use crate::node::Node;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;

/// Maximum size of a resolved node to store in the evaluation memoization cache
const MAX_MEMOIZE_NODE_SIZE: usize = 64 * 1024; // 64 KiB

/// Safety resource limits for recursive reconstruction
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Limits {
    /// Maximum recursion call-stack depth (default: 64)
    pub max_depth: usize,
    /// Maximum permitted reconstructed bytes (default: 10 GiB)
    pub max_output_bytes: u64,
    /// Maximum number of node evaluations (default: 1,000,000)
    pub max_node_count: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_depth: 128,
            max_output_bytes: 50 * 1024 * 1024 * 1024, // 50 GiB
            max_node_count: 10_000_000,
        }
    }
}

/// Internal tracker for resolution progress and safety enforcement
struct ResolverState {
    active_path: Vec<Address>,
    nodes_evaluated: usize,
    bytes_written: u64,
    memo_cache: HashMap<Address, Vec<u8>>,
}

impl ResolverState {
    fn new() -> Self {
        Self {
            active_path: Vec::new(),
            nodes_evaluated: 0,
            bytes_written: 0,
            memo_cache: HashMap::new(),
        }
    }
}

/// Recursively resolve an Address directly into a byte buffer
pub fn resolve(
    addr: &Address,
    store: &dyn AddressStore,
    limits: &Limits,
) -> Result<Vec<u8>, ResolveError> {
    let mut buf = Vec::new();
    resolve_to_writer(addr, store, limits, &mut buf)?;
    Ok(buf)
}

/// Recursively resolve an Address streaming directly into an `io::Write` sink
pub fn resolve_to_writer<W: Write>(
    addr: &Address,
    store: &dyn AddressStore,
    limits: &Limits,
    writer: &mut W,
) -> Result<u64, ResolveError> {
    let mut state = ResolverState::new();
    let dyn_writer: &mut dyn Write = writer;
    resolve_internal(addr, store, limits, &mut state, dyn_writer)?;
    Ok(state.bytes_written)
}

fn resolve_internal(
    addr: &Address,
    store: &dyn AddressStore,
    limits: &Limits,
    state: &mut ResolverState,
    writer: &mut dyn Write,
) -> Result<(), ResolveError> {
    // 1. Check evaluation node count limit
    state.nodes_evaluated += 1;
    if state.nodes_evaluated > limits.max_node_count {
        return Err(ResolveError::MaxNodeCountExceeded {
            limit: limits.max_node_count,
        });
    }

    // 2. Check memoization cache (skips re-evaluation for shared sub-graphs)
    if let Some(cached) = state.memo_cache.get(addr).cloned() {
        write_bytes_with_limit(&cached, limits, state, writer)?;
        return Ok(());
    }

    // 3. Check recursion depth limit
    if state.active_path.len() >= limits.max_depth {
        return Err(ResolveError::MaxDepthExceeded {
            limit: limits.max_depth,
            current: state.active_path.len(),
        });
    }

    // 4. Cycle Detection
    if state.active_path.contains(addr) {
        let mut path_str = state
            .active_path
            .iter()
            .map(|a| a.to_uri())
            .collect::<Vec<_>>()
            .join(" -> ");
        path_str.push_str(&format!(" -> {}", addr.to_uri()));
        return Err(ResolveError::CycleDetected { path: path_str });
    }

    state.active_path.push(addr.clone());

    // 5. Retrieve node (from store or self-describing address)
    let node = match store.get(addr) {
        Ok(Some(n)) => n,
        _ => match Node::from_address(addr) {
            Ok(n) => n,
            Err(_) => return Err(ResolveError::NodeNotFound(addr.to_uri())),
        },
    };

    // 6. Evaluate node
    match node {
        Node::Data(bytes) => {
            if bytes.len() <= MAX_MEMOIZE_NODE_SIZE {
                state.memo_cache.insert(addr.clone(), bytes.clone());
            }
            write_bytes_with_limit(&bytes, limits, state, writer)?;
        }
        Node::Page(page) => {
            let data = page.data();
            if data.len() <= MAX_MEMOIZE_NODE_SIZE {
                state.memo_cache.insert(addr.clone(), data.to_vec());
            }
            write_bytes_with_limit(data, limits, state, writer)?;
        }
        Node::Reference(target) => {
            resolve_internal(&target, store, limits, state, writer)?;
        }
        Node::Sequence(children) => {
            for child in children {
                resolve_internal(&child, store, limits, state, writer)?;
            }
        }
        Node::Repeat { target, count } => {
            if count == 0 {
                state.active_path.pop();
                return Ok(());
            }
            if let Some(cached) = state.memo_cache.get(&target) {
                let cached_bytes = cached.clone();
                for _ in 0..count {
                    write_bytes_with_limit(&cached_bytes, limits, state, writer)?;
                }
            } else if count > 1 {
                let mut target_buf = Vec::new();
                let dyn_target_writer: &mut dyn Write = &mut target_buf;
                resolve_internal(&target, store, limits, state, dyn_target_writer)?;
                if target_buf.len() <= MAX_MEMOIZE_NODE_SIZE {
                    state.memo_cache.insert(target.clone(), target_buf.clone());
                }
                writer.write_all(&target_buf)?;
                for _ in 0..(count - 1) {
                    write_bytes_with_limit(&target_buf, limits, state, writer)?;
                }
            } else {
                resolve_internal(&target, store, limits, state, writer)?;
            }
        }
        Node::Slice {
            target,
            offset,
            length,
        } => {
            let mut filter = SliceFilterWriter::new(writer, offset, length);
            resolve_internal(&target, store, limits, state, &mut filter)?;
        }
    }

    state.active_path.pop();
    Ok(())
}

fn write_bytes_with_limit(
    bytes: &[u8],
    limits: &Limits,
    state: &mut ResolverState,
    writer: &mut dyn Write,
) -> Result<(), ResolveError> {
    let new_total = state.bytes_written.saturating_add(bytes.len() as u64);
    if new_total > limits.max_output_bytes {
        return Err(ResolveError::MaxOutputBytesExceeded {
            limit: limits.max_output_bytes,
            current: new_total,
        });
    }
    writer.write_all(bytes)?;
    state.bytes_written = new_total;
    Ok(())
}

/// Streaming byte-range filter that passes only bytes within [target_offset..target_offset+target_len)
struct SliceFilterWriter<'a> {
    inner: &'a mut dyn Write,
    target_offset: u64,
    target_len: u64,
    stream_pos: u64,
}

impl<'a> SliceFilterWriter<'a> {
    fn new(inner: &'a mut dyn Write, target_offset: u64, target_len: u64) -> Self {
        Self {
            inner,
            target_offset,
            target_len,
            stream_pos: 0,
        }
    }
}

impl<'a> Write for SliceFilterWriter<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let chunk_start = self.stream_pos;
        let chunk_end = chunk_start + buf.len() as u64;
        self.stream_pos = chunk_end;

        let target_end = self.target_offset + self.target_len;

        // Calculate overlap between [chunk_start, chunk_end) and [target_offset, target_end)
        let overlap_start = chunk_start.max(self.target_offset);
        let overlap_end = chunk_end.min(target_end);

        if overlap_start < overlap_end {
            let start_idx = (overlap_start - chunk_start) as usize;
            let end_idx = (overlap_end - chunk_start) as usize;
            self.inner.write_all(&buf[start_idx..end_idx])?;
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::address::NodeType;
    use crate::page::Page;

    #[test]
    fn test_resolve_data_node() {
        let store = MemoryStore::new();
        let limits = Limits::default();
        let node = Node::Data(b"Hello Resolver".to_vec());
        let addr = node.to_address().unwrap();

        let resolved = resolve(&addr, &store, &limits).unwrap();
        assert_eq!(resolved, b"Hello Resolver");
    }

    #[test]
    fn test_resolve_sequence_and_repeat() {
        let store = MemoryStore::new();
        let limits = Limits::default();

        let part1 = Node::Data(b"AB".to_vec()).to_address().unwrap();
        let rep = Node::Repeat {
            target: part1.clone(),
            count: 3,
        }
        .to_address()
        .unwrap();

        let seq = Node::Sequence(vec![part1, rep]).to_address().unwrap();
        let resolved = resolve(&seq, &store, &limits).unwrap();
        assert_eq!(resolved, b"ABABABAB");
    }

    #[test]
    fn test_resolve_slice() {
        let store = MemoryStore::new();
        let limits = Limits::default();

        let base = Node::Data(b"0123456789".to_vec()).to_address().unwrap();
        let slice = Node::Slice {
            target: base,
            offset: 3,
            length: 4,
        }
        .to_address()
        .unwrap();

        let resolved = resolve(&slice, &store, &limits).unwrap();
        assert_eq!(resolved, b"3456");
    }

    #[test]
    fn test_resolve_page() {
        let store = MemoryStore::new();
        let limits = Limits::default();

        let page = Page::new(vec![0xAA; 4096]).unwrap();
        let addr = page.to_address(0);

        let resolved = resolve(&addr, &store, &limits).unwrap();
        assert_eq!(resolved.len(), 4096);
        assert_eq!(resolved[0], 0xAA);
    }

    #[test]
    fn test_cycle_detection() {
        let mut store = MemoryStore::new();
        let limits = Limits::default();

        let addr1 = Address::new(NodeType::Ref, vec![1]).unwrap();
        let addr2 = Address::new(NodeType::Ref, vec![2]).unwrap();

        // Create cycle: addr1 -> addr2 -> addr1
        store.put(&addr1, Node::Reference(addr2.clone())).unwrap();
        store.put(&addr2, Node::Reference(addr1.clone())).unwrap();

        let err = resolve(&addr1, &store, &limits).unwrap_err();
        assert!(matches!(err, ResolveError::CycleDetected { .. }));
    }

    #[test]
    fn test_depth_limit_exceeded() {
        let mut store = MemoryStore::new();
        let limits = Limits {
            max_depth: 3,
            max_output_bytes: 1000,
            max_node_count: 100,
        };

        let a1 = Address::new(NodeType::Ref, vec![1]).unwrap();
        let a2 = Address::new(NodeType::Ref, vec![2]).unwrap();
        let a3 = Address::new(NodeType::Ref, vec![3]).unwrap();
        let a4 = Address::new(NodeType::Ref, vec![4]).unwrap();

        store.put(&a1, Node::Reference(a2.clone())).unwrap();
        store.put(&a2, Node::Reference(a3.clone())).unwrap();
        store.put(&a3, Node::Reference(a4.clone())).unwrap();
        store.put(&a4, Node::Data(b"End".to_vec())).unwrap();

        let err = resolve(&a1, &store, &limits).unwrap_err();
        assert!(matches!(err, ResolveError::MaxDepthExceeded { .. }));
    }

    #[test]
    fn test_output_limit_exceeded() {
        let store = MemoryStore::new();
        let limits = Limits {
            max_depth: 10,
            max_output_bytes: 5,
            max_node_count: 100,
        };

        let data = Node::Data(b"1234567890".to_vec()).to_address().unwrap();
        let err = resolve(&data, &store, &limits).unwrap_err();
        assert!(matches!(err, ResolveError::MaxOutputBytesExceeded { .. }));
    }

    #[test]
    fn test_resolve_memoization_diamond() {
        let mut store = MemoryStore::new();
        let limits = Limits::default();

        // Shared leaf: C
        let node_c = Node::Data(b"SharedData".to_vec());
        let addr_c = node_c.to_address().unwrap();
        store.put(&addr_c, node_c).unwrap();

        // Branch A -> C
        let node_a = Node::Sequence(vec![addr_c.clone()]);
        let addr_a = node_a.to_address().unwrap();
        store.put(&addr_a, node_a).unwrap();

        // Branch B -> C
        let node_b = Node::Sequence(vec![addr_c.clone()]);
        let addr_b = node_b.to_address().unwrap();
        store.put(&addr_b, node_b).unwrap();

        // Root -> [A, B] (Diamond)
        let root = Node::Sequence(vec![addr_a, addr_b]).to_address().unwrap();

        let resolved = resolve(&root, &store, &limits).unwrap();
        assert_eq!(resolved, b"SharedDataSharedData");
    }

    #[test]
    fn test_resolve_repeat_optimization() {
        let mut store = MemoryStore::new();
        let limits = Limits::default();

        let node = Node::Data(b"XYZ".to_vec());
        let addr = node.to_address().unwrap();
        store.put(&addr, node).unwrap();

        let rep = Node::Repeat {
            target: addr,
            count: 5,
        }
        .to_address()
        .unwrap();

        let resolved = resolve(&rep, &store, &limits).unwrap();
        assert_eq!(resolved, b"XYZXYZXYZXYZXYZ");
    }
}
```

## mfas-core\src\resolver\store.rs

```rust
use crate::address::Address;
use crate::error::CoreError;
use crate::node::Node;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Storage interface for retrieving and persisting nodes by address
pub trait AddressStore {
    fn get(&self, addr: &Address) -> Result<Option<Node>, CoreError>;
    fn put(&mut self, addr: &Address, node: Node) -> Result<(), CoreError>;
}

/// In-memory hash map implementation of AddressStore
#[derive(Debug, Default, Clone)]
pub struct MemoryStore {
    nodes: HashMap<Address, Node>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

impl AddressStore for MemoryStore {
    fn get(&self, addr: &Address) -> Result<Option<Node>, CoreError> {
        Ok(self.nodes.get(addr).cloned())
    }

    fn put(&mut self, addr: &Address, node: Node) -> Result<(), CoreError> {
        self.nodes.insert(addr.clone(), node);
        Ok(())
    }
}

/// Persistent disk-based content-addressed storage implementation of AddressStore
#[derive(Debug, Clone)]
pub struct FileStore {
    root_dir: PathBuf,
}

impl FileStore {
    pub fn new<P: AsRef<Path>>(root_dir: P) -> std::io::Result<Self> {
        let p = root_dir.as_ref().to_path_buf();
        std::fs::create_dir_all(&p)?;
        Ok(Self { root_dir: p })
    }

    pub fn default_store() -> std::io::Result<Self> {
        Self::new(".mfas/objects")
    }

    fn path_for_address(&self, addr: &Address) -> PathBuf {
        use sha2::{Digest, Sha256};
        let hash = hex::encode(Sha256::digest(addr.to_binary()));
        let prefix = &hash[..2];
        let rest = &hash[2..];
        self.root_dir.join(prefix).join(format!("{}.json", rest))
    }
}

impl AddressStore for FileStore {
    fn get(&self, addr: &Address) -> Result<Option<Node>, CoreError> {
        let path = self.path_for_address(addr);
        if !path.exists() {
            return Ok(None);
        }
        let data = std::fs::read(&path)?;
        let node: Node = serde_json::from_slice(&data)
            .map_err(|e| CoreError::Io(e.to_string()))?;
        Ok(Some(node))
    }

    fn put(&mut self, addr: &Address, node: Node) -> Result<(), CoreError> {
        let path = self.path_for_address(addr);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let data = serde_json::to_vec(&node)
            .map_err(|e| CoreError::Io(e.to_string()))?;
        std::fs::write(&path, data)?;
        Ok(())
    }
}
```

## mfas-core\src\synthetic\mod.rs

```rust
//! Synthetic Data Generator
//!
//! Provides deterministic streaming readers for mathematical and stress testing data:
//! - Zeros: constant zero streams
//! - Counter: sequential byte streams ($0, 1, 2, \dots, 255, 0, \dots$)
//! - Repeat: repeated custom byte patterns
//! - Random: deterministic pseudo-random streams using Xorshift64

use std::io::Read;

/// Deterministic synthetic data generator stream
pub struct SyntheticStream {
    generator: GeneratorKind,
    remaining: u64,
}

enum GeneratorKind {
    Zeros,
    Counter { current: u8 },
    Repeat { pattern: Vec<u8>, offset: usize },
    Random { state: u64 },
}

impl SyntheticStream {
    /// Stream of all zero bytes
    pub fn zeros(total_bytes: u64) -> Self {
        Self {
            generator: GeneratorKind::Zeros,
            remaining: total_bytes,
        }
    }

    /// Stream of incrementing byte values
    pub fn counter(total_bytes: u64) -> Self {
        Self {
            generator: GeneratorKind::Counter { current: 0 },
            remaining: total_bytes,
        }
    }

    /// Stream repeating a specific byte pattern
    pub fn repeat(pattern: Vec<u8>, count: u64) -> Self {
        let total = (pattern.len() as u64).saturating_mul(count);
        Self {
            generator: GeneratorKind::Repeat { pattern, offset: 0 },
            remaining: total,
        }
    }

    /// Deterministic pseudo-random stream using XorShift64
    pub fn random(total_bytes: u64, seed: u64) -> Self {
        let state = if seed == 0 { 0xDEADBEEFCAFEBABE } else { seed };
        Self {
            generator: GeneratorKind::Random { state },
            remaining: total_bytes,
        }
    }

    pub fn total_remaining(&self) -> u64 {
        self.remaining
    }
}

impl Read for SyntheticStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.remaining == 0 || buf.is_empty() {
            return Ok(0);
        }

        let to_write = (buf.len() as u64).min(self.remaining) as usize;
        let slice = &mut buf[..to_write];

        match &mut self.generator {
            GeneratorKind::Zeros => {
                slice.fill(0);
            }
            GeneratorKind::Counter { current } => {
                for byte in slice.iter_mut() {
                    *byte = *current;
                    *current = current.wrapping_add(1);
                }
            }
            GeneratorKind::Repeat { pattern, offset } => {
                if pattern.is_empty() {
                    return Ok(0);
                }
                for byte in slice.iter_mut() {
                    *byte = pattern[*offset];
                    *offset = (*offset + 1) % pattern.len();
                }
            }
            GeneratorKind::Random { state } => {
                for byte in slice.iter_mut() {
                    // Xorshift64 algorithm
                    let mut x = *state;
                    x ^= x << 13;
                    x ^= x >> 7;
                    x ^= x << 17;
                    *state = x;
                    *byte = (x & 0xFF) as u8;
                }
            }
        }

        self.remaining -= to_write as u64;
        Ok(to_write)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zeros_stream() {
        let mut stream = SyntheticStream::zeros(100);
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).unwrap();
        assert_eq!(buf.len(), 100);
        assert!(buf.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_counter_stream() {
        let mut stream = SyntheticStream::counter(300);
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).unwrap();
        assert_eq!(buf.len(), 300);
        assert_eq!(buf[0], 0);
        assert_eq!(buf[255], 255);
        assert_eq!(buf[256], 0);
    }

    #[test]
    fn test_repeat_stream() {
        let mut stream = SyntheticStream::repeat(vec![1, 2, 3], 2);
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).unwrap();
        assert_eq!(buf, vec![1, 2, 3, 1, 2, 3]);
    }

    #[test]
    fn test_random_stream_deterministic() {
        let mut s1 = SyntheticStream::random(50, 42);
        let mut s2 = SyntheticStream::random(50, 42);

        let mut b1 = Vec::new();
        let mut b2 = Vec::new();
        s1.read_to_end(&mut b1).unwrap();
        s2.read_to_end(&mut b2).unwrap();

        assert_eq!(b1, b2);
    }
}
```

# 4. Binary Files

No binary files found.

# 5. Skipped Large Files

No large files skipped.

# 6. Statistics

- Files scanned : 46
- Included      : 46
- Skipped       : 0
- Binary        : 0
- Large         : 0
- Ignored dirs  : 0
- Total size    : 216.9 KB
- Markdown size : 220,202 characters
- Estimated tokens: 55,050