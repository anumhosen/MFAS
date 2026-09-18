mod commands;

use clap::{Parser, Subcommand};
use commands::address::{handle_address_cmd, AddressArgs};
use commands::number::{handle_number_cmd, NumberArgs};
use commands::page::{handle_page_cmd, PageArgs};
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
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Address(args) => handle_address_cmd(args, cli.json, cli.quiet),
        Commands::Number(args) => handle_number_cmd(args, cli.json, cli.quiet),
        Commands::Page(args) => handle_page_cmd(args, cli.json, cli.quiet),
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
