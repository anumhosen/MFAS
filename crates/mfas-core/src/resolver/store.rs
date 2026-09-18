use crate::address::Address;
use crate::error::CoreError;
use crate::node::Node;
use std::collections::HashMap;

/// Storage interface for retrieving and persisting nodes by address
pub trait AddressStore {
    fn get(&self, addr: &Address) -> Result<Option<Node>, CoreError>;
    fn put(&mut self, addr: &Address, node: Node) -> Result<(), CoreError>;
}

/// In-memory hash map implementation of AddressStore
#[derive(Debug, Default, Clone)]
pub struct MemoryStore {
    nodes: HashMap<Address, Node>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

impl AddressStore for MemoryStore {
    fn get(&self, addr: &Address) -> Result<Option<Node>, CoreError> {
        Ok(self.nodes.get(addr).cloned())
    }

    fn put(&mut self, addr: &Address, node: Node) -> Result<(), CoreError> {
        self.nodes.insert(addr.clone(), node);
        Ok(())
    }
}
