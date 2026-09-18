//! Performance Benchmarks & Metrics
//!
//! Provides measurement of operations per second, latency, and throughput across:
//! - Address parsing, canonical formatting, and binary serialization
//! - Radix-16 spatial hierarchy coordinate computation
//! - Constant-memory streaming encoding and decoding throughput (MB/s)
//! - DAG traversal, CSE optimization, and node deduplication throughput

use crate::address::{Address, NodeType};
use crate::codec::{decode_to_writer, encode};
use crate::dag::{build_dag, optimize_dag};
use crate::error::CoreError;
use crate::node::Node;
use crate::page::RadixCoordinate;
use crate::resolver::{AddressStore, MemoryStore};
use crate::synthetic::SyntheticStream;
use serde::{Deserialize, Serialize};
use std::io::sink;
use std::time::Instant;

/// Benchmark metrics for address parsing and formatting
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddressBenchResult {
    pub iterations: u64,
    pub parse_ops_per_sec: f64,
    pub parse_nanos_per_op: f64,
    pub format_ops_per_sec: f64,
    pub format_nanos_per_op: f64,
    pub binary_ops_per_sec: f64,
    pub binary_nanos_per_op: f64,
}

/// Benchmark metrics for logical pages and radix-16 coordinates
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageBenchResult {
    pub iterations: u64,
    pub coordinate_ops_per_sec: f64,
    pub coordinate_nanos_per_op: f64,
}

/// Benchmark metrics for streaming codec throughput
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodecBenchResult {
    pub size_bytes: u64,
    pub encode_throughput_mb_s: f64,
    pub encode_duration_secs: f64,
    pub decode_throughput_mb_s: f64,
    pub decode_duration_secs: f64,
}

/// Benchmark metrics for DAG construction and optimization
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DagBenchResult {
    pub node_count: usize,
    pub build_nodes_per_sec: f64,
    pub build_duration_secs: f64,
    pub optimize_nodes_per_sec: f64,
    pub optimize_duration_secs: f64,
}

/// Comprehensive benchmark report combining all subsystem metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllBenchResult {
    pub address: AddressBenchResult,
    pub page: PageBenchResult,
    pub codec: CodecBenchResult,
    pub dag: DagBenchResult,
}

/// Benchmark Address parsing, canonical formatting, and binary roundtrips
pub fn bench_address(iterations: u64) -> AddressBenchResult {
    let uri_sample = "mfas:v1:data:48656c6c6f204d464153";
    let addr_sample = Address::from_hex(NodeType::Data, "48656c6c6f204d464153").unwrap();
    let binary_sample = addr_sample.to_binary();

    // 1. Benchmark URI Parsing
    let start_parse = Instant::now();
    for _ in 0..iterations {
        let addr: Address = uri_sample.parse().unwrap();
        std::hint::black_box(addr);
    }
    let parse_elapsed = start_parse.elapsed();
    let parse_secs = parse_elapsed.as_secs_f64();
    let parse_ops_per_sec = iterations as f64 / parse_secs.max(1e-9);
    let parse_nanos_per_op = (parse_elapsed.as_nanos() as f64) / iterations as f64;

    // 2. Benchmark URI Formatting
    let start_format = Instant::now();
    for _ in 0..iterations {
        let uri = addr_sample.to_uri();
        std::hint::black_box(uri);
    }
    let format_elapsed = start_format.elapsed();
    let format_secs = format_elapsed.as_secs_f64();
    let format_ops_per_sec = iterations as f64 / format_secs.max(1e-9);
    let format_nanos_per_op = (format_elapsed.as_nanos() as f64) / iterations as f64;

    // 3. Benchmark Binary Parsing
    let start_binary = Instant::now();
    for _ in 0..iterations {
        let addr = Address::from_binary(&binary_sample).unwrap();
        std::hint::black_box(addr);
    }
    let binary_elapsed = start_binary.elapsed();
    let binary_secs = binary_elapsed.as_secs_f64();
    let binary_ops_per_sec = iterations as f64 / binary_secs.max(1e-9);
    let binary_nanos_per_op = (binary_elapsed.as_nanos() as f64) / iterations as f64;

    AddressBenchResult {
        iterations,
        parse_ops_per_sec,
        parse_nanos_per_op,
        format_ops_per_sec,
        format_nanos_per_op,
        binary_ops_per_sec,
        binary_nanos_per_op,
    }
}

/// Benchmark Radix-16 coordinate calculations
pub fn bench_page(iterations: u64) -> PageBenchResult {
    let start_coord = Instant::now();
    for i in 0..iterations {
        let offset = (i * 1234567) ^ 0xA5A5A5;
        let coord = RadixCoordinate::from_byte_offset(offset);
        let back = coord.to_byte_offset();
        std::hint::black_box(back);
    }
    let coord_elapsed = start_coord.elapsed();
    let coord_secs = coord_elapsed.as_secs_f64();
    let coordinate_ops_per_sec = iterations as f64 / coord_secs.max(1e-9);
    let coordinate_nanos_per_op = (coord_elapsed.as_nanos() as f64) / iterations as f64;

    PageBenchResult {
        iterations,
        coordinate_ops_per_sec,
        coordinate_nanos_per_op,
    }
}

/// Benchmark streaming encoding and decoding throughput (MB/s)
pub fn bench_codec(size_bytes: u64) -> Result<CodecBenchResult, CoreError> {
    let mut store = MemoryStore::new();

    // 1. Benchmark Streaming Encode
    let mut stream = SyntheticStream::counter(size_bytes);
    let start_encode = Instant::now();
    let root_addr = encode(&mut stream, &mut store)?;
    let encode_secs = start_encode.elapsed().as_secs_f64();
    let size_mb = size_bytes as f64 / (1024.0 * 1024.0);
    let encode_throughput_mb_s = if encode_secs > 0.0 {
        size_mb / encode_secs
    } else {
        0.0
    };

    // 2. Benchmark Streaming Decode
    let mut sink_writer = sink();
    let start_decode = Instant::now();
    decode_to_writer(&root_addr, &store, &mut sink_writer)?;
    let decode_secs = start_decode.elapsed().as_secs_f64();
    let decode_throughput_mb_s = if decode_secs > 0.0 {
        size_mb / decode_secs
    } else {
        0.0
    };

    Ok(CodecBenchResult {
        size_bytes,
        encode_throughput_mb_s,
        encode_duration_secs: encode_secs,
        decode_throughput_mb_s,
        decode_duration_secs: decode_secs,
    })
}

/// Benchmark DAG graph traversal and CSE optimization speed
pub fn bench_dag(nodes: usize) -> Result<DagBenchResult, CoreError> {
    let mut store = MemoryStore::new();

    // Create a DAG with `nodes` sequential elements with duplicates to optimize
    let mut child_addrs = Vec::with_capacity(nodes);
    for i in 0..nodes {
        let data = format!("NodePayload_{}", i % 50).into_bytes();
        let node = Node::Data(data);
        let addr = node.to_address().unwrap();
        store.put(&addr, node).unwrap();
        child_addrs.push(addr);
    }

    let root_seq = Node::Sequence(child_addrs);
    let root_addr = root_seq.to_address().unwrap();
    store.put(&root_addr, root_seq).unwrap();

    // 1. Benchmark DAG Build
    let start_build = Instant::now();
    let dag = build_dag(&root_addr, &store)?;
    let _stats = dag.stats();
    let build_secs = start_build.elapsed().as_secs_f64();
    let build_nodes_per_sec = (nodes as f64) / build_secs.max(1e-9);

    // 2. Benchmark DAG Optimize
    let start_opt = Instant::now();
    let _report = optimize_dag(&root_addr, &mut store)?;
    let opt_secs = start_opt.elapsed().as_secs_f64();
    let optimize_nodes_per_sec = (nodes as f64) / opt_secs.max(1e-9);

    Ok(DagBenchResult {
        node_count: nodes,
        build_nodes_per_sec,
        build_duration_secs: build_secs,
        optimize_nodes_per_sec,
        optimize_duration_secs: opt_secs,
    })
}

/// Run all benchmarks
pub fn bench_all(iterations: u64, size_bytes: u64) -> Result<AllBenchResult, CoreError> {
    let address = bench_address(iterations);
    let page = bench_page(iterations);
    let codec = bench_codec(size_bytes)?;
    let dag = bench_dag(iterations.min(5000) as usize)?;

    Ok(AllBenchResult {
        address,
        page,
        codec,
        dag,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bench_address_execution() {
        let res = bench_address(1000);
        assert_eq!(res.iterations, 1000);
        assert!(res.parse_ops_per_sec > 0.0);
        assert!(res.format_ops_per_sec > 0.0);
        assert!(res.binary_ops_per_sec > 0.0);
    }

    #[test]
    fn test_bench_page_execution() {
        let res = bench_page(1000);
        assert_eq!(res.iterations, 1000);
        assert!(res.coordinate_ops_per_sec > 0.0);
    }

    #[test]
    fn test_bench_codec_execution() {
        let res = bench_codec(65536).unwrap();
        assert_eq!(res.size_bytes, 65536);
        assert!(res.encode_throughput_mb_s > 0.0);
        assert!(res.decode_throughput_mb_s > 0.0);
    }

    #[test]
    fn test_bench_dag_execution() {
        let res = bench_dag(100).unwrap();
        assert_eq!(res.node_count, 100);
        assert!(res.build_nodes_per_sec > 0.0);
        assert!(res.optimize_nodes_per_sec > 0.0);
    }
}
