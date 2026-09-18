use std::process::Command;

#[test]
fn test_cli_dag_stats_and_json() {
    // 1. Generate a synthetic multi-page tree
    let gen_out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["--json", "generate", "synthetic", "--pattern", "counter", "--size", "10000"])
        .output()
        .unwrap();
    assert!(gen_out.status.success());
    let gen_json: serde_json::Value = serde_json::from_slice(&gen_out.stdout).unwrap();
    let root_addr = gen_json["root_address"].as_str().unwrap();

    // 2. Query DAG stats in JSON
    let stats_out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["--json", "dag", "stats", root_addr])
        .output()
        .unwrap();
    assert!(stats_out.status.success());
    let stats_json: serde_json::Value = serde_json::from_slice(&stats_out.stdout).unwrap();
    assert_eq!(stats_json["root_address"], root_addr);
    assert_eq!(stats_json["total_nodes"], 4); // Root seq + 3 page nodes
    assert_eq!(stats_json["total_edges"], 3);
    assert_eq!(stats_json["leaf_nodes"], 2);

    // 3. Human readable stats
    let stats_human = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["dag", "stats", root_addr])
        .output()
        .unwrap();
    assert!(stats_human.status.success());
    let stdout = String::from_utf8_lossy(&stats_human.stdout);
    assert!(stdout.contains("MFAS DAG Topology Statistics"));
    assert!(stdout.contains("Total Nodes (V): 4"));
}

#[test]
fn test_cli_dag_dot_export() {
    let gen_out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["--json", "generate", "synthetic", "--pattern", "zeros", "--size", "8192"])
        .output()
        .unwrap();
    assert!(gen_out.status.success());
    let gen_json: serde_json::Value = serde_json::from_slice(&gen_out.stdout).unwrap();
    let root_addr = gen_json["root_address"].as_str().unwrap();

    let dot_out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["dag", "dot", root_addr])
        .output()
        .unwrap();
    assert!(dot_out.status.success());
    let dot = String::from_utf8_lossy(&dot_out.stdout);
    assert!(dot.starts_with("digraph MFAS {"));
    assert!(dot.contains("->"));
    assert!(dot.contains("}\n"));
}

#[test]
fn test_cli_dag_optimize() {
    // Create data node
    let node_out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["node", "data", "--input", "DAGOptimizationTest"])
        .output()
        .unwrap();
    assert!(node_out.status.success());
    let data_addr = String::from_utf8_lossy(&node_out.stdout).trim().to_string();

    // Create 1x repeat of that node
    let rep_out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["node", "repeat", "--target", &data_addr, "--count", "1"])
        .output()
        .unwrap();
    assert!(rep_out.status.success());
    let rep_addr = String::from_utf8_lossy(&rep_out.stdout).trim().to_string();

    // Optimize DAG on rep_addr -> should collapse 1x Repeat to the target data node
    let opt_out = Command::new(env!("CARGO_BIN_EXE_mfas"))
        .args(["--json", "dag", "optimize", &rep_addr])
        .output()
        .unwrap();
    assert!(opt_out.status.success());
    let report: serde_json::Value = serde_json::from_slice(&opt_out.stdout).unwrap();
    assert_eq!(report["optimized_root"], data_addr);
    assert!(report["transformations"].as_array().unwrap().len() > 0);
}
