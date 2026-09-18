use crate::commands::parse_address_str;
use mfas_core::codec::{decode_to_file as core_decode_to_file, encode, hash_file, verify_file as core_verify_file};
use mfas_core::resolver::store::FileStore;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncodeResultDto {
    pub file_path: String,
    pub root_address: String,
    pub canonical_hex: String,
    pub file_size: u64,
    pub sha256: String,
    pub elapsed_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodeResultDto {
    pub root_address: String,
    pub output_path: String,
    pub bytes_written: u64,
    pub sha256: String,
    pub elapsed_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyResultDto {
    pub file_path: String,
    pub root_address: String,
    pub total_bytes: u64,
    pub sha256: String,
    pub verified: bool,
    pub elapsed_ms: u64,
}

#[tauri::command]
pub fn encode_file(path: String) -> Result<EncodeResultDto, String> {
    let p = Path::new(&path);
    if !p.exists() {
        return Err(format!("File does not exist: {}", path));
    }

    let start = Instant::now();
    let (sha256, file_size) = hash_file(p).map_err(|e| e.to_string())?;

    let file = File::open(p).map_err(|e| e.to_string())?;
    let mut reader = BufReader::new(file);

    let mut store = FileStore::default_store().map_err(|e| e.to_string())?;
    let root_addr = encode(&mut reader, &mut store).map_err(|e| e.to_string())?;
    let elapsed_ms = start.elapsed().as_millis() as u64;

    Ok(EncodeResultDto {
        file_path: path,
        root_address: root_addr.to_uri(),
        canonical_hex: hex::encode(root_addr.to_binary()),
        file_size,
        sha256,
        elapsed_ms,
    })
}

#[tauri::command]
pub fn decode_file(addr_str: String, out_path: String) -> Result<DecodeResultDto, String> {
    let start = Instant::now();
    let addr = parse_address_str(&addr_str)?;
    let store = FileStore::default_store().map_err(|e| e.to_string())?;

    let out_p = Path::new(&out_path);
    if let Some(parent) = out_p.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let bytes_written = core_decode_to_file(&addr, &store, out_p).map_err(|e| e.to_string())?;
    let (sha256, _) = hash_file(out_p).map_err(|e| e.to_string())?;
    let elapsed_ms = start.elapsed().as_millis() as u64;

    Ok(DecodeResultDto {
        root_address: addr.to_uri(),
        output_path: out_path,
        bytes_written,
        sha256,
        elapsed_ms,
    })
}

#[tauri::command]
pub fn verify_file(path: String) -> Result<VerifyResultDto, String> {
    let p = Path::new(&path);
    if !p.exists() {
        return Err(format!("File does not exist: {}", path));
    }

    let start = Instant::now();
    let mut store = FileStore::default_store().map_err(|e| e.to_string())?;
    let report = core_verify_file(p, &mut store).map_err(|e| e.to_string())?;
    let elapsed_ms = start.elapsed().as_millis() as u64;

    Ok(VerifyResultDto {
        file_path: report.file_path,
        root_address: report.root_address.to_uri(),
        total_bytes: report.total_bytes,
        sha256: report.sha256,
        verified: report.verified,
        elapsed_ms,
    })
}
