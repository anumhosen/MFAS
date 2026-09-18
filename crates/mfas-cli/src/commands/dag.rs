use clap::{Args, Subcommand};
use mfas_core::address::Address;
use mfas_core::dag::{build_dag, optimize_dag};
use mfas_core::resolver::FileStore;
use serde_json::json;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct DagArgs {
    #[command(subcommand)]
    pub command: DagSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum DagSubcommand {
    /// Show topological statistics and sharing ratio for an MFAS DAG
    Stats {
        /// Target root address
        address: String,
    },
    /// Export DAG into Graphviz DOT format for visual rendering
    Dot {
        /// Target root address
        address: String,
        /// Optional path to write the .dot file (defaults to stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Optimize DAG using Common Subexpression Elimination and structural reductions
    Optimize {
        /// Target root address
        address: String,
    },
}

pub fn handle_dag_cmd(
    args: DagArgs,
    json_mode: bool,
    quiet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut store = FileStore::default_store()?;

    match args.command {
        DagSubcommand::Stats { address } => {
            let addr: Address = address.parse()?;
            let dag = build_dag(&addr, &store)?;
            let stats = dag.stats();

            if json_mode {
                println!("{}", serde_json::to_string_pretty(&stats)?);
            } else if !quiet {
                println!("MFAS DAG Topology Statistics");
                println!("──────────────────────────────────────────");
                println!("Root Address:    {}", stats.root_address);
                println!("Total Nodes (V): {}", stats.total_nodes);
                println!("Total Edges (E): {}", stats.total_edges);
                println!("Shared Nodes:    {}", stats.shared_nodes);
                println!("Leaf Nodes:      {}", stats.leaf_nodes);
                println!("Max Depth:       {}", stats.max_depth);
                println!("Sharing Ratio:   {:.4}", stats.sharing_ratio);
            }
        }
        DagSubcommand::Dot { address, output } => {
            let addr: Address = address.parse()?;
            let dag = build_dag(&addr, &store)?;
            let dot = dag.to_dot();

            if let Some(out_path) = output {
                let mut file = File::create(&out_path)?;
                file.write_all(dot.as_bytes())?;
                if json_mode {
                    println!(
                        "{}",
                        json!({
                            "address": addr.to_uri(),
                            "output_path": out_path.display().to_string(),
                            "bytes": dot.len(),
                        })
                    );
                } else if !quiet {
                    println!("Exported Graphviz DOT to {}", out_path.display());
                }
            } else if json_mode {
                println!(
                    "{}",
                    json!({
                        "address": addr.to_uri(),
                        "dot": dot,
                    })
                );
            } else {
                print!("{}", dot);
            }
        }
        DagSubcommand::Optimize { address } => {
            let addr: Address = address.parse()?;
            let report = optimize_dag(&addr, &mut store)?;

            if json_mode {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else if !quiet {
                println!("MFAS DAG Optimization");
                println!("──────────────────────────────────────────");
                println!("Original Root:   {}", report.original_root.to_uri());
                println!("Optimized Root:  {}", report.optimized_root.to_uri());
                println!(
                    "Nodes:           {} -> {} ({})",
                    report.nodes_before,
                    report.nodes_after,
                    report.nodes_after as isize - report.nodes_before as isize
                );
                println!(
                    "Edges:           {} -> {} ({})",
                    report.edges_before,
                    report.edges_after,
                    report.edges_after as isize - report.edges_before as isize
                );

                if !report.transformations.is_empty() {
                    println!("\nTransformations Applied:");
                    for t in &report.transformations {
                        println!("  • {}", t);
                    }
                } else {
                    println!("\nGraph is already canonically optimal (0 transformations needed).");
                }
            } else {
                println!("{}", report.optimized_root.to_uri());
            }
        }
    }

    Ok(())
}
