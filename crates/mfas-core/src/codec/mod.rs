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

/// Encode an arbitrary byte reader into a canonical root Address in constant memory
pub fn encode<R: Read>(reader: &mut R, store: &mut dyn AddressStore) -> Result<Address, CoreError> {
    let mut buf = vec![0u8; PAGE_SIZE];
    let mut page_count = 0u64;
    let mut first_page: Option<Page> = None;
    let mut grouped_addrs: Vec<Address> = Vec::new();
    let mut current_run: Option<(Address, u64)> = None;

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

        if page_count == 0 {
            // Buffer the first page to see if EOF is reached next (single-page optimization)
            first_page = Some(page);
            page_count = 1;
        } else {
            // If first_page is still pending, commit it as page index 0
            if let Some(fp) = first_page.take() {
                let first_addr = fp.to_hash_address(0);
                store.put(&first_addr, Node::Page(fp))?;
                current_run = Some((first_addr, 1));
            }

            let curr_addr = page.to_hash_address(0);
            store.put(&curr_addr, Node::Page(page))?;
            page_count += 1;

            // Update on-the-fly run-length grouping
            match current_run.take() {
                Some((prev_addr, count)) => {
                    if prev_addr == curr_addr {
                        current_run = Some((prev_addr, count + 1));
                    } else {
                        if count == 1 {
                            grouped_addrs.push(prev_addr);
                        } else {
                            let rep = Node::Repeat {
                                target: prev_addr,
                                count,
                            };
                            let rep_addr = rep.to_address()?;
                            store.put(&rep_addr, rep)?;
                            grouped_addrs.push(rep_addr);
                        }
                        current_run = Some((curr_addr, 1));
                    }
                }
                None => {
                    current_run = Some((curr_addr, 1));
                }
            }
        }
    }

    // 1. Empty input
    if page_count == 0 {
        let empty_node = Node::Data(Vec::new());
        let root = empty_node.to_address()?;
        store.put(&root, empty_node)?;
        return Ok(root);
    }

    // 2. Single page input (<= 4096 bytes)
    if let Some(fp) = first_page {
        let addr = fp.to_address(0);
        store.put(&addr, Node::Page(fp))?;
        return Ok(addr);
    }

    // 3. Multi-page: flush remaining run
    if let Some((prev_addr, count)) = current_run {
        if count == 1 {
            grouped_addrs.push(prev_addr);
        } else {
            let rep = Node::Repeat {
                target: prev_addr,
                count,
            };
            let rep_addr = rep.to_address()?;
            store.put(&rep_addr, rep)?;
            grouped_addrs.push(rep_addr);
        }
    }

    // 4. Create root sequence
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

/// Reader adapter that passes all reads through a SHA-256 hasher and counts total bytes
pub struct HashReader<'a, R: Read> {
    inner: &'a mut R,
    hasher: Sha256,
    byte_count: u64,
}

impl<'a, R: Read> HashReader<'a, R> {
    pub fn new(inner: &'a mut R) -> Self {
        Self {
            inner,
            hasher: Sha256::new(),
            byte_count: 0,
        }
    }

    pub fn finalize(self) -> (String, u64) {
        let hash = self.hasher.finalize();
        (hex::encode(hash), self.byte_count)
    }
}

impl<'a, R: Read> Read for HashReader<'a, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.inner.read(buf)?;
        if n > 0 {
            self.hasher.update(&buf[..n]);
            self.byte_count += n as u64;
        }
        Ok(n)
    }
}

/// Verify an arbitrary byte stream by encoding to an address, decoding to a streaming hasher,
/// and verifying exact SHA-256 and byte length match without intermediate files.
pub fn verify_stream<R: Read>(
    reader: &mut R,
    store: &mut dyn AddressStore,
) -> Result<VerificationReport, CoreError> {
    let mut hash_reader = HashReader::new(reader);
    let root_addr = encode(&mut hash_reader, store)?;
    let (expected_hash, total_bytes) = hash_reader.finalize();

    let mut verifier = HashWriter::<Vec<u8>>::new(None);
    let count = decode_to_writer(&root_addr, store, &mut verifier)?;
    let (reconstructed_hash, _) = verifier.finalize();

    let verified = count == total_bytes && reconstructed_hash == expected_hash;

    Ok(VerificationReport {
        file_path: "<stream>".to_string(),
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
