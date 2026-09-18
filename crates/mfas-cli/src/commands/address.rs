use clap::{Args, Subcommand};
use mfas_core::{Address, NodeType};
use serde::Serialize;
use std::str::FromStr;

#[derive(Args, Debug)]
pub struct AddressArgs {
    #[command(subcommand)]
    pub action: AddressAction,
}

#[derive(Subcommand, Debug)]
pub enum AddressAction {
    /// Parse and inspect an MFAS address URI
    Parse {
        /// Canonical URI string (e.g. mfas:v1:data:48656c6c6f)
        uri: String,
    },
    /// Format components into a canonical MFAS address URI
    Format {
        /// Node type (data, ref, seq, rep, slice, page)
        #[arg(short = 't', long)]
        node_type: String,
        /// Hexadecimal payload
        #[arg(short = 'p', long)]
        payload: String,
    },
    /// Validate an MFAS address URI syntax and payload
    Validate {
        /// URI to validate
        uri: String,
    },
    /// Convert a canonical address URI into its binary format (hex-encoded)
    ToBinary {
        /// Canonical URI string
        uri: String,
    },
    /// Decode binary address bytes (hex-encoded) back into a canonical URI
    FromBinary {
        /// Hexadecimal binary address representation
        hex: String,
    },
}

#[derive(Serialize)]
struct AddressOutput<'a> {
    uri: &'a str,
    version: u8,
    node_type: &'a str,
    payload_hex: String,
    payload_len_bytes: usize,
}

#[derive(Serialize)]
struct ValidationOutput<'a> {
    valid: bool,
    uri: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Serialize)]
struct BinaryOutput<'a> {
    uri: &'a str,
    binary_hex: String,
    binary_len_bytes: usize,
}

pub fn handle_address_cmd(
    args: AddressArgs,
    json: bool,
    quiet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match args.action {
        AddressAction::Parse { uri } => {
            let addr: Address = uri.parse()?;
            if json {
                let out = AddressOutput {
                    uri: &addr.to_uri(),
                    version: addr.version(),
                    node_type: addr.node_type().as_str(),
                    payload_hex: addr.payload_hex(),
                    payload_len_bytes: addr.payload().len(),
                };
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else if quiet {
                println!("{}", addr.to_uri());
            } else {
                println!("MFAS Canonical Address");
                println!("──────────────────────────────────────────");
                println!("URI:          {}", addr.to_uri());
                println!("Version:      v{}", addr.version());
                println!("Node Type:    {}", addr.node_type());
                println!("Payload (B):  {} bytes", addr.payload().len());
                println!("Payload Hex:  {}", addr.payload_hex());
            }
        }
        AddressAction::Format { node_type, payload } => {
            let nt = NodeType::from_str(&node_type)?;
            let addr = Address::from_hex(nt, &payload)?;
            if json {
                let out = AddressOutput {
                    uri: &addr.to_uri(),
                    version: addr.version(),
                    node_type: addr.node_type().as_str(),
                    payload_hex: addr.payload_hex(),
                    payload_len_bytes: addr.payload().len(),
                };
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else {
                println!("{}", addr.to_uri());
            }
        }
        AddressAction::Validate { uri } => match uri.parse::<Address>() {
            Ok(addr) => {
                if json {
                    let out = ValidationOutput {
                        valid: true,
                        uri: &addr.to_uri(),
                        error: None,
                    };
                    println!("{}", serde_json::to_string_pretty(&out)?);
                } else if !quiet {
                    println!("VALID: {}", addr.to_uri());
                }
            }
            Err(e) => {
                if json {
                    let out = ValidationOutput {
                        valid: false,
                        uri: &uri,
                        error: Some(e.to_string()),
                    };
                    println!("{}", serde_json::to_string_pretty(&out)?);
                } else {
                    eprintln!("INVALID: {}", e);
                }
                std::process::exit(1);
            }
        },
        AddressAction::ToBinary { uri } => {
            let addr: Address = uri.parse()?;
            let bin = addr.to_binary();
            let bin_hex = hex::encode(&bin);
            if json {
                let out = BinaryOutput {
                    uri: &addr.to_uri(),
                    binary_hex: bin_hex,
                    binary_len_bytes: bin.len(),
                };
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else if quiet {
                println!("{}", bin_hex);
            } else {
                println!("Binary Length: {} bytes", bin.len());
                println!("Binary Hex:    {}", bin_hex);
            }
        }
        AddressAction::FromBinary { hex } => {
            let bin = hex::decode(hex.trim())?;
            let addr = Address::from_binary(&bin)?;
            if json {
                let out = AddressOutput {
                    uri: &addr.to_uri(),
                    version: addr.version(),
                    node_type: addr.node_type().as_str(),
                    payload_hex: addr.payload_hex(),
                    payload_len_bytes: addr.payload().len(),
                };
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else {
                println!("{}", addr.to_uri());
            }
        }
    }
    Ok(())
}
