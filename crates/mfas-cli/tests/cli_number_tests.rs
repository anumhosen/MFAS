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
