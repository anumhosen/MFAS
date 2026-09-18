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
