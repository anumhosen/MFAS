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
