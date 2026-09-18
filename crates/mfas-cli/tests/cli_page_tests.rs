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
