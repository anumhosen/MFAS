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
