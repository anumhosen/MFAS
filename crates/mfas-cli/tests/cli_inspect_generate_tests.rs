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
    assert_eq!(report["page_count"], 3); // 4096 + 4096 + 1808
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
    assert!(stdout.contains("Pages:         3"));
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
