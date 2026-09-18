//! Recursive Node Model
//!
//! Provides the strongly typed recursive node system:
//! - Data: literal raw bytes
//! - Reference: pointer to another address
//! - Sequence: ordered list of child addresses
//! - Repeat: repeated evaluation of a child address
//! - Slice: byte-range slice of a child address
//! - Page: canonical 4096-byte logical page

use crate::address::{Address, NodeType};
use crate::error::NodeError;
use crate::page::Page;
use serde::{Deserialize, Serialize};

/// Fundamental recursive node primitives
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Node {
    /// Literal raw byte content
    Data(Vec<u8>),
    /// Reference pointing to another canonical address
    Reference(Address),
    /// Ordered concatenation of child addresses
    Sequence(Vec<Address>),
    /// Repetition of a child address `count` times
    Repeat {
        target: Address,
        count: u64,
    },
    /// Sliced range of a child address
    Slice {
        target: Address,
        offset: u64,
        length: u64,
    },
    /// Canonical logical page
    Page(Page),
}

impl Node {
    /// Associated node type
    pub fn node_type(&self) -> NodeType {
        match self {
            Node::Data(_) => NodeType::Data,
            Node::Reference(_) => NodeType::Ref,
            Node::Sequence(_) => NodeType::Seq,
            Node::Repeat { .. } => NodeType::Rep,
            Node::Slice { .. } => NodeType::Slice,
            Node::Page(_) => NodeType::Page,
        }
    }

    /// Extract all immediate child addresses referenced by this node
    pub fn child_addresses(&self) -> Vec<&Address> {
        match self {
            Node::Data(_) => Vec::new(),
            Node::Reference(addr) => vec![addr],
            Node::Sequence(addrs) => addrs.iter().collect(),
            Node::Repeat { target, .. } => vec![target],
            Node::Slice { target, .. } => vec![target],
            Node::Page(_) => Vec::new(),
        }
    }

    /// Convert node into its canonical Address representation
    pub fn to_address(&self) -> Result<Address, NodeError> {
        match self {
            Node::Data(bytes) => {
                Ok(Address::new(NodeType::Data, bytes.clone())?)
            }
            Node::Reference(addr) => {
                let bin = addr.to_binary();
                Ok(Address::new(NodeType::Ref, bin)?)
            }
            Node::Sequence(children) => {
                if children.is_empty() {
                    return Err(NodeError::EmptySequence);
                }
                let mut payload = Vec::new();
                payload.extend_from_slice(&(children.len() as u32).to_be_bytes());
                for child in children {
                    let child_bin = child.to_binary();
                    payload.extend_from_slice(&(child_bin.len() as u32).to_be_bytes());
                    payload.extend_from_slice(&child_bin);
                }
                Ok(Address::new(NodeType::Seq, payload)?)
            }
            Node::Repeat { target, count } => {
                if *count == 0 {
                    return Err(NodeError::ZeroRepeatCount);
                }
                let mut payload = Vec::new();
                payload.extend_from_slice(&count.to_be_bytes());
                payload.extend_from_slice(&target.to_binary());
                Ok(Address::new(NodeType::Rep, payload)?)
            }
            Node::Slice {
                target,
                offset,
                length,
            } => {
                if *length == 0 {
                    return Err(NodeError::InvalidSliceRange {
                        offset: *offset,
                        length: *length,
                    });
                }
                let mut payload = Vec::new();
                payload.extend_from_slice(&offset.to_be_bytes());
                payload.extend_from_slice(&length.to_be_bytes());
                payload.extend_from_slice(&target.to_binary());
                Ok(Address::new(NodeType::Slice, payload)?)
            }
            Node::Page(page) => Ok(page.to_address(0)),
        }
    }

    /// Reconstruct a node from its canonical Address
    pub fn from_address(addr: &Address) -> Result<Self, NodeError> {
        let payload = addr.payload();
        match addr.node_type() {
            NodeType::Data => Ok(Node::Data(payload.to_vec())),
            NodeType::Ref => {
                let target = Address::from_binary(payload)?;
                Ok(Node::Reference(target))
            }
            NodeType::Seq => {
                if payload.len() < 4 {
                    return Err(NodeError::InvalidPayload("Truncated sequence header".into()));
                }
                let count = u32::from_be_bytes([payload[0], payload[1], payload[2], payload[3]]) as usize;
                if count == 0 {
                    return Err(NodeError::EmptySequence);
                }
                let mut offset = 4;
                let mut children = Vec::with_capacity(count);
                for _ in 0..count {
                    if offset + 4 > payload.len() {
                        return Err(NodeError::InvalidPayload("Truncated child length".into()));
                    }
                    let child_len = u32::from_be_bytes([
                        payload[offset],
                        payload[offset + 1],
                        payload[offset + 2],
                        payload[offset + 3],
                    ]) as usize;
                    offset += 4;
                    if offset + child_len > payload.len() {
                        return Err(NodeError::InvalidPayload("Truncated child binary".into()));
                    }
                    let child_bin = &payload[offset..offset + child_len];
                    let child = Address::from_binary(child_bin)?;
                    children.push(child);
                    offset += child_len;
                }
                Ok(Node::Sequence(children))
            }
            NodeType::Rep => {
                if payload.len() < 8 {
                    return Err(NodeError::InvalidPayload("Truncated repeat count".into()));
                }
                let count = u64::from_be_bytes([
                    payload[0], payload[1], payload[2], payload[3], payload[4], payload[5], payload[6],
                    payload[7],
                ]);
                if count == 0 {
                    return Err(NodeError::ZeroRepeatCount);
                }
                let target_bin = &payload[8..];
                let target = Address::from_binary(target_bin)?;
                Ok(Node::Repeat { target, count })
            }
            NodeType::Slice => {
                if payload.len() < 16 {
                    return Err(NodeError::InvalidPayload("Truncated slice header".into()));
                }
                let offset = u64::from_be_bytes([
                    payload[0], payload[1], payload[2], payload[3], payload[4], payload[5], payload[6],
                    payload[7],
                ]);
                let length = u64::from_be_bytes([
                    payload[8], payload[9], payload[10], payload[11], payload[12], payload[13],
                    payload[14], payload[15],
                ]);
                if length == 0 {
                    return Err(NodeError::InvalidSliceRange { offset, length });
                }
                let target_bin = &payload[16..];
                let target = Address::from_binary(target_bin)?;
                Ok(Node::Slice {
                    target,
                    offset,
                    length,
                })
            }
            NodeType::Page => {
                let (_, page) = Page::from_address(addr)?;
                Ok(Node::Page(page))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_node_roundtrip() {
        let node = Node::Data(b"Inline Literal Bytes".to_vec());
        let addr = node.to_address().unwrap();
        assert_eq!(addr.node_type(), NodeType::Data);
        let restored = Node::from_address(&addr).unwrap();
        assert_eq!(restored, node);
        assert!(node.child_addresses().is_empty());
    }

    #[test]
    fn test_reference_node_roundtrip() {
        let target = Address::new(NodeType::Data, vec![0xCA, 0xFE]).unwrap();
        let node = Node::Reference(target.clone());
        let addr = node.to_address().unwrap();
        assert_eq!(addr.node_type(), NodeType::Ref);
        let restored = Node::from_address(&addr).unwrap();
        assert_eq!(restored, node);
        assert_eq!(node.child_addresses(), vec![&target]);
    }

    #[test]
    fn test_sequence_node_roundtrip() {
        let child1 = Address::new(NodeType::Data, vec![1, 2]).unwrap();
        let child2 = Address::new(NodeType::Data, vec![3, 4]).unwrap();
        let node = Node::Sequence(vec![child1.clone(), child2.clone()]);
        let addr = node.to_address().unwrap();
        assert_eq!(addr.node_type(), NodeType::Seq);
        let restored = Node::from_address(&addr).unwrap();
        assert_eq!(restored, node);
        assert_eq!(node.child_addresses(), vec![&child1, &child2]);
    }

    #[test]
    fn test_repeat_node_roundtrip() {
        let target = Address::new(NodeType::Data, vec![0xFF]).unwrap();
        let node = Node::Repeat {
            target: target.clone(),
            count: 1024,
        };
        let addr = node.to_address().unwrap();
        assert_eq!(addr.node_type(), NodeType::Rep);
        let restored = Node::from_address(&addr).unwrap();
        assert_eq!(restored, node);
        assert_eq!(node.child_addresses(), vec![&target]);
    }

    #[test]
    fn test_slice_node_roundtrip() {
        let target = Address::new(NodeType::Data, vec![0, 1, 2, 3, 4, 5]).unwrap();
        let node = Node::Slice {
            target: target.clone(),
            offset: 2,
            length: 3,
        };
        let addr = node.to_address().unwrap();
        assert_eq!(addr.node_type(), NodeType::Slice);
        let restored = Node::from_address(&addr).unwrap();
        assert_eq!(restored, node);
        assert_eq!(node.child_addresses(), vec![&target]);
    }

    #[test]
    fn test_page_node_roundtrip() {
        let page = Page::new(vec![42; 100]).unwrap();
        let node = Node::Page(page);
        let addr = node.to_address().unwrap();
        assert_eq!(addr.node_type(), NodeType::Page);
        let restored = Node::from_address(&addr).unwrap();
        assert_eq!(restored, node);
    }

    #[test]
    fn test_node_validation_errors() {
        // Empty sequence error
        let empty_seq = Node::Sequence(vec![]);
        assert!(matches!(empty_seq.to_address().unwrap_err(), NodeError::EmptySequence));

        // Zero repeat count error
        let target = Address::new(NodeType::Data, vec![1]).unwrap();
        let zero_rep = Node::Repeat {
            target: target.clone(),
            count: 0,
        };
        assert!(matches!(zero_rep.to_address().unwrap_err(), NodeError::ZeroRepeatCount));

        // Zero slice length error
        let zero_slice = Node::Slice {
            target,
            offset: 10,
            length: 0,
        };
        assert!(matches!(zero_slice.to_address().unwrap_err(), NodeError::InvalidSliceRange { .. }));
    }
}
