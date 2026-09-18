//! Directed Acyclic Graph (DAG) Engine & Optimization
//!
//! Provides:
//! - Structural representation and analysis of MFAS address graphs
//! - Metric calculation (sharing ratio, in/out degrees, topological depth)
//! - Graphviz DOT language export for visual architecture diagrams
//! - Common Subexpression Elimination (CSE) and graph normalization / reductions

use crate::address::{Address, NodeType};
use crate::error::CoreError;
use crate::node::Node;
use crate::resolver::AddressStore;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

/// Structural graph representation of an MFAS address hierarchy
#[derive(Debug, Clone)]
pub struct DagGraph {
    /// Root address of the DAG
    pub root: Address,
    /// Discovered nodes in the graph
    pub nodes: HashMap<Address, Node>,
    /// Directed edges (parent, child)
    pub edges: Vec<(Address, Address)>,
    /// In-degree per node (number of incoming references)
    pub in_degree: HashMap<Address, usize>,
    /// Out-degree per node (number of outgoing dependencies)
    pub out_degree: HashMap<Address, usize>,
    /// Topological depth from root (root = 1)
    pub depth_map: HashMap<Address, usize>,
}

/// Comprehensive topological metrics for an MFAS DAG
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DagStats {
    /// Root address formatted as canonical URI
    pub root_address: String,
    /// Total number of unique nodes (vertices $V$)
    pub total_nodes: usize,
    /// Total directed reference edges ($E$)
    pub total_edges: usize,
    /// Number of nodes referenced by more than one parent (shared subtrees)
    pub shared_nodes: usize,
    /// Number of terminal nodes (leaves with out-degree 0)
    pub leaf_nodes: usize,
    /// Maximum path depth from root to any leaf
    pub max_depth: usize,
    /// Sharing ratio measure: $(E - V + 1) / \max(1, E)$, indicating subexpression reuse
    pub sharing_ratio: f64,
}

/// Report emitted after running CSE and structural graph reductions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptimizationReport {
    /// Original root address
    pub original_root: Address,
    /// Optimized root address
    pub optimized_root: Address,
    /// Node count before optimization
    pub nodes_before: usize,
    /// Node count after optimization
    pub nodes_after: usize,
    /// Edge count before optimization
    pub edges_before: usize,
    /// Edge count after optimization
    pub edges_after: usize,
    /// Human-readable list of structural transformations performed
    pub transformations: Vec<String>,
}

/// Build a `DagGraph` by traversing all reachable nodes from a root `Address`
pub fn build_dag(root: &Address, store: &dyn AddressStore) -> Result<DagGraph, CoreError> {
    let mut nodes = HashMap::new();
    let mut edges = Vec::new();
    let mut in_degree: HashMap<Address, usize> = HashMap::new();
    let mut out_degree: HashMap<Address, usize> = HashMap::new();
    let mut depth_map: HashMap<Address, usize> = HashMap::new();

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((root.clone(), 1usize));
    visited.insert(root.clone());
    depth_map.insert(root.clone(), 1);

    while let Some((curr_addr, depth)) = queue.pop_front() {
        in_degree.entry(curr_addr.clone()).or_insert(0);

        // Fetch node from store or parse self-describing address
        let node = match store.get(&curr_addr) {
            Ok(Some(n)) => n,
            _ => match Node::from_address(&curr_addr) {
                Ok(n) => n,
                Err(_) => {
                    // Unresolvable node recorded as empty data node leaf
                    Node::Data(vec![])
                }
            },
        };

        let children = node.children();
        out_degree.insert(curr_addr.clone(), children.len());

        for child in children {
            edges.push((curr_addr.clone(), child.clone()));
            *in_degree.entry(child.clone()).or_insert(0) += 1;

            let next_depth = depth + 1;
            let current_child_depth = depth_map.entry(child.clone()).or_insert(next_depth);
            if next_depth > *current_child_depth {
                *current_child_depth = next_depth;
            }

            if !visited.contains(&child) {
                visited.insert(child.clone());
                queue.push_back((child, next_depth));
            }
        }

        nodes.insert(curr_addr, node);
    }

    Ok(DagGraph {
        root: root.clone(),
        nodes,
        edges,
        in_degree,
        out_degree,
        depth_map,
    })
}

impl DagGraph {
    /// Calculate summary metrics for this graph
    pub fn stats(&self) -> DagStats {
        let total_nodes = self.nodes.len();
        let total_edges = self.edges.len();

        let shared_nodes = self
            .in_degree
            .values()
            .filter(|&&deg| deg > 1)
            .count();

        let leaf_nodes = self
            .out_degree
            .values()
            .filter(|&&deg| deg == 0)
            .count();

        let max_depth = self.depth_map.values().cloned().max().unwrap_or(0);

        let sharing_ratio = if total_edges > 0 && total_nodes > 0 {
            let extra_edges = (total_edges as isize) - (total_nodes as isize) + 1;
            if extra_edges > 0 {
                extra_edges as f64 / total_edges as f64
            } else {
                0.0
            }
        } else {
            0.0
        };

        DagStats {
            root_address: self.root.to_uri(),
            total_nodes,
            total_edges,
            shared_nodes,
            leaf_nodes,
            max_depth,
            sharing_ratio,
        }
    }

    /// Render the DAG into standard Graphviz DOT format
    pub fn to_dot(&self) -> String {
        let mut dot = String::new();
        dot.push_str("digraph MFAS {\n");
        dot.push_str("    rankdir=TB;\n");
        dot.push_str("    node [fontname=\"Helvetica,Arial,sans-serif\", fontsize=10, shape=box, style=\"filled,rounded\", penwidth=1.5];\n");
        dot.push_str("    edge [fontname=\"Helvetica,Arial,sans-serif\", fontsize=8, color=\"#555555\", arrowsize=0.8];\n\n");

        // Index nodes with short IDs
        let mut addr_to_id = HashMap::new();
        for (i, addr) in self.nodes.keys().enumerate() {
            addr_to_id.insert(addr, format!("node_{}", i));
        }

        for (addr, node) in &self.nodes {
            let id = &addr_to_id[addr];
            let is_root = addr == &self.root;

            let (type_name, fill_color, border_color) = match addr.node_type() {
                NodeType::Data => ("Data", "#E1F5FE", "#0288D1"),
                NodeType::Page => ("Page", "#E8F5E9", "#388E3C"),
                NodeType::Ref => ("Reference", "#FFF3E0", "#F57C00"),
                NodeType::Seq => ("Sequence", "#F3E5F5", "#7B1FA2"),
                NodeType::Rep => ("Repeat", "#FCE4EC", "#C2185B"),
                NodeType::Slice => ("Slice", "#FFFDE7", "#FBC02D"),
            };

            let short_uri = {
                let uri = addr.to_uri();
                if uri.len() > 32 {
                    format!("{}...{}", &uri[..18], &uri[uri.len() - 8..])
                } else {
                    uri
                }
            };

            let details = match node {
                Node::Data(b) => format!("{} bytes", b.len()),
                Node::Page(p) => format!("{} B", p.len()),
                Node::Sequence(ch) => format!("{} children", ch.len()),
                Node::Repeat { count, .. } => format!("repeat {}x", count),
                Node::Slice { offset, length, .. } => format!("[{}..+{}]", offset, length),
                Node::Reference(_) => "ref".to_string(),
            };

            let root_badge = if is_root { " [ROOT]\\n" } else { "" };
            let label = format!("{}{}\\n{}\\n{}", root_badge, type_name, details, short_uri);

            let border_width = if is_root { "3.0" } else { "1.5" };

            dot.push_str(&format!(
                "    {} [label=\"{}\", fillcolor=\"{}\", color=\"{}\", penwidth={}];\n",
                id, label, fill_color, border_color, border_width
            ));
        }

        dot.push('\n');

        // Edges
        for (from, to) in &self.edges {
            if let (Some(from_id), Some(to_id)) = (addr_to_id.get(from), addr_to_id.get(to)) {
                dot.push_str(&format!("    {} -> {};\n", from_id, to_id));
            }
        }

        dot.push_str("}\n");
        dot
    }
}

/// Optimize an address DAG using Common Subexpression Elimination (CSE) and structural reductions
pub fn optimize_dag(
    root: &Address,
    store: &mut dyn AddressStore,
) -> Result<OptimizationReport, CoreError> {
    let graph_before = build_dag(root, store)?;
    let stats_before = graph_before.stats();

    let mut transformations = Vec::new();
    let mut memo = HashMap::new();

    let optimized_root = optimize_node_recursive(root, store, &mut transformations, &mut memo)?;

    let graph_after = build_dag(&optimized_root, store)?;
    let stats_after = graph_after.stats();

    Ok(OptimizationReport {
        original_root: root.clone(),
        optimized_root,
        nodes_before: stats_before.total_nodes,
        nodes_after: stats_after.total_nodes,
        edges_before: stats_before.total_edges,
        edges_after: stats_after.total_edges,
        transformations,
    })
}

fn optimize_node_recursive(
    addr: &Address,
    store: &mut dyn AddressStore,
    transformations: &mut Vec<String>,
    memo: &mut HashMap<Address, Address>,
) -> Result<Address, CoreError> {
    if let Some(cached) = memo.get(addr) {
        return Ok(cached.clone());
    }

    let node = match store.get(addr) {
        Ok(Some(n)) => n,
        _ => match Node::from_address(addr) {
            Ok(n) => n,
            Err(_) => return Ok(addr.clone()),
        },
    };

    let optimized_node = match node {
        Node::Data(_) | Node::Page(_) => node,

        Node::Reference(target) => {
            let opt_target = optimize_node_recursive(&target, store, transformations, memo)?;
            // Collapse redundant reference: Ref(Ref(X)) -> Ref(X)
            if let Ok(Some(Node::Reference(sub_target))) = store.get(&opt_target) {
                transformations.push(format!("Collapsed reference chain to {}", sub_target.to_uri()));
                Node::Reference(sub_target)
            } else {
                Node::Reference(opt_target)
            }
        }

        Node::Repeat { target, count } => {
            if count == 0 {
                transformations.push("Collapsed zero-repeat to empty Data node".to_string());
                Node::Data(vec![])
            } else if count == 1 {
                let opt_target = optimize_node_recursive(&target, store, transformations, memo)?;
                transformations.push(format!("Collapsed 1x Repeat to target {}", opt_target.to_uri()));
                // Return target directly
                memo.insert(addr.clone(), opt_target.clone());
                return Ok(opt_target);
            } else {
                let opt_target = optimize_node_recursive(&target, store, transformations, memo)?;
                // Check if target is itself a Repeat: Repeat(Repeat(X, m), n) -> Repeat(X, m * n)
                let target_node = store.get(&opt_target).ok().flatten().or_else(|| Node::from_address(&opt_target).ok());
                if let Some(Node::Repeat { target: sub_target, count: sub_count }) = target_node {
                    let total_count = count.saturating_mul(sub_count);
                    transformations.push(format!(
                        "Merged nested repeats ({}x * {}x = {}x)",
                        count, sub_count, total_count
                    ));
                    Node::Repeat {
                        target: sub_target,
                        count: total_count,
                    }
                } else {
                    Node::Repeat {
                        target: opt_target,
                        count,
                    }
                }
            }
        }

        Node::Slice {
            target,
            offset,
            length,
        } => {
            if length == 0 {
                transformations.push("Collapsed zero-length slice to empty Data node".to_string());
                Node::Data(vec![])
            } else {
                let opt_target = optimize_node_recursive(&target, store, transformations, memo)?;
                // Check if target is a Data or Page node and slice covers entire content
                let target_node = store.get(&opt_target).ok().flatten().or_else(|| Node::from_address(&opt_target).ok());
                let full_length = match &target_node {
                    Some(Node::Data(b)) => Some(b.len() as u64),
                    Some(Node::Page(p)) => Some(p.len() as u64),
                    _ => None,
                };

                if let Some(total_len) = full_length {
                    if offset == 0 && length >= total_len {
                        transformations.push(format!(
                            "Eliminated redundant full-range slice on {}",
                            opt_target.to_uri()
                        ));
                        memo.insert(addr.clone(), opt_target.clone());
                        return Ok(opt_target);
                    }
                }

                Node::Slice {
                    target: opt_target,
                    offset,
                    length,
                }
            }
        }

        Node::Sequence(children) => {
            // 1. Recursively optimize all children
            let mut opt_children = Vec::new();
            for child in children {
                let opt_child = optimize_node_recursive(&child, store, transformations, memo)?;
                opt_children.push(opt_child);
            }

            // 2. Flatten nested sequences: Seq([Seq([A, B]), C]) -> Seq([A, B, C])
            let mut flattened = Vec::new();
            for child in opt_children {
                let child_node = store.get(&child).ok().flatten().or_else(|| Node::from_address(&child).ok());
                if let Some(Node::Sequence(sub_children)) = child_node {
                    transformations.push(format!(
                        "Flattened nested sequence with {} children",
                        sub_children.len()
                    ));
                    flattened.extend(sub_children);
                } else {
                    flattened.push(child);
                }
            }

            // 3. Fold adjacent identical children into Repeat: [A, A, A] -> [Repeat(A, 3)]
            let mut folded: Vec<Address> = Vec::new();
            let mut current_child: Option<(Address, u64)> = None;

            for child in flattened {
                match current_child.take() {
                    Some((prev_addr, count)) => {
                        if prev_addr == child {
                            current_child = Some((prev_addr, count + 1));
                        } else {
                            // Emit prev
                            if count == 1 {
                                folded.push(prev_addr);
                            } else {
                                let rep = Node::Repeat {
                                    target: prev_addr.clone(),
                                    count,
                                };
                                let rep_addr = rep.to_address()?;
                                store.put(&rep_addr, rep)?;
                                transformations.push(format!("Folded {}x adjacent occurrences into Repeat", count));
                                folded.push(rep_addr);
                            }
                            current_child = Some((child, 1));
                        }
                    }
                    None => {
                        current_child = Some((child, 1));
                    }
                }
            }

            if let Some((prev_addr, count)) = current_child {
                if count == 1 {
                    folded.push(prev_addr);
                } else {
                    let rep = Node::Repeat {
                        target: prev_addr.clone(),
                        count,
                    };
                    let rep_addr = rep.to_address()?;
                    store.put(&rep_addr, rep)?;
                    transformations.push(format!("Folded {}x adjacent occurrences into Repeat", count));
                    folded.push(rep_addr);
                }
            }

            // 4. If sequence has 0 children -> empty Data
            if folded.is_empty() {
                transformations.push("Empty sequence collapsed to empty Data node".to_string());
                Node::Data(vec![])
            } else if folded.len() == 1 {
                // Collapse 1-child sequence: Seq([A]) -> A
                let single = folded.remove(0);
                transformations.push(format!("Collapsed single-child sequence to {}", single.to_uri()));
                memo.insert(addr.clone(), single.clone());
                return Ok(single);
            } else {
                Node::Sequence(folded)
            }
        }
    };

    let result_addr = optimized_node.to_address()?;
    store.put(&result_addr, optimized_node)?;
    memo.insert(addr.clone(), result_addr.clone());
    Ok(result_addr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resolver::{resolve, Limits, MemoryStore};

    #[test]
    fn test_dag_stats_diamond() {
        let mut store = MemoryStore::new();

        // C (leaf)
        let node_c = Node::Data(b"DataC".to_vec());
        let addr_c = node_c.to_address().unwrap();
        store.put(&addr_c, node_c).unwrap();

        // A -> C (Repeat 2x)
        let node_a = Node::Repeat {
            target: addr_c.clone(),
            count: 2,
        };
        let addr_a = node_a.to_address().unwrap();
        store.put(&addr_a, node_a).unwrap();

        // B -> C (Repeat 3x)
        let node_b = Node::Repeat {
            target: addr_c.clone(),
            count: 3,
        };
        let addr_b = node_b.to_address().unwrap();
        store.put(&addr_b, node_b).unwrap();

        // Root -> [A, B] (Diamond)
        let root = Node::Sequence(vec![addr_a, addr_b]).to_address().unwrap();

        let dag = build_dag(&root, &store).unwrap();
        let stats = dag.stats();

        assert_eq!(stats.total_nodes, 4); // Root, A, B, C
        assert_eq!(stats.total_edges, 4); // Root->A, Root->B, A->C, B->C
        assert_eq!(stats.shared_nodes, 1); // C has in-degree 2
        assert_eq!(stats.leaf_nodes, 1); // C is leaf
        assert!(stats.sharing_ratio > 0.0);
    }

    #[test]
    fn test_dag_dot_export() {
        let mut store = MemoryStore::new();
        let data = Node::Data(b"Graphviz".to_vec());
        let addr = data.to_address().unwrap();
        store.put(&addr, data).unwrap();

        let root = Node::Repeat {
            target: addr,
            count: 3,
        }
        .to_address()
        .unwrap();

        let dag = build_dag(&root, &store).unwrap();
        let dot = dag.to_dot();

        assert!(dot.starts_with("digraph MFAS {"));
        assert!(dot.contains("Repeat"));
        assert!(dot.contains("Data"));
        assert!(dot.ends_with("}\n"));
    }

    #[test]
    fn test_dag_optimize_flattens_and_preserves_content() {
        let mut store = MemoryStore::new();
        let limits = Limits::default();

        let leaf1 = Node::Data(b"Part1".to_vec()).to_address().unwrap();
        let leaf2 = Node::Data(b"Part2".to_vec()).to_address().unwrap();

        // Nested sequence: Seq([Seq([leaf1, leaf2])])
        let inner_seq = Node::Sequence(vec![leaf1.clone(), leaf2.clone()])
            .to_address()
            .unwrap();
        let outer_seq = Node::Sequence(vec![inner_seq]).to_address().unwrap();

        let original_bytes = resolve(&outer_seq, &store, &limits).unwrap();
        assert_eq!(original_bytes, b"Part1Part2");

        let report = optimize_dag(&outer_seq, &mut store).unwrap();
        assert!(report.transformations.len() > 0);

        // Verify invariant: decode(optimized) == decode(original)
        let optimized_bytes = resolve(&report.optimized_root, &store, &limits).unwrap();
        assert_eq!(optimized_bytes, original_bytes);
    }

    #[test]
    fn test_dag_optimize_folds_adjacent_repeats() {
        let mut store = MemoryStore::new();
        let limits = Limits::default();

        let leaf = Node::Data(b"R".to_vec()).to_address().unwrap();
        // Sequence with [R, R, R]
        let seq = Node::Sequence(vec![leaf.clone(), leaf.clone(), leaf.clone()])
            .to_address()
            .unwrap();

        let report = optimize_dag(&seq, &mut store).unwrap();
        assert!(report.transformations.iter().any(|t| t.contains("Folded 3x")));

        let optimized_bytes = resolve(&report.optimized_root, &store, &limits).unwrap();
        assert_eq!(optimized_bytes, b"RRR");
    }
}
