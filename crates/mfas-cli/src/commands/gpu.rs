use clap::{Args, Subcommand};
use mfas_gpu::{
    CpuEvaluator, Evaluator, GpuEvaluator, SimdEvaluator,
};
use serde_json::json;
use std::time::Instant;

#[derive(Args, Debug)]
pub struct GpuArgs {
    #[command(subcommand)]
    pub command: Option<GpuSubcommand>,

    /// Size in bytes for evaluator throughput comparison (default: 10485760 / 10MB)
    #[arg(short, long, default_value_t = 10_485_760, global = true)]
    pub size: usize,
}

#[derive(Subcommand, Debug)]
pub enum GpuSubcommand {
    /// Show detected GPU compute hardware, driver backends, and fallback status
    Info,
    /// Compare CPU vs SIMD vs GPU evaluator generation throughput
    Bench,
}

pub fn handle_gpu_cmd(
    args: GpuArgs,
    json_mode: bool,
    quiet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let sub = args.command.unwrap_or(GpuSubcommand::Info);

    match sub {
        GpuSubcommand::Info => {
            let gpu = GpuEvaluator::new();
            let adapters = GpuEvaluator::detect_adapters();

            if json_mode {
                let out = json!({
                    "gpu_available": gpu.is_available(),
                    "active_evaluator": gpu.name(),
                    "adapters": adapters,
                });
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else if !quiet {
                println!("MFAS GPU Compute Environment");
                println!("──────────────────────────────────────────");
                println!("Hardware Available: {}", if gpu.is_available() { "YES" } else { "NO (CPU Fallback)" });
                println!("Active Engine:      {}", gpu.name());
                println!("\nDetected Compute Adapters ({}):", adapters.len());
                for (i, a) in adapters.iter().enumerate() {
                    println!("  [{}] {}", i + 1, a.adapter_name);
                    println!("      Backend:         {}", a.backend);
                    println!("      Device Type:     {}", a.device_type);
                    println!("      Dedicated VRAM:  {}", if a.is_dedicated { "Yes" } else { "Shared" });
                    println!("      Max Buffer Size: {:.2} GiB", a.max_buffer_size as f64 / (1024.0 * 1024.0 * 1024.0));
                }
            } else {
                println!("{}", gpu.name());
            }
        }
        GpuSubcommand::Bench => {
            let size = args.size;
            let size_mb = size as f64 / (1024.0 * 1024.0);

            // 1. CPU Evaluator
            let cpu = CpuEvaluator::new();
            let start_cpu = Instant::now();
            let _cpu_bytes = cpu.generate_bytes("counter", size, 0)?;
            let cpu_dur = start_cpu.elapsed().as_secs_f64();
            let cpu_mb_s = if cpu_dur > 0.0 { size_mb / cpu_dur } else { 0.0 };

            // 2. SIMD Evaluator
            let simd = SimdEvaluator::new();
            let start_simd = Instant::now();
            let _simd_bytes = simd.generate_bytes("counter", size, 0)?;
            let simd_dur = start_simd.elapsed().as_secs_f64();
            let simd_mb_s = if simd_dur > 0.0 { size_mb / simd_dur } else { 0.0 };

            // 3. GPU Evaluator
            let gpu = GpuEvaluator::new();
            let start_gpu = Instant::now();
            let _gpu_bytes = gpu.generate_bytes("counter", size, 0)?;
            let gpu_dur = start_gpu.elapsed().as_secs_f64();
            let gpu_mb_s = if gpu_dur > 0.0 { size_mb / gpu_dur } else { 0.0 };

            if json_mode {
                let out = json!({
                    "size_bytes": size,
                    "cpu_mb_s": cpu_mb_s,
                    "simd_mb_s": simd_mb_s,
                    "gpu_mb_s": gpu_mb_s,
                    "gpu_backend": gpu.name(),
                });
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else if !quiet {
                println!("MFAS Evaluator Hardware Comparison Benchmark");
                println!("──────────────────────────────────────────");
                println!("Payload Size:   {} bytes ({:.2} MiB)", size, size_mb);
                println!("Pattern:        counter (0..255)");
                println!("\nThroughput Results:");
                println!("  • CPU Evaluator:   {:.2} MB/s ({:.4} s)", cpu_mb_s, cpu_dur);
                println!("  • SIMD Evaluator:  {:.2} MB/s ({:.4} s)", simd_mb_s, simd_dur);
                println!("  • GPU Evaluator:   {:.2} MB/s ({:.4} s) [{}]", gpu_mb_s, gpu_dur, gpu.name());
            }
        }
    }

    Ok(())
}
