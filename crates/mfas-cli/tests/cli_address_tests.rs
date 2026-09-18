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
