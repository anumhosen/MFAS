use clap::{Args, Subcommand};
use mfas_core::{
    decode_to_file, encode, hash_file, Address, FileStore, SyntheticStream,
};
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct GenerateArgs {
    #[command(subcommand)]
    pub action: GenerateAction,
}

#[derive(Subcommand, Debug)]
pub enum GenerateAction {
    /// Reconstruct an existing canonical address directly into an output file
    Address {
        /// Canonical MFAS address URI
        address: String,

        /// Destination output file path
        output: PathBuf,
    },
    /// Generate deterministic synthetic test data and encode it into MFAS
    Synthetic {
        /// Synthetic pattern: 'zeros', 'counter', 'repeat', 'random'
        #[arg(short, long, default_value = "zeros")]
        pattern: String,

        /// Total size in bytes to generate
        #[arg(short, long, default_value_t = 1048576)]
        size: u64,

        /// Pattern hex string (used when pattern == 'repeat')
        #[arg(long, default_value = "414243")]
        repeat_hex: String,

        /// Deterministic random seed (used when pattern == 'random')
        #[arg(long, default_value_t = 42)]
        seed: u64,

        /// Also write the raw synthetic bytes to this file path
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

#[derive(Serialize)]
struct SyntheticOutput {
    pattern: String,
    size_bytes: u64,
    root_address: String,
    sha256: Option<String>,
    output_file: Option<String>,
}

pub fn handle_generate_cmd(
    args: GenerateArgs,
    json: bool,
    quiet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match args.action {
        GenerateAction::Address { address, output } => {
            let addr: Address = address.parse()?;
            let store = FileStore::default_store()?;
            let bytes_written = decode_to_file(&addr, &store, &output)?;

            if json {
                let out = serde_json::json!({
                    "address": addr.to_uri(),
                    "output_file": output.display().to_string(),
                    "bytes_written": bytes_written,
                });
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else if quiet {
                println!("{}", output.display());
            } else {
                println!("MFAS Generator (Address Rebuild)");
                println!("──────────────────────────────────────────");
                println!("Address:       {}", addr.to_uri());
                println!("Output:        {}", output.display());
                println!("Bytes Written: {} bytes", bytes_written);
            }
        }
        GenerateAction::Synthetic {
            pattern,
            size,
            repeat_hex,
            seed,
            output,
        } => {
            let mut stream = match pattern.as_str() {
                "zeros" => SyntheticStream::zeros(size),
                "counter" => SyntheticStream::counter(size),
                "repeat" => {
                    let pat_bytes = hex::decode(repeat_hex.trim())?;
                    let count = (size / pat_bytes.len().max(1) as u64).max(1);
                    SyntheticStream::repeat(pat_bytes, count)
                }
                "random" => SyntheticStream::random(size, seed),
                _ => {
                    return Err(format!(
                        "Unknown synthetic pattern '{}'. Valid: zeros, counter, repeat, random",
                        pattern
                    )
                    .into())
                }
            };

            let mut store = FileStore::default_store()?;

            let (root_addr, file_out, sha256_out) = if let Some(out_path) = output {
                // Tee into output file and encode
                let file = File::create(&out_path)?;
                let mut writer = BufWriter::new(file);
                let mut buf = vec![0u8; 8192];
                loop {
                    let n = stream.read(&mut buf)?;
                    if n == 0 {
                        break;
                    }
                    writer.write_all(&buf[..n])?;
                }
                writer.flush()?;

                // Now encode from generated file
                let mut read_back = File::open(&out_path)?;
                let addr = encode(&mut read_back, &mut store)?;
                let (hash, _) = hash_file(&out_path)?;
                (
                    addr,
                    Some(out_path.display().to_string()),
                    Some(hash),
                )
            } else {
                let addr = encode(&mut stream, &mut store)?;
                (addr, None, None)
            };

            if json {
                let out = SyntheticOutput {
                    pattern,
                    size_bytes: size,
                    root_address: root_addr.to_uri(),
                    sha256: sha256_out,
                    output_file: file_out,
                };
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else if quiet {
                println!("{}", root_addr.to_uri());
            } else {
                println!("MFAS Synthetic Data Generator");
                println!("──────────────────────────────────────────");
                println!("Pattern:       {}", pattern);
                println!("Size:          {} bytes", size);
                if let Some(h) = sha256_out {
                    println!("SHA-256:       {}", h);
                }
                if let Some(f) = file_out {
                    println!("Saved to:      {}", f);
                }
                println!("Root Address:  {}", root_addr.to_uri());
            }
        }
    }
    Ok(())
}
