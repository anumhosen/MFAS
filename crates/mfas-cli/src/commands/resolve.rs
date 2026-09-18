use clap::Args;
use mfas_core::{resolve_to_writer, Address, FileStore, Limits};
use serde::Serialize;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct ResolveArgs {
    /// Canonical address URI to recursively resolve
    pub uri: String,

    /// Write output directly to a file
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Display reconstructed bytes as hexadecimal
    #[arg(long)]
    pub hex: bool,

    /// Maximum recursion depth limit
    #[arg(long, default_value_t = 64)]
    pub max_depth: usize,

    /// Maximum permitted output bytes
    #[arg(long, default_value_t = 10 * 1024 * 1024 * 1024)]
    pub max_bytes: u64,

    /// Simulate resolution and output diagnostics without writing bytes
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Serialize)]
struct ResolveOutput {
    uri: String,
    reconstructed_bytes: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    hex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_file: Option<String>,
}

/// Sink that counts bytes written without storing them (used for dry-run)
struct NullWriter {
    count: u64,
}

impl Write for NullWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.count += buf.len() as u64;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub fn handle_resolve_cmd(
    args: ResolveArgs,
    json: bool,
    quiet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr: Address = args.uri.parse()?;
    let store = FileStore::default_store()?;
    let limits = Limits {
        max_depth: args.max_depth,
        max_output_bytes: args.max_bytes,
        max_node_count: 1_000_000,
    };

    if args.dry_run {
        let mut null_sink = NullWriter { count: 0 };
        let bytes_resolved = resolve_to_writer(&addr, &store, &limits, &mut null_sink)?;

        if json {
            let out = serde_json::json!({
                "uri": addr.to_uri(),
                "dry_run": true,
                "projected_bytes": bytes_resolved,
                "status": "valid",
            });
            println!("{}", serde_json::to_string_pretty(&out)?);
        } else if !quiet {
            println!("MFAS Resolve (Dry Run)");
            println!("──────────────────────────────────────────");
            println!("Address:          {}", addr.to_uri());
            println!("Reconstructed:    {} bytes", bytes_resolved);
            println!("Cycle Check:      Passed (No cycles detected)");
            println!("Depth Limits:     Passed");
        }
        return Ok(());
    }

    if let Some(out_path) = args.output {
        let file = File::create(&out_path)?;
        let mut writer = BufWriter::new(file);
        let bytes_resolved = resolve_to_writer(&addr, &store, &limits, &mut writer)?;
        writer.flush()?;

        if json {
            let out = ResolveOutput {
                uri: addr.to_uri(),
                reconstructed_bytes: bytes_resolved,
                hex: None,
                output_file: Some(out_path.display().to_string()),
            };
            println!("{}", serde_json::to_string_pretty(&out)?);
        } else if !quiet {
            println!(
                "Successfully reconstructed {} bytes to {}",
                bytes_resolved,
                out_path.display()
            );
        }
    } else if args.hex {
        let mut buf = Vec::new();
        let bytes_resolved = resolve_to_writer(&addr, &store, &limits, &mut buf)?;
        let hex_str = hex::encode(&buf);

        if json {
            let out = ResolveOutput {
                uri: addr.to_uri(),
                reconstructed_bytes: bytes_resolved,
                hex: Some(hex_str),
                output_file: None,
            };
            println!("{}", serde_json::to_string_pretty(&out)?);
        } else if quiet {
            println!("{}", hex_str);
        } else {
            println!("Reconstructed {} bytes:\n{}", bytes_resolved, hex_str);
        }
    } else {
        let mut buf = Vec::new();
        let bytes_resolved = resolve_to_writer(&addr, &store, &limits, &mut buf)?;

        if json {
            let out = serde_json::json!({
                "uri": addr.to_uri(),
                "reconstructed_bytes": bytes_resolved,
                "text": String::from_utf8(buf.clone()).ok(),
                "hex": hex::encode(&buf),
            });
            println!("{}", serde_json::to_string_pretty(&out)?);
        } else if quiet {
            io::stdout().write_all(&buf)?;
        } else {
            match String::from_utf8(buf.clone()) {
                Ok(s) => println!("{}", s),
                Err(_) => {
                    println!("Reconstructed binary: {} bytes", bytes_resolved);
                    println!("{}", hex::encode(&buf));
                }
            }
        }
    }

    Ok(())
}
