use crate::commands::parse_address_str;
use mfas_core::dag::{build_dag, optimize_dag as core_optimize_dag};
use mfas_core::resolver::store::FileStore;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DagAnalysisDto {
    pub root_address: String,
    pub total_nodes: usize,
    pub total_edges: usize,
    pub shared_nodes: usize,
    pub leaf_nodes: usize,
    pub max_depth: usize,
    pub sharing_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DagOptimizationDto {
    pub original_root: String,
    pub optimized_root: String,
    pub nodes_before: usize,
    pub nodes_after: usize,
    pub edges_before: usize,
    pub edges_after: usize,
    pub nodes_eliminated: usize,
    pub reduction_pct: f64,
    pub transformations: Vec<String>,
}

#[tauri::command]
pub fn analyze_dag(raw_addr: String) -> Result<DagAnalysisDto, String> {
    let addr = parse_address_str(&raw_addr)?;
    let store = FileStore::default_store().map_err(|e| e.to_string())?;
    let dag = build_dag(&addr, &store).map_err(|e| e.to_string())?;
    let stats = dag.stats();

    Ok(DagAnalysisDto {
        root_address: stats.root_address,
        total_nodes: stats.total_nodes,
        total_edges: stats.total_edges,
        shared_nodes: stats.shared_nodes,
        leaf_nodes: stats.leaf_nodes,
        max_depth: stats.max_depth,
        sharing_ratio: stats.sharing_ratio,
    })
}

#[tauri::command]
pub fn get_dag_dot(raw_addr: String) -> Result<String, String> {
    let addr = parse_address_str(&raw_addr)?;
    let store = FileStore::default_store().map_err(|e| e.to_string())?;
    let dag = build_dag(&addr, &store).map_err(|e| e.to_string())?;
    Ok(dag.to_dot())
}

#[tauri::command]
pub fn optimize_dag(raw_addr: String) -> Result<DagOptimizationDto, String> {
    let addr = parse_address_str(&raw_addr)?;
    let mut store = FileStore::default_store().map_err(|e| e.to_string())?;
    let report = core_optimize_dag(&addr, &mut store).map_err(|e| e.to_string())?;

    let nodes_eliminated = report.nodes_before.saturating_sub(report.nodes_after);
    let reduction_pct = if report.nodes_before > 0 {
        (nodes_eliminated as f64 / report.nodes_before as f64) * 100.0
    } else {
        0.0
    };

    Ok(DagOptimizationDto {
        original_root: report.original_root.to_uri(),
        optimized_root: report.optimized_root.to_uri(),
        nodes_before: report.nodes_before,
        nodes_after: report.nodes_after,
        edges_before: report.edges_before,
        edges_after: report.edges_after,
        nodes_eliminated,
        reduction_pct,
        transformations: report.transformations,
    })
}
