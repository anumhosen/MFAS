//! Recursive Resolver & Streaming Reconstructor
//!
//! Provides deterministic evaluation of MFAS DAGs with:
//! - Cycle detection
//! - Safety limits (depth, output bytes, node count)
//! - Streaming page-by-page output without unbounded memory allocation

pub mod store;

pub use store::{AddressStore, FileStore, MemoryStore};

use crate::address::Address;
use crate::error::ResolveError;
use crate::node::Node;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;

/// Maximum size of a resolved node to store in the evaluation memoization cache
const MAX_MEMOIZE_NODE_SIZE: usize = 64 * 1024; // 64 KiB

/// Safety resource limits for recursive reconstruction
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Limits {
    /// Maximum recursion call-stack depth (default: 64)
    pub max_depth: usize,
    /// Maximum permitted reconstructed bytes (default: 10 GiB)
    pub max_output_bytes: u64,
    /// Maximum number of node evaluations (default: 1,000,000)
    pub max_node_count: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_depth: 128,
            max_output_bytes: 50 * 1024 * 1024 * 1024, // 50 GiB
            max_node_count: 10_000_000,
        }
    }
}

/// Internal tracker for resolution progress and safety enforcement
struct ResolverState {
    active_path: Vec<Address>,
    nodes_evaluated: usize,
    bytes_written: u64,
    memo_cache: HashMap<Address, Vec<u8>>,
}

impl ResolverState {
    fn new() -> Self {
        Self {
            active_path: Vec::new(),
            nodes_evaluated: 0,
            bytes_written: 0,
            memo_cache: HashMap::new(),
        }
    }
}

/// Recursively resolve an Address directly into a byte buffer
pub fn resolve(
    addr: &Address,
    store: &dyn AddressStore,
    limits: &Limits,
) -> Result<Vec<u8>, ResolveError> {
    let mut buf = Vec::new();
    resolve_to_writer(addr, store, limits, &mut buf)?;
    Ok(buf)
}

/// Recursively resolve an Address streaming directly into an `io::Write` sink
pub fn resolve_to_writer<W: Write>(
    addr: &Address,
    store: &dyn AddressStore,
    limits: &Limits,
    writer: &mut W,
) -> Result<u64, ResolveError> {
    let mut state = ResolverState::new();
    let dyn_writer: &mut dyn Write = writer;
    resolve_internal(addr, store, limits, &mut state, dyn_writer)?;
    Ok(state.bytes_written)
}

fn resolve_internal(
    addr: &Address,
    store: &dyn AddressStore,
    limits: &Limits,
    state: &mut ResolverState,
    writer: &mut dyn Write,
) -> Result<(), ResolveError> {
    // 1. Check evaluation node count limit
    state.nodes_evaluated += 1;
    if state.nodes_evaluated > limits.max_node_count {
        return Err(ResolveError::MaxNodeCountExceeded {
            limit: limits.max_node_count,
        });
    }

    // 2. Check memoization cache (skips re-evaluation for shared sub-graphs)
    if let Some(cached) = state.memo_cache.get(addr).cloned() {
        write_bytes_with_limit(&cached, limits, state, writer)?;
        return Ok(());
    }

    // 3. Check recursion depth limit
    if state.active_path.len() >= limits.max_depth {
        return Err(ResolveError::MaxDepthExceeded {
            limit: limits.max_depth,
            current: state.active_path.len(),
        });
    }

    // 4. Cycle Detection
    if state.active_path.contains(addr) {
        let mut path_str = state
            .active_path
            .iter()
            .map(|a| a.to_uri())
            .collect::<Vec<_>>()
            .join(" -> ");
        path_str.push_str(&format!(" -> {}", addr.to_uri()));
        return Err(ResolveError::CycleDetected { path: path_str });
    }

    state.active_path.push(addr.clone());

    // 5. Retrieve node (from store or self-describing address)
    let node = match store.get(addr) {
        Ok(Some(n)) => n,
        _ => match Node::from_address(addr) {
            Ok(n) => n,
            Err(_) => return Err(ResolveError::NodeNotFound(addr.to_uri())),
        },
    };

    // 6. Evaluate node
    match node {
        Node::Data(bytes) => {
            if bytes.len() <= MAX_MEMOIZE_NODE_SIZE {
                state.memo_cache.insert(addr.clone(), bytes.clone());
            }
            write_bytes_with_limit(&bytes, limits, state, writer)?;
        }
        Node::Page(page) => {
            let data = page.data();
            if data.len() <= MAX_MEMOIZE_NODE_SIZE {
                state.memo_cache.insert(addr.clone(), data.to_vec());
            }
            write_bytes_with_limit(data, limits, state, writer)?;
        }
        Node::Reference(target) => {
            resolve_internal(&target, store, limits, state, writer)?;
        }
        Node::Sequence(children) => {
            for child in children {
                resolve_internal(&child, store, limits, state, writer)?;
            }
        }
        Node::Repeat { target, count } => {
            if count == 0 {
                state.active_path.pop();
                return Ok(());
            }
            if let Some(cached) = state.memo_cache.get(&target) {
                let cached_bytes = cached.clone();
                for _ in 0..count {
                    write_bytes_with_limit(&cached_bytes, limits, state, writer)?;
                }
            } else if count > 1 {
                let mut target_buf = Vec::new();
                let dyn_target_writer: &mut dyn Write = &mut target_buf;
                resolve_internal(&target, store, limits, state, dyn_target_writer)?;
                if target_buf.len() <= MAX_MEMOIZE_NODE_SIZE {
                    state.memo_cache.insert(target.clone(), target_buf.clone());
                }
                writer.write_all(&target_buf)?;
                for _ in 0..(count - 1) {
                    write_bytes_with_limit(&target_buf, limits, state, writer)?;
                }
            } else {
                resolve_internal(&target, store, limits, state, writer)?;
            }
        }
        Node::Slice {
            target,
            offset,
            length,
        } => {
            let mut filter = SliceFilterWriter::new(writer, offset, length);
            resolve_internal(&target, store, limits, state, &mut filter)?;
        }
    }

    state.active_path.pop();
    Ok(())
}

fn write_bytes_with_limit(
    bytes: &[u8],
    limits: &Limits,
    state: &mut ResolverState,
    writer: &mut dyn Write,
) -> Result<(), ResolveError> {
    let new_total = state.bytes_written.saturating_add(bytes.len() as u64);
    if new_total > limits.max_output_bytes {
        return Err(ResolveError::MaxOutputBytesExceeded {
            limit: limits.max_output_bytes,
            current: new_total,
        });
    }
    writer.write_all(bytes)?;
    state.bytes_written = new_total;
    Ok(())
}

/// Streaming byte-range filter that passes only bytes within [target_offset..target_offset+target_len)
struct SliceFilterWriter<'a> {
    inner: &'a mut dyn Write,
    target_offset: u64,
    target_len: u64,
    stream_pos: u64,
}

impl<'a> SliceFilterWriter<'a> {
    fn new(inner: &'a mut dyn Write, target_offset: u64, target_len: u64) -> Self {
        Self {
            inner,
            target_offset,
            target_len,
            stream_pos: 0,
        }
    }
}

impl<'a> Write for SliceFilterWriter<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let chunk_start = self.stream_pos;
        let chunk_end = chunk_start + buf.len() as u64;
        self.stream_pos = chunk_end;

        let target_end = self.target_offset + self.target_len;

        // Calculate overlap between [chunk_start, chunk_end) and [target_offset, target_end)
        let overlap_start = chunk_start.max(self.target_offset);
        let overlap_end = chunk_end.min(target_end);

        if overlap_start < overlap_end {
            let start_idx = (overlap_start - chunk_start) as usize;
            let end_idx = (overlap_end - chunk_start) as usize;
            self.inner.write_all(&buf[start_idx..end_idx])?;
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::address::NodeType;
    use crate::page::Page;

    #[test]
    fn test_resolve_data_node() {
        let store = MemoryStore::new();
        let limits = Limits::default();
        let node = Node::Data(b"Hello Resolver".to_vec());
        let addr = node.to_address().unwrap();

        let resolved = resolve(&addr, &store, &limits).unwrap();
        assert_eq!(resolved, b"Hello Resolver");
    }

    #[test]
    fn test_resolve_sequence_and_repeat() {
        let store = MemoryStore::new();
        let limits = Limits::default();

        let part1 = Node::Data(b"AB".to_vec()).to_address().unwrap();
        let rep = Node::Repeat {
            target: part1.clone(),
            count: 3,
        }
        .to_address()
        .unwrap();

        let seq = Node::Sequence(vec![part1, rep]).to_address().unwrap();
        let resolved = resolve(&seq, &store, &limits).unwrap();
        assert_eq!(resolved, b"ABABABAB");
    }

    #[test]
    fn test_resolve_slice() {
        let store = MemoryStore::new();
        let limits = Limits::default();

        let base = Node::Data(b"0123456789".to_vec()).to_address().unwrap();
        let slice = Node::Slice {
            target: base,
            offset: 3,
            length: 4,
        }
        .to_address()
        .unwrap();

        let resolved = resolve(&slice, &store, &limits).unwrap();
        assert_eq!(resolved, b"3456");
    }

    #[test]
    fn test_resolve_page() {
        let store = MemoryStore::new();
        let limits = Limits::default();

        let page = Page::new(vec![0xAA; 4096]).unwrap();
        let addr = page.to_address(0);

        let resolved = resolve(&addr, &store, &limits).unwrap();
        assert_eq!(resolved.len(), 4096);
        assert_eq!(resolved[0], 0xAA);
    }

    #[test]
    fn test_cycle_detection() {
        let mut store = MemoryStore::new();
        let limits = Limits::default();

        let addr1 = Address::new(NodeType::Ref, vec![1]).unwrap();
        let addr2 = Address::new(NodeType::Ref, vec![2]).unwrap();

        // Create cycle: addr1 -> addr2 -> addr1
        store.put(&addr1, Node::Reference(addr2.clone())).unwrap();
        store.put(&addr2, Node::Reference(addr1.clone())).unwrap();

        let err = resolve(&addr1, &store, &limits).unwrap_err();
        assert!(matches!(err, ResolveError::CycleDetected { .. }));
    }

    #[test]
    fn test_depth_limit_exceeded() {
        let mut store = MemoryStore::new();
        let limits = Limits {
            max_depth: 3,
            max_output_bytes: 1000,
            max_node_count: 100,
        };

        let a1 = Address::new(NodeType::Ref, vec![1]).unwrap();
        let a2 = Address::new(NodeType::Ref, vec![2]).unwrap();
        let a3 = Address::new(NodeType::Ref, vec![3]).unwrap();
        let a4 = Address::new(NodeType::Ref, vec![4]).unwrap();

        store.put(&a1, Node::Reference(a2.clone())).unwrap();
        store.put(&a2, Node::Reference(a3.clone())).unwrap();
        store.put(&a3, Node::Reference(a4.clone())).unwrap();
        store.put(&a4, Node::Data(b"End".to_vec())).unwrap();

        let err = resolve(&a1, &store, &limits).unwrap_err();
        assert!(matches!(err, ResolveError::MaxDepthExceeded { .. }));
    }

    #[test]
    fn test_output_limit_exceeded() {
        let store = MemoryStore::new();
        let limits = Limits {
            max_depth: 10,
            max_output_bytes: 5,
            max_node_count: 100,
        };

        let data = Node::Data(b"1234567890".to_vec()).to_address().unwrap();
        let err = resolve(&data, &store, &limits).unwrap_err();
        assert!(matches!(err, ResolveError::MaxOutputBytesExceeded { .. }));
    }

    #[test]
    fn test_resolve_memoization_diamond() {
        let mut store = MemoryStore::new();
        let limits = Limits::default();

        // Shared leaf: C
        let node_c = Node::Data(b"SharedData".to_vec());
        let addr_c = node_c.to_address().unwrap();
        store.put(&addr_c, node_c).unwrap();

        // Branch A -> C
        let node_a = Node::Sequence(vec![addr_c.clone()]);
        let addr_a = node_a.to_address().unwrap();
        store.put(&addr_a, node_a).unwrap();

        // Branch B -> C
        let node_b = Node::Sequence(vec![addr_c.clone()]);
        let addr_b = node_b.to_address().unwrap();
        store.put(&addr_b, node_b).unwrap();

        // Root -> [A, B] (Diamond)
        let root = Node::Sequence(vec![addr_a, addr_b]).to_address().unwrap();

        let resolved = resolve(&root, &store, &limits).unwrap();
        assert_eq!(resolved, b"SharedDataSharedData");
    }

    #[test]
    fn test_resolve_repeat_optimization() {
        let mut store = MemoryStore::new();
        let limits = Limits::default();

        let node = Node::Data(b"XYZ".to_vec());
        let addr = node.to_address().unwrap();
        store.put(&addr, node).unwrap();

        let rep = Node::Repeat {
            target: addr,
            count: 5,
        }
        .to_address()
        .unwrap();

        let resolved = resolve(&rep, &store, &limits).unwrap();
        assert_eq!(resolved, b"XYZXYZXYZXYZXYZ");
    }
}
