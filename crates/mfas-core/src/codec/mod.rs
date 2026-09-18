//! Canonical Encoder, Decoder & Verification Engine
//!
//! Provides deterministic streaming encoding of arbitrary finite byte streams into
//! canonical MFAS page DAGs, and streaming reconstruction with SHA-256 integrity verification.

use crate::address::Address;
use crate::error::CoreError;
use crate::node::Node;
use crate::page::{Page, PAGE_SIZE};
use crate::resolver::{resolve, resolve_to_writer, AddressStore, Limits};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

/// Report produced by comprehensive file verification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationReport {
    pub file_path: String,
    pub root_address: Address,
    pub total_bytes: u64,
    pub sha256: String,
    pub verified: bool,
}

/// Encode an arbitrary byte reader into a canonical root Address
pub fn encode<R: Read>(reader: &mut R, store: &mut dyn AddressStore) -> Result<Address, CoreError> {
    let mut pages: Vec<Page> = Vec::new();
    let mut buf = vec![0u8; PAGE_SIZE];

    loop {
        let mut bytes_in_page = 0;
        while bytes_in_page < PAGE_SIZE {
            let n = reader.read(&mut buf[bytes_in_page..])?;
            if n == 0 {
                break;
            }
            bytes_in_page += n;
        }

        if bytes_in_page == 0 {
            break;
        }

        let page = Page::new(buf[..bytes_in_page].to_vec())?;
        pages.push(page);
    }

    // 1. Empty input
    if pages.is_empty() {
        let empty_node = Node::Data(Vec::new());
        let root = empty_node.to_address()?;
        store.put(&root, empty_node)?;
        return Ok(root);
    }

    // 2. Single page input (<= 4096 bytes)
    if pages.len() == 1 {
        let page = pages.remove(0);
        let addr = page.to_address(0);
        store.put(&addr, Node::Page(page))?;
        return Ok(addr);
    }

    // 3. Multi-page input: convert each page to its address and store it
    let mut page_addrs: Vec<Address> = Vec::with_capacity(pages.len());
    for (i, page) in pages.into_iter().enumerate() {
        let addr = page.to_hash_address(i as u64);
        store.put(&addr, Node::Page(page))?;
        page_addrs.push(addr);
    }

    // 4. Run-length grouping of identical consecutive page addresses
    let mut grouped_addrs: Vec<Address> = Vec::new();
    let mut i = 0;
    while i < page_addrs.len() {
        let current = &page_addrs[i];
        let mut count = 1u64;
        while i + 1 < page_addrs.len() && page_addrs[i + 1] == *current {
            count += 1;
            i += 1;
        }

        if count > 1 {
            let rep_node = Node::Repeat {
                target: current.clone(),
                count,
            };
            let rep_addr = rep_node.to_address()?;
            store.put(&rep_addr, rep_node)?;
            grouped_addrs.push(rep_addr);
        } else {
            grouped_addrs.push(current.clone());
        }
        i += 1;
    }

    // 5. Create root sequence
    let seq_node = Node::Sequence(grouped_addrs);
    let root_addr = seq_node.to_address()?;
    store.put(&root_addr, seq_node)?;
    Ok(root_addr)
}

/// Decode an Address directly into memory
pub fn decode(addr: &Address, store: &dyn AddressStore) -> Result<Vec<u8>, CoreError> {
    let limits = Limits::default();
    let bytes = resolve(addr, store, &limits)?;
    Ok(bytes)
}

/// Decode an Address streaming directly into a writer
pub fn decode_to_writer<W: Write>(
    addr: &Address,
    store: &dyn AddressStore,
    writer: &mut W,
) -> Result<u64, CoreError> {
    let limits = Limits::default();
    let count = resolve_to_writer(addr, store, &limits, writer)?;
    Ok(count)
}

/// Decode an Address streaming directly to an output file
pub fn decode_to_file<P: AsRef<Path>>(
    addr: &Address,
    store: &dyn AddressStore,
    path: P,
) -> Result<u64, CoreError> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    let count = decode_to_writer(addr, store, &mut writer)?;
    writer.flush()?;
    Ok(count)
}

/// Hasher sink that computes SHA-256 and counts total bytes
struct HashWriter<W: Write> {
    hasher: Sha256,
    byte_count: u64,
    inner: Option<W>,
}

impl<W: Write> HashWriter<W> {
    fn new(inner: Option<W>) -> Self {
        Self {
            hasher: Sha256::new(),
            byte_count: 0,
            inner,
        }
    }

    fn finalize(self) -> (String, u64) {
        let hash = self.hasher.finalize();
        (hex::encode(hash), self.byte_count)
    }
}

impl<W: Write> Write for HashWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.hasher.update(buf);
        self.byte_count += buf.len() as u64;
        if let Some(ref mut w) = self.inner {
            w.write_all(buf)?;
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if let Some(ref mut w) = self.inner {
            w.flush()?;
        }
        Ok(())
    }
}

/// Calculate the SHA-256 hash and byte length of a file
pub fn hash_file<P: AsRef<Path>>(path: P) -> std::io::Result<(String, u64)> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut total_bytes = 0u64;
    let mut buf = [0u8; 8192];

    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
        total_bytes += n as u64;
    }

    let hash_hex = hex::encode(hasher.finalize());
    Ok((hash_hex, total_bytes))
}

/// Verify a file by encoding it, decoding it to a streaming hasher, and verifying equality
pub fn verify_file<P: AsRef<Path>>(
    path: P,
    store: &mut dyn AddressStore,
) -> Result<VerificationReport, CoreError> {
    let p = path.as_ref();
    let (expected_hash, expected_len) = hash_file(p)?;

    // 1. Encode file
    let file = File::open(p)?;
    let mut reader = BufReader::new(file);
    let root_addr = encode(&mut reader, store)?;

    // 2. Decode streaming to hash verifier
    let mut verifier = HashWriter::<Vec<u8>>::new(None);
    let count = decode_to_writer(&root_addr, store, &mut verifier)?;
    let (reconstructed_hash, _) = verifier.finalize();

    let verified = count == expected_len && reconstructed_hash == expected_hash;

    Ok(VerificationReport {
        file_path: p.display().to_string(),
        root_address: root_addr,
        total_bytes: count,
        sha256: reconstructed_hash,
        verified,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resolver::MemoryStore;

    #[test]
    fn test_codec_empty_stream() {
        let mut store = MemoryStore::new();
        let mut empty = std::io::Cursor::new(b"");
        let root = encode(&mut empty, &mut store).unwrap();

        assert_eq!(root.to_uri(), "mfas:v1:data:");
        let decoded = decode(&root, &store).unwrap();
        assert_eq!(decoded, b"");
    }

    #[test]
    fn test_codec_single_page() {
        let mut store = MemoryStore::new();
        let data = b"Hello Single Page MFAS Encoding!";
        let mut cursor = std::io::Cursor::new(data);
        let root = encode(&mut cursor, &mut store).unwrap();

        let decoded = decode(&root, &store).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_codec_multi_page_and_partial() {
        let mut store = MemoryStore::new();
        // 10,000 bytes = 2 full pages (8192 B) + 1 partial page (1808 B)
        let mut data = Vec::with_capacity(10_000);
        for i in 0..10_000 {
            data.push((i % 256) as u8);
        }

        let mut cursor = std::io::Cursor::new(&data);
        let root = encode(&mut cursor, &mut store).unwrap();
        assert_eq!(root.node_type(), crate::address::NodeType::Seq);

        let decoded = decode(&root, &store).unwrap();
        assert_eq!(decoded.len(), 10_000);
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_codec_repetitive_data_compression() {
        let mut store = MemoryStore::new();
        // 5 identical pages of 4096 bytes
        let single_page = vec![0x42; 4096];
        let mut data = Vec::new();
        for _ in 0..5 {
            data.extend_from_slice(&single_page);
        }

        let mut cursor = std::io::Cursor::new(&data);
        let root = encode(&mut cursor, &mut store).unwrap();

        let decoded = decode(&root, &store).unwrap();
        assert_eq!(decoded.len(), 20_480);
        assert_eq!(decoded, data);
    }
}
