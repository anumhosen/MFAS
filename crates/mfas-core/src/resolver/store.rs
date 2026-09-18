use crate::address::Address;
use crate::error::CoreError;
use crate::node::Node;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

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

/// Persistent disk-based content-addressed storage implementation of AddressStore
#[derive(Debug, Clone)]
pub struct FileStore {
    root_dir: PathBuf,
}

impl FileStore {
    pub fn new<P: AsRef<Path>>(root_dir: P) -> std::io::Result<Self> {
        let p = root_dir.as_ref().to_path_buf();
        std::fs::create_dir_all(&p)?;
        Ok(Self { root_dir: p })
    }

    pub fn default_store() -> std::io::Result<Self> {
        Self::new(".mfas/objects")
    }

    fn path_for_address(&self, addr: &Address) -> PathBuf {
        use sha2::{Digest, Sha256};
        let hash = hex::encode(Sha256::digest(addr.to_binary()));
        let prefix = &hash[..2];
        let rest = &hash[2..];
        self.root_dir.join(prefix).join(format!("{}.json", rest))
    }
}

impl AddressStore for FileStore {
    fn get(&self, addr: &Address) -> Result<Option<Node>, CoreError> {
        let path = self.path_for_address(addr);
        if !path.exists() {
            return Ok(None);
        }
        let data = std::fs::read(&path)?;
        let node: Node = serde_json::from_slice(&data)
            .map_err(|e| CoreError::Io(e.to_string()))?;
        Ok(Some(node))
    }

    fn put(&mut self, addr: &Address, node: Node) -> Result<(), CoreError> {
        let path = self.path_for_address(addr);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let data = serde_json::to_vec(&node)
            .map_err(|e| CoreError::Io(e.to_string()))?;
        std::fs::write(&path, data)?;
        Ok(())
    }
}
