//! DAG Inspection & Structural Visualization Engine
//!
//! Provides topological DAG inspection, node statistics calculation,
//! and ASCII hierarchy tree generation for arbitrary MFAS addresses.

use crate::address::Address;
use crate::error::CoreError;
use crate::node::Node;
use crate::page::PAGE_SIZE;
use crate::resolver::AddressStore;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Comprehensive topological inspection summary
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectionReport {
    pub address: String,
    pub node_type: String,
    pub total_bytes: u64,
    pub page_count: usize,
    pub max_depth: usize,
    pub total_nodes: usize,
    pub unique_nodes: usize,
    pub tree_ascii: String,
}

/// Inspect an Address DAG, computing structural statistics and ASCII visualization
pub fn inspect_address(
    addr: &Address,
    store: &dyn AddressStore,
    max_tree_depth: usize,
) -> Result<InspectionReport, CoreError> {
    let mut visited_unique: HashSet<Address> = HashSet::new();
    let mut stats = CollectorStats::default();

    collect_stats(addr, store, 1, &mut visited_unique, &mut stats)?;

    let mut tree_lines = Vec::new();
    render_tree_node(addr, store, "", true, 0, max_tree_depth, &mut tree_lines)?;
    let tree_ascii = tree_lines.join("\n");

    Ok(InspectionReport {
        address: addr.to_uri(),
        node_type: addr.node_type().as_str().to_string(),
        total_bytes: stats.total_bytes,
        page_count: stats.page_count,
        max_depth: stats.max_depth,
        total_nodes: stats.total_nodes,
        unique_nodes: visited_unique.len(),
        tree_ascii,
    })
}

#[derive(Default)]
struct CollectorStats {
    total_bytes: u64,
    page_count: usize,
    max_depth: usize,
    total_nodes: usize,
}

fn collect_stats(
    addr: &Address,
    store: &dyn AddressStore,
    current_depth: usize,
    visited: &mut HashSet<Address>,
    stats: &mut CollectorStats,
) -> Result<u64, CoreError> {
    visited.insert(addr.clone());
    stats.total_nodes += 1;
    if current_depth > stats.max_depth {
        stats.max_depth = current_depth;
    }

    let node = match store.get(addr)? {
        Some(n) => n,
        None => Node::from_address(addr)?,
    };

    let bytes = match node {
        Node::Data(bytes) => {
            let len = bytes.len() as u64;
            stats.total_bytes += len;
            len
        }
        Node::Page(page) => {
            let len = page.logical_len() as u64;
            stats.total_bytes += len;
            stats.page_count += 1;
            len
        }
        Node::Reference(target) => {
            collect_stats(&target, store, current_depth + 1, visited, stats)?
        }
        Node::Sequence(children) => {
            let mut sum = 0u64;
            for child in &children {
                sum += collect_stats(child, store, current_depth + 1, visited, stats)?;
            }
            sum
        }
        Node::Repeat { target, count } => {
            let single_bytes = collect_stats(&target, store, current_depth + 1, visited, stats)?;
            let total_rep_bytes = single_bytes.saturating_mul(count.saturating_sub(1));
            stats.total_bytes += total_rep_bytes;
            single_bytes.saturating_mul(count)
        }
        Node::Slice {
            target,
            offset: _,
            length,
        } => {
            let _ = collect_stats(&target, store, current_depth + 1, visited, stats)?;
            stats.total_bytes += length;
            length
        }
    };

    Ok(bytes)
}

fn render_tree_node(
    addr: &Address,
    store: &dyn AddressStore,
    prefix: &str,
    is_last: bool,
    depth: usize,
    max_depth: usize,
    lines: &mut Vec<String>,
) -> Result<(), CoreError> {
    let branch = if depth == 0 {
        "ROOT"
    } else if is_last {
        "└── "
    } else {
        "├── "
    };

    let node_opt = match store.get(addr)? {
        Some(n) => Some(n),
        None => Node::from_address(addr).ok(),
    };

    let label = match &node_opt {
        Some(Node::Data(b)) => format!("Data ({} bytes)", b.len()),
        Some(Node::Page(p)) => {
            if p.is_full() {
                format!("Page ({PAGE_SIZE} B)")
            } else {
                format!("Partial Page ({} B)", p.logical_len())
            }
        }
        Some(Node::Reference(_)) => "Ref".to_string(),
        Some(Node::Sequence(c)) => format!("Sequence ({} children)", c.len()),
        Some(Node::Repeat { count, .. }) => format!("Repeat (x{})", count),
        Some(Node::Slice { offset, length, .. }) => {
            format!("Slice [{}..{}]", offset, offset + length)
        }
        None => format!("Unresolved [{}]", addr.to_uri()),
    };

    if depth == 0 {
        lines.push(format!("{label} ({})", addr.to_uri()));
    } else {
        lines.push(format!("{prefix}{branch}{label}"));
    }

    if depth >= max_depth {
        return Ok(());
    }

    if let Some(node) = node_opt {
        let children = node.child_addresses();
        let child_count = children.len();
        let next_prefix = if depth == 0 {
            ""
        } else if is_last {
            &format!("{prefix}    ")
        } else {
            &format!("{prefix}│   ")
        };

        for (idx, child_addr) in children.into_iter().enumerate() {
            let child_is_last = idx + 1 == child_count;
            render_tree_node(
                child_addr,
                store,
                next_prefix,
                child_is_last,
                depth + 1,
                max_depth,
                lines,
            )?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::page::Page;
    use crate::resolver::MemoryStore;

    #[test]
    fn test_inspect_simple_tree() {
        let mut store = MemoryStore::new();
        let p1 = Page::new(vec![1; 4096]).unwrap();
        let p2 = Page::new(vec![2; 500]).unwrap();

        let a1 = p1.to_address(0);
        let a2 = p2.to_address(1);
        store.put(&a1, Node::Page(p1)).unwrap();
        store.put(&a2, Node::Page(p2)).unwrap();

        let root_node = Node::Sequence(vec![a1, a2]);
        let root_addr = root_node.to_address().unwrap();
        store.put(&root_addr, root_node).unwrap();

        let report = inspect_address(&root_addr, &store, 5).unwrap();
        assert_eq!(report.node_type, "seq");
        assert_eq!(report.total_bytes, 4096 + 500);
        assert_eq!(report.page_count, 2);
        assert_eq!(report.total_nodes, 3);
        assert!(report.tree_ascii.contains("Page (4096 B)"));
        assert!(report.tree_ascii.contains("Partial Page (500 B)"));
    }
}
