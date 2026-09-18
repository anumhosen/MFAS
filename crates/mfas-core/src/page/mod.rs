//! Logical Page Model & Radix-16 Hierarchy
//!
//! Provides the primary $16^3 = 4096$-byte logical page abstraction,
//! partial final page handling, and base-16 spatial coordinate calculations.

use crate::address::{Address, NodeType};
use crate::error::PageError;
use serde::{Deserialize, Serialize};

/// Canonical logical page size in bytes ($16^3 = 4096$)
pub const PAGE_SIZE: usize = 4096;

/// Radix-16 hierarchy scale constants
pub const BYTES_PER_PAGE: u64 = 4096; // 16^3
pub const BYTES_PER_VOLUME: u64 = 65_536; // 16^4 = 16 pages
pub const BYTES_PER_SHELF: u64 = 1_048_576; // 16^5 = 1 MiB
pub const BYTES_PER_WALL: u64 = 16_777_216; // 16^6 = 16 MiB
pub const BYTES_PER_ROOM: u64 = 268_435_456; // 16^7 = 256 MiB
pub const BYTES_PER_FLOOR: u64 = 4_294_967_296; // 16^8 = 4 GiB

/// Radix-16 spatial hierarchy coordinates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RadixCoordinate {
    pub floor: u64,
    pub room: u8,
    pub wall: u8,
    pub shelf: u8,
    pub volume: u8,
    pub page: u8,
    pub offset_in_page: u16,
}

impl RadixCoordinate {
    /// Compute the exact radix-16 spatial coordinate for any byte offset
    pub fn from_byte_offset(offset: u64) -> Self {
        let offset_in_page = (offset & 0x0FFF) as u16;
        let page_index = offset >> 12;

        let page = (page_index & 0x0F) as u8;
        let volume = ((page_index >> 4) & 0x0F) as u8;
        let shelf = ((page_index >> 8) & 0x0F) as u8;
        let wall = ((page_index >> 12) & 0x0F) as u8;
        let room = ((page_index >> 16) & 0x0F) as u8;
        let floor = page_index >> 20;

        Self {
            floor,
            room,
            wall,
            shelf,
            volume,
            page,
            offset_in_page,
        }
    }

    /// Convert the coordinate back into its exact absolute byte offset
    pub fn to_byte_offset(&self) -> u64 {
        (self.floor * BYTES_PER_FLOOR)
            + (self.room as u64 * BYTES_PER_ROOM)
            + (self.wall as u64 * BYTES_PER_WALL)
            + (self.shelf as u64 * BYTES_PER_SHELF)
            + (self.volume as u64 * BYTES_PER_VOLUME)
            + (self.page as u64 * BYTES_PER_PAGE)
            + (self.offset_in_page as u64)
    }

    /// Format as canonical hierarchical coordinate notation:
    /// `Floor[f].Room[r].Wall[w].Shelf[s].Volume[v].Page[p]+Offset[0x...]`
    pub fn format_coordinate(&self) -> String {
        format!(
            "Floor[{}].Room[{:X}].Wall[{:X}].Shelf[{:X}].Volume[{:X}].Page[{:X}]+Offset[0x{:03X}]",
            self.floor,
            self.room,
            self.wall,
            self.shelf,
            self.volume,
            self.page,
            self.offset_in_page
        )
    }
}

/// Logical page containing between 1 and 4096 bytes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Page {
    data: Vec<u8>,
}

impl Page {
    /// Create a new logical page from bytes ($1 \le \text{len} \le 4096$)
    pub fn new(data: Vec<u8>) -> Result<Self, PageError> {
        if data.is_empty() {
            return Err(PageError::EmptyPage);
        }
        if data.len() > PAGE_SIZE {
            return Err(PageError::PageSizeExceeded(data.len()));
        }
        Ok(Self { data })
    }

    /// Logical length in bytes ($1 \le \text{len} \le 4096$)
    pub fn logical_len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if this page is a full 4096-byte page
    pub fn is_full(&self) -> bool {
        self.data.len() == PAGE_SIZE
    }

    /// Reference to underlying page bytes
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Canonical address for this page:
    /// Payload layout: `[page_index: u64 BE (8B)][logical_len: u16 BE (2B)][content_bytes]`
    pub fn to_address(&self, page_index: u64) -> Address {
        let mut payload = Vec::with_capacity(10 + self.data.len());
        payload.extend_from_slice(&page_index.to_be_bytes());
        payload.extend_from_slice(&(self.data.len() as u16).to_be_bytes());
        payload.extend_from_slice(&self.data);
        Address::new(NodeType::Page, payload).expect("Page address creation is valid")
    }

    /// Decode a page and its page index from a canonical page Address
    pub fn from_address(addr: &Address) -> Result<(u64, Self), PageError> {
        if addr.node_type() != NodeType::Page {
            return Err(PageError::NotAPageAddress);
        }
        let payload = addr.payload();
        if payload.len() < 10 {
            return Err(PageError::InvalidPayloadLength(payload.len()));
        }

        let mut index_bytes = [0u8; 8];
        index_bytes.copy_from_slice(&payload[0..8]);
        let page_index = u64::from_be_bytes(index_bytes);

        let mut len_bytes = [0u8; 2];
        len_bytes.copy_from_slice(&payload[8..10]);
        let expected_len = u16::from_be_bytes(len_bytes) as usize;

        let content = payload[10..].to_vec();
        if content.len() != expected_len {
            return Err(PageError::LogicalLengthMismatch {
                expected: expected_len,
                actual: content.len(),
            });
        }

        let page = Page::new(content)?;
        Ok((page_index, page))
    }
}

/// Split an arbitrary byte stream into consecutive logical pages,
/// properly preserving the partial final page length if not aligned to 4096.
pub fn split_into_pages(bytes: &[u8]) -> Vec<Page> {
    if bytes.is_empty() {
        return Vec::new();
    }
    bytes
        .chunks(PAGE_SIZE)
        .map(|chunk| Page::new(chunk.to_vec()).expect("Chunk is <= PAGE_SIZE"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radix_coordinates_roundtrip() {
        let offsets: Vec<u64> = vec![
            0,
            1,
            15,
            16,
            255,
            256,
            4095,
            4096,
            4097,
            65535,
            65536,
            1_048_575,
            1_048_576,
            16_777_215,
            16_777_216,
            268_435_455,
            268_435_456,
            4_294_967_295,
            4_294_967_296,
            10_000_000_000,
        ];

        for offset in offsets {
            let coord = RadixCoordinate::from_byte_offset(offset);
            let restored = coord.to_byte_offset();
            assert_eq!(restored, offset, "Failed for offset {}", offset);
        }
    }

    #[test]
    fn test_radix_hierarchy_steps() {
        // Page 0 boundary
        let c0 = RadixCoordinate::from_byte_offset(0);
        assert_eq!(c0.floor, 0);
        assert_eq!(c0.room, 0);
        assert_eq!(c0.wall, 0);
        assert_eq!(c0.shelf, 0);
        assert_eq!(c0.volume, 0);
        assert_eq!(c0.page, 0);
        assert_eq!(c0.offset_in_page, 0);

        // Exactly 1 page = 4096 bytes -> Page 1
        let c_page = RadixCoordinate::from_byte_offset(4096);
        assert_eq!(c_page.volume, 0);
        assert_eq!(c_page.page, 1);
        assert_eq!(c_page.offset_in_page, 0);

        // Exactly 1 volume = 16 pages = 65536 bytes -> Volume 1, Page 0
        let c_vol = RadixCoordinate::from_byte_offset(65536);
        assert_eq!(c_vol.volume, 1);
        assert_eq!(c_vol.page, 0);

        // Exactly 1 shelf = 1 MiB -> Shelf 1
        let c_shelf = RadixCoordinate::from_byte_offset(1_048_576);
        assert_eq!(c_shelf.shelf, 1);
        assert_eq!(c_shelf.volume, 0);

        // Exactly 1 wall = 16 MiB -> Wall 1
        let c_wall = RadixCoordinate::from_byte_offset(16_777_216);
        assert_eq!(c_wall.wall, 1);

        // Exactly 1 room = 256 MiB -> Room 1
        let c_room = RadixCoordinate::from_byte_offset(268_435_456);
        assert_eq!(c_room.room, 1);

        // Exactly 1 floor = 4 GiB -> Floor 1
        let c_floor = RadixCoordinate::from_byte_offset(4_294_967_296);
        assert_eq!(c_floor.floor, 1);
        assert_eq!(c_floor.room, 0);
    }

    #[test]
    fn test_split_into_pages_exact_and_partial() {
        // Exactly 4096 bytes
        let data_exact = vec![0x42; 4096];
        let pages = split_into_pages(&data_exact);
        assert_eq!(pages.len(), 1);
        assert_eq!(pages[0].logical_len(), 4096);
        assert!(pages[0].is_full());

        // 4097 bytes -> 1 full page + 1 partial page of 1 byte
        let data_extra = vec![0x42; 4097];
        let pages2 = split_into_pages(&data_extra);
        assert_eq!(pages2.len(), 2);
        assert_eq!(pages2[0].logical_len(), 4096);
        assert!(pages2[0].is_full());
        assert_eq!(pages2[1].logical_len(), 1);
        assert!(!pages2[1].is_full());
    }

    #[test]
    fn test_page_address_roundtrip() {
        let page_data = vec![1, 2, 3, 4, 5];
        let page = Page::new(page_data.clone()).unwrap();
        let addr = page.to_address(42);

        assert_eq!(addr.node_type(), NodeType::Page);
        let (index, restored_page) = Page::from_address(&addr).unwrap();
        assert_eq!(index, 42);
        assert_eq!(restored_page.data(), page_data.as_slice());
        assert_eq!(restored_page.logical_len(), 5);
    }
}
