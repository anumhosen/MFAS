use clap::Args;
use mfas_core::{decode_to_file, encode, hash_file, verify_file, Address, FileStore};
use serde::Serialize;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct EncodeArgs {
    /// Path to the input file to encode
    pub file: PathBuf,
}

#[derive(Args, Debug)]
pub struct DecodeArgs {
    /// Canonical MFAS address URI to decode
    pub address: String,

    /// Output destination file path
    pub output: PathBuf,
}

#[derive(Args, Debug)]
pub struct VerifyArgs {
    /// Path to the file to verify
    pub file: PathBuf,
}

#[derive(Serialize)]
struct EncodeOutput {
    file: String,
    address: String,
    bytes: u64,
    sha256: String,
}

#[derive(Serialize)]
struct DecodeOutput {
    address: String,
    output_file: String,
    bytes_written: u64,
    sha256: String,
}

pub fn handle_encode_cmd(
    args: EncodeArgs,
    json: bool,
    quiet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut store = FileStore::default_store()?;
    let (sha256, total_bytes) = hash_file(&args.file)?;

    let file = File::open(&args.file)?;
    let mut reader = BufReader::new(file);
    let root_addr = encode(&mut reader, &mut store)?;

    if json {
        let out = EncodeOutput {
            file: args.file.display().to_string(),
            address: root_addr.to_uri(),
            bytes: total_bytes,
            sha256,
        };
        println!("{}", serde_json::to_string_pretty(&out)?);
    } else if quiet {
        println!("{}", root_addr.to_uri());
    } else {
        println!("MFAS Canonical File Encoding");
        println!("──────────────────────────────────────────");
        println!("File:         {}", args.file.display());
        println!("Size:         {} bytes", total_bytes);
        println!("SHA-256:      {}", sha256);
        println!("Root Address: {}", root_addr.to_uri());
    }
    Ok(())
}

pub fn handle_decode_cmd(
    args: DecodeArgs,
    json: bool,
    quiet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr: Address = args.address.parse()?;
    let store = FileStore::default_store()?;

    let bytes_written = decode_to_file(&addr, &store, &args.output)?;
    let (sha256, _) = hash_file(&args.output)?;

    if json {
        let out = DecodeOutput {
            address: addr.to_uri(),
            output_file: args.output.display().to_string(),
            bytes_written,
            sha256,
        };
        println!("{}", serde_json::to_string_pretty(&out)?);
    } else if quiet {
        println!("{}", args.output.display());
    } else {
        println!("MFAS File Reconstruction");
        println!("──────────────────────────────────────────");
        println!("Address:      {}", addr.to_uri());
        println!("Destination:  {}", args.output.display());
        println!("Bytes Written:{} bytes", bytes_written);
        println!("SHA-256:      {}", sha256);
    }
    Ok(())
}

pub fn handle_verify_cmd(
    args: VerifyArgs,
    json: bool,
    quiet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut store = FileStore::default_store()?;
    let report = verify_file(&args.file, &mut store)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else if quiet {
        if report.verified {
            println!("OK");
        } else {
            eprintln!("FAILED");
            std::process::exit(1);
        }
    } else {
        println!("MFAS Integrity Verification");
        println!("──────────────────────────────────────────");
        println!("File:         {}", report.file_path);
        println!("Root Address: {}", report.root_address.to_uri());
        println!("Size:         {} bytes", report.total_bytes);
        println!("SHA-256:      {}", report.sha256);
        if report.verified {
            println!("Result:       VERIFIED (Exact byte-for-byte match)");
        } else {
            println!("Result:       FAILED (Checksum or length mismatch)");
            std::process::exit(1);
        }
    }
    Ok(())
}
