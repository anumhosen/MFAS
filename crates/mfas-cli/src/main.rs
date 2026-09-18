mod commands;

use clap::{Parser, Subcommand};
use commands::address::{handle_address_cmd, AddressArgs};
use commands::bench::{handle_bench_cmd, BenchArgs};
use commands::codec::{
    handle_decode_cmd, handle_encode_cmd, handle_verify_cmd, DecodeArgs, EncodeArgs, VerifyArgs,
};
use commands::dag::{handle_dag_cmd, DagArgs};
use commands::generate::{handle_generate_cmd, GenerateArgs};
use commands::gpu::{handle_gpu_cmd, GpuArgs};
use commands::inspect::{handle_inspect_cmd, InspectArgs};
use commands::node::{handle_node_cmd, NodeArgs};
use commands::number::{handle_number_cmd, NumberArgs};
use commands::page::{handle_page_cmd, PageArgs};
use commands::resolve::{handle_resolve_cmd, ResolveArgs};
use serde_json::json;

#[derive(Parser, Debug)]
#[command(
    name = "mfas",
    author = "MFAS Development Team",
    version = "0.1.0",
    about = "Mathematical File Address Space - Addressing and Reconstruction CLI",
    long_about = "MFAS provides deterministic mathematical addressing, validation, and reconstruction of finite digital data."
)]
struct Cli {
    /// Emit machine-readable JSON output
    #[arg(long, global = true)]
    json: bool,

    /// Suppress non-essential output
    #[arg(short, long, global = true)]
    quiet: bool,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Manage, parse, format, and validate MFAS addresses
    Address(AddressArgs),
    /// Map between natural numbers and finite byte strings (N <-> B*)
    Number(NumberArgs),
    /// Inspect 4096-byte logical pages and Radix-16 spatial coordinates
    Page(PageArgs),
    /// Create, inspect, and manipulate recursive MFAS nodes
    Node(NodeArgs),
    /// Recursively resolve an MFAS address into concrete bytes or files
    Resolve(ResolveArgs),
    /// Encode an arbitrary file into a canonical root Address
    Encode(EncodeArgs),
    /// Reconstruct an original file from its canonical root Address
    Decode(DecodeArgs),
    /// Encode, reconstruct, and verify SHA-256 integrity of a file
    Verify(VerifyArgs),
    /// Topologically inspect an MFAS address and render its ASCII DAG hierarchy
    Inspect(InspectArgs),
    /// Deterministic large file generator and test data builder
    Generate(GenerateArgs),
    /// Directed Acyclic Graph analysis, DOT export, and CSE optimization
    Dag(DagArgs),
    /// Performance benchmarks across addressing, codec, pages, and DAG
    Bench(BenchArgs),
    /// GPU hardware compute adapter inspection and performance evaluation
    Gpu(GpuArgs),
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Address(args) => handle_address_cmd(args, cli.json, cli.quiet),
        Commands::Number(args) => handle_number_cmd(args, cli.json, cli.quiet),
        Commands::Page(args) => handle_page_cmd(args, cli.json, cli.quiet),
        Commands::Node(args) => handle_node_cmd(args, cli.json, cli.quiet),
        Commands::Resolve(args) => handle_resolve_cmd(args, cli.json, cli.quiet),
        Commands::Encode(args) => handle_encode_cmd(args, cli.json, cli.quiet),
        Commands::Decode(args) => handle_decode_cmd(args, cli.json, cli.quiet),
        Commands::Verify(args) => handle_verify_cmd(args, cli.json, cli.quiet),
        Commands::Inspect(args) => handle_inspect_cmd(args, cli.json, cli.quiet),
        Commands::Generate(args) => handle_generate_cmd(args, cli.json, cli.quiet),
        Commands::Dag(args) => handle_dag_cmd(args, cli.json, cli.quiet),
        Commands::Bench(args) => handle_bench_cmd(args, cli.json, cli.quiet),
        Commands::Gpu(args) => handle_gpu_cmd(args, cli.json, cli.quiet),
    };

    if let Err(e) = result {
        if cli.json {
            let err_json = json!({
                "error": true,
                "message": e.to_string(),
            });
            eprintln!("{}", err_json);
        } else {
            eprintln!("Error: {}", e);
        }
        std::process::exit(1);
    }
}
