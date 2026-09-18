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
