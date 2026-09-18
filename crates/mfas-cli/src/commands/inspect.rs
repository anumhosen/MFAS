use clap::Args;
use mfas_core::{inspect_address, Address, FileStore};

#[derive(Args, Debug)]
pub struct InspectArgs {
    /// Canonical MFAS address URI to inspect
    pub address: String,

    /// Maximum visual depth for the ASCII tree
    #[arg(long, default_value_t = 10)]
    pub max_depth: usize,
}

pub fn handle_inspect_cmd(
    args: InspectArgs,
    json: bool,
    quiet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr: Address = args.address.parse()?;
    let store = FileStore::default_store()?;
    let report = inspect_address(&addr, &store, args.max_depth)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else if quiet {
        println!("{}", report.address);
    } else {
        println!("MFAS Address Inspection");
        println!("──────────────────────────────────────────");
        println!("Address:       {}", report.address);
        println!("Type:          {}", report.node_type);
        println!("Size:          {} bytes", report.total_bytes);
        println!("Pages:         {}", report.page_count);
        println!("Depth:         {}", report.max_depth);
        println!("Total Nodes:   {}", report.total_nodes);
        println!("Unique Nodes:  {}", report.unique_nodes);
        println!("\nDAG Hierarchy:");
        println!("{}", report.tree_ascii);
    }
    Ok(())
}
