use clap::{Args, Subcommand};
use mfas_core::{bytes_to_number, number_to_bytes, Address, NodeType};
use num_bigint::BigUint;
use num_traits::Num;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Args, Debug)]
pub struct NumberArgs {
    #[command(subcommand)]
    pub action: NumberAction,
}

#[derive(Subcommand, Debug)]
pub enum NumberAction {
    /// Map finite bytes to a unique natural number N
    Encode {
        /// Raw text input string (or hex string if --hex is specified)
        #[arg(short, long)]
        input: Option<String>,

        /// Read bytes from a file
        #[arg(short, long)]
        file: Option<PathBuf>,

        /// Treat input argument as a hexadecimal string
        #[arg(long)]
        hex: bool,
    },
    /// Map a natural number N to its unique finite byte sequence
    Decode {
        /// Natural number N in decimal (or hex with '0x' prefix)
        number: String,

        /// Output the decoded bytes as a hexadecimal string
        #[arg(long)]
        hex: bool,

        /// Save decoded bytes to a file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Map an existing MFAS address payload to its natural number N
    FromAddress {
        /// Canonical address URI
        uri: String,
    },
    /// Construct a canonical MFAS address from a natural number N
    ToAddress {
        /// Natural number N
        number: String,

        /// Node type (data, ref, seq, rep, slice, page)
        #[arg(short = 't', long, default_value = "data")]
        node_type: String,
    },
}

#[derive(Serialize)]
struct NumberEncodeOutput {
    bytes_len: usize,
    number_dec: String,
    number_hex: String,
}

#[derive(Serialize)]
struct NumberDecodeOutput {
    number_dec: String,
    bytes_len: usize,
    hex: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
}

#[derive(Serialize)]
struct AddressNumberOutput {
    uri: String,
    node_type: String,
    number_dec: String,
    number_hex: String,
}

pub fn handle_number_cmd(
    args: NumberArgs,
    json: bool,
    quiet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match args.action {
        NumberAction::Encode { input, file, hex } => {
            let bytes = if let Some(path) = file {
                fs::read(path)?
            } else if let Some(text) = input {
                if hex {
                    hex::decode(text.trim())?
                } else {
                    text.into_bytes()
                }
            } else {
                return Err("Either --input or --file must be specified".into());
            };

            let n = bytes_to_number(&bytes);
            let n_dec = n.to_str_radix(10);
            let n_hex = format!("0x{:x}", n);

            if json {
                let out = NumberEncodeOutput {
                    bytes_len: bytes.len(),
                    number_dec: n_dec,
                    number_hex: n_hex,
                };
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else if quiet {
                println!("{}", n_dec);
            } else {
                println!("Natural Number Representation");
                println!("──────────────────────────────────────────");
                println!("Input Bytes:  {} bytes", bytes.len());
                println!("Decimal (N):  {}", n_dec);
                println!("Hex (N):      {}", n_hex);
            }
        }
        NumberAction::Decode {
            number,
            hex: output_hex,
            output,
        } => {
            let n = parse_biguint(&number)?;
            let bytes = number_to_bytes(&n);

            if let Some(out_path) = output {
                fs::write(&out_path, &bytes)?;
                if !quiet && !json {
                    println!("Wrote {} bytes to {}", bytes.len(), out_path.display());
                }
            } else if json {
                let text_repr = String::from_utf8(bytes.clone()).ok();
                let out = NumberDecodeOutput {
                    number_dec: n.to_str_radix(10),
                    bytes_len: bytes.len(),
                    hex: hex::encode(&bytes),
                    text: text_repr,
                };
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else if quiet {
                if output_hex {
                    println!("{}", hex::encode(&bytes));
                } else {
                    match String::from_utf8(bytes.clone()) {
                        Ok(s) => print!("{}", s),
                        Err(_) => println!("{}", hex::encode(&bytes)),
                    }
                }
            } else if output_hex {
                println!("{}", hex::encode(&bytes));
            } else {
                match String::from_utf8(bytes.clone()) {
                    Ok(s) => {
                        println!("Decoded String: \"{}\"", s);
                        println!("Byte Length:    {} bytes", bytes.len());
                        println!("Hex:            {}", hex::encode(&bytes));
                    }
                    Err(_) => {
                        println!("Decoded Binary: {} bytes", bytes.len());
                        println!("Hex:            {}", hex::encode(&bytes));
                    }
                }
            }
        }
        NumberAction::FromAddress { uri } => {
            let addr: Address = uri.parse()?;
            let n = bytes_to_number(addr.payload());
            let n_dec = n.to_str_radix(10);
            let n_hex = format!("0x{:x}", n);

            if json {
                let out = AddressNumberOutput {
                    uri: addr.to_uri(),
                    node_type: addr.node_type().as_str().to_string(),
                    number_dec: n_dec,
                    number_hex: n_hex,
                };
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else if quiet {
                println!("{}", n_dec);
            } else {
                println!("Address -> Number");
                println!("──────────────────────────────────────────");
                println!("URI:          {}", addr.to_uri());
                println!("Node Type:    {}", addr.node_type());
                println!("Decimal (N):  {}", n_dec);
                println!("Hex (N):      {}", n_hex);
            }
        }
        NumberAction::ToAddress { number, node_type } => {
            let n = parse_biguint(&number)?;
            let bytes = number_to_bytes(&n);
            let nt = NodeType::from_str(&node_type)?;
            let addr = Address::new(nt, bytes)?;

            if json {
                let out = AddressNumberOutput {
                    uri: addr.to_uri(),
                    node_type: addr.node_type().as_str().to_string(),
                    number_dec: n.to_str_radix(10),
                    number_hex: format!("0x{:x}", n),
                };
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else {
                println!("{}", addr.to_uri());
            }
        }
    }
    Ok(())
}

fn parse_biguint(s: &str) -> Result<BigUint, Box<dyn std::error::Error>> {
    let s = s.trim();
    if s.starts_with("0x") || s.starts_with("0X") {
        BigUint::from_str_radix(&s[2..], 16).map_err(|e| e.into())
    } else {
        BigUint::from_str_radix(s, 10).map_err(|e| e.into())
    }
}
