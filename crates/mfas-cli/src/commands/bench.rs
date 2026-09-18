use clap::{Args, Subcommand};
use mfas_core::bench::{
    bench_address, bench_all, bench_codec, bench_dag, bench_page,
};

#[derive(Args, Debug)]
pub struct BenchArgs {
    #[command(subcommand)]
    pub command: Option<BenchSubcommand>,

    /// Number of iterations for micro-benchmarks
    #[arg(short, long, default_value_t = 50_000, global = true)]
    pub iterations: u64,

    /// Data stream size in bytes for codec benchmarks (default: 10485760 / 10MB)
    #[arg(short, long, default_value_t = 10_485_760, global = true)]
    pub size: u64,
}

#[derive(Subcommand, Debug)]
pub enum BenchSubcommand {
    /// Benchmark all subsystems (Address, Page, Codec, DAG)
    All,
    /// Benchmark address parsing, formatting, and binary serialization
    Address,
    /// Benchmark radix-16 spatial coordinate calculations
    Page,
    /// Benchmark streaming encoding and decoding throughput (MB/s)
    Codec,
    /// Benchmark DAG traversal and CSE optimization speed
    Dag,
}

pub fn handle_bench_cmd(
    args: BenchArgs,
    json_mode: bool,
    quiet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let subcommand = args.command.unwrap_or(BenchSubcommand::All);

    match subcommand {
        BenchSubcommand::All => {
            let res = bench_all(args.iterations, args.size)?;
            if json_mode {
                println!("{}", serde_json::to_string_pretty(&res)?);
            } else if !quiet {
                println!("MFAS Comprehensive Performance Benchmark");
                println!("──────────────────────────────────────────");
                println!("Iterations:           {}", args.iterations);
                println!("Codec Stream Size:    {} bytes ({:.2} MiB)", args.size, args.size as f64 / (1024.0 * 1024.0));
                println!("\n[1] Address Subsystem:");
                println!("  • Parsing:          {:.0} ops/s ({:.2} ns/op)", res.address.parse_ops_per_sec, res.address.parse_nanos_per_op);
                println!("  • Formatting:       {:.0} ops/s ({:.2} ns/op)", res.address.format_ops_per_sec, res.address.format_nanos_per_op);
                println!("  • Binary Roundtrip: {:.0} ops/s ({:.2} ns/op)", res.address.binary_ops_per_sec, res.address.binary_nanos_per_op);
                println!("\n[2] Radix-16 Page Coordinates:");
                println!("  • Coordinates:      {:.0} ops/s ({:.2} ns/op)", res.page.coordinate_ops_per_sec, res.page.coordinate_nanos_per_op);
                println!("\n[3] Streaming Codec:");
                println!("  • Encoding:         {:.2} MB/s ({:.3} s)", res.codec.encode_throughput_mb_s, res.codec.encode_duration_secs);
                println!("  • Decoding:         {:.2} MB/s ({:.3} s)", res.codec.decode_throughput_mb_s, res.codec.decode_duration_secs);
                println!("\n[4] DAG Engine:");
                println!("  • Traversal/Build:  {:.0} nodes/s ({:.3} s)", res.dag.build_nodes_per_sec, res.dag.build_duration_secs);
                println!("  • CSE Optimization: {:.0} nodes/s ({:.3} s)", res.dag.optimize_nodes_per_sec, res.dag.optimize_duration_secs);
            }
        }
        BenchSubcommand::Address => {
            let res = bench_address(args.iterations);
            if json_mode {
                println!("{}", serde_json::to_string_pretty(&res)?);
            } else if !quiet {
                println!("MFAS Address Benchmark");
                println!("──────────────────────────────────────────");
                println!("Iterations:           {}", res.iterations);
                println!("Parsing:              {:.0} ops/s ({:.2} ns/op)", res.parse_ops_per_sec, res.parse_nanos_per_op);
                println!("Formatting:           {:.0} ops/s ({:.2} ns/op)", res.format_ops_per_sec, res.format_nanos_per_op);
                println!("Binary Serialization: {:.0} ops/s ({:.2} ns/op)", res.binary_ops_per_sec, res.binary_nanos_per_op);
            }
        }
        BenchSubcommand::Page => {
            let res = bench_page(args.iterations);
            if json_mode {
                println!("{}", serde_json::to_string_pretty(&res)?);
            } else if !quiet {
                println!("MFAS Page Coordinate Benchmark");
                println!("──────────────────────────────────────────");
                println!("Iterations:           {}", res.iterations);
                println!("Coordinate Transform: {:.0} ops/s ({:.2} ns/op)", res.coordinate_ops_per_sec, res.coordinate_nanos_per_op);
            }
        }
        BenchSubcommand::Codec => {
            let res = bench_codec(args.size)?;
            if json_mode {
                println!("{}", serde_json::to_string_pretty(&res)?);
            } else if !quiet {
                println!("MFAS Codec Streaming Benchmark");
                println!("──────────────────────────────────────────");
                println!("Stream Size:          {} bytes ({:.2} MiB)", res.size_bytes, res.size_bytes as f64 / (1024.0 * 1024.0));
                println!("Encoding Speed:       {:.2} MB/s ({:.3} s)", res.encode_throughput_mb_s, res.encode_duration_secs);
                println!("Decoding Speed:       {:.2} MB/s ({:.3} s)", res.decode_throughput_mb_s, res.decode_duration_secs);
            }
        }
        BenchSubcommand::Dag => {
            let res = bench_dag(args.iterations.min(10_000) as usize)?;
            if json_mode {
                println!("{}", serde_json::to_string_pretty(&res)?);
            } else if !quiet {
                println!("MFAS DAG Engine Benchmark");
                println!("──────────────────────────────────────────");
                println!("Node Count:           {}", res.node_count);
                println!("DAG Build / Travers:  {:.0} nodes/s ({:.3} s)", res.build_nodes_per_sec, res.build_duration_secs);
                println!("CSE Optimization:     {:.0} nodes/s ({:.3} s)", res.optimize_nodes_per_sec, res.optimize_duration_secs);
            }
        }
    }

    Ok(())
}
