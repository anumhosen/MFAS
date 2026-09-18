use clap::{Args, Subcommand};
use mfas_core::{Address, Node};
use serde::Serialize;

#[derive(Args, Debug)]
pub struct NodeArgs {
    #[command(subcommand)]
    pub action: NodeAction,
}

#[derive(Subcommand, Debug)]
pub enum NodeAction {
    /// Inspect the structured Node represented by a canonical Address URI
    Inspect {
        /// Canonical Address URI
        uri: String,
    },
    /// Create a Data node
    Data {
        /// Literal text (or hex if --hex is specified)
        #[arg(short, long)]
        input: String,

        /// Treat input as hex
        #[arg(long)]
        hex: bool,
    },
    /// Create a Reference node pointing to another address
    Ref {
        /// Target Address URI
        #[arg(short, long)]
        target: String,
    },
    /// Create a Sequence node concatenating multiple child addresses
    Seq {
        /// Child Address URIs (space-separated)
        #[arg(required = true)]
        children: Vec<String>,
    },
    /// Create a Repeat node repeating a child address N times
    Repeat {
        /// Target Address URI
        #[arg(short, long)]
        target: String,

        /// Number of repetitions
        #[arg(short, long)]
        count: u64,
    },
    /// Create a Slice node representing a byte-range subslice of a child address
    Slice {
        /// Target Address URI
        #[arg(short, long)]
        target: String,

        /// Starting byte offset
        #[arg(short, long)]
        offset: u64,

        /// Subslice length in bytes
        #[arg(short, long)]
        length: u64,
    },
}

#[derive(Serialize)]
struct NodeInspectOutput {
    uri: String,
    node_type: String,
    child_addresses: Vec<String>,
    details: serde_json::Value,
}

pub fn handle_node_cmd(
    args: NodeArgs,
    json: bool,
    quiet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match args.action {
        NodeAction::Inspect { uri } => {
            let addr: Address = uri.parse()?;
            let node = Node::from_address(&addr)?;
            let children: Vec<String> = node
                .child_addresses()
                .into_iter()
                .map(|a| a.to_uri())
                .collect();

            let details = match &node {
                Node::Data(bytes) => serde_json::json!({
                    "bytes_len": bytes.len(),
                    "hex": hex::encode(bytes),
                    "text": String::from_utf8(bytes.clone()).ok(),
                }),
                Node::Reference(target) => serde_json::json!({
                    "target_uri": target.to_uri(),
                }),
                Node::Sequence(children) => serde_json::json!({
                    "child_count": children.len(),
                    "children": children.iter().map(|c| c.to_uri()).collect::<Vec<_>>(),
                }),
                Node::Repeat { target, count } => serde_json::json!({
                    "target_uri": target.to_uri(),
                    "count": count,
                }),
                Node::Slice {
                    target,
                    offset,
                    length,
                } => serde_json::json!({
                    "target_uri": target.to_uri(),
                    "offset": offset,
                    "length": length,
                }),
                Node::Page(page) => serde_json::json!({
                    "logical_len": page.logical_len(),
                    "is_full": page.is_full(),
                }),
            };

            if json {
                let out = NodeInspectOutput {
                    uri: addr.to_uri(),
                    node_type: node.node_type().as_str().to_string(),
                    child_addresses: children,
                    details,
                };
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else if quiet {
                println!("{}", addr.to_uri());
            } else {
                println!("MFAS Recursive Node");
                println!("──────────────────────────────────────────");
                println!("Address:      {}", addr.to_uri());
                println!("Type:         {}", node.node_type());
                println!("Children:     {} child node(s)", children.len());
                for (i, c) in children.iter().enumerate() {
                    println!("  [{}] {}", i, c);
                }
                println!("Details:      {}", details);
            }
        }
        NodeAction::Data { input, hex } => {
            let bytes = if hex {
                hex::decode(input.trim())?
            } else {
                input.into_bytes()
            };
            let node = Node::Data(bytes);
            let addr = node.to_address()?;
            print_address(&addr, json, quiet)?;
        }
        NodeAction::Ref { target } => {
            let target_addr: Address = target.parse()?;
            let node = Node::Reference(target_addr);
            let addr = node.to_address()?;
            print_address(&addr, json, quiet)?;
        }
        NodeAction::Seq { children } => {
            let mut addrs = Vec::with_capacity(children.len());
            for c in children {
                addrs.push(c.parse::<Address>()?);
            }
            let node = Node::Sequence(addrs);
            let addr = node.to_address()?;
            print_address(&addr, json, quiet)?;
        }
        NodeAction::Repeat { target, count } => {
            let target_addr: Address = target.parse()?;
            let node = Node::Repeat {
                target: target_addr,
                count,
            };
            let addr = node.to_address()?;
            print_address(&addr, json, quiet)?;
        }
        NodeAction::Slice {
            target,
            offset,
            length,
        } => {
            let target_addr: Address = target.parse()?;
            let node = Node::Slice {
                target: target_addr,
                offset,
                length,
            };
            let addr = node.to_address()?;
            print_address(&addr, json, quiet)?;
        }
    }
    Ok(())
}

fn print_address(addr: &Address, json: bool, quiet: bool) -> Result<(), Box<dyn std::error::Error>> {
    if json {
        let out = serde_json::json!({
            "uri": addr.to_uri(),
            "node_type": addr.node_type().as_str(),
            "payload_len_bytes": addr.payload().len(),
        });
        println!("{}", serde_json::to_string_pretty(&out)?);
    } else if quiet {
        println!("{}", addr.to_uri());
    } else {
        println!("{}", addr.to_uri());
    }
    Ok(())
}
