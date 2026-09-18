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
