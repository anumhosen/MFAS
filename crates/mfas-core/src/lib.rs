//! MFAS Core - Mathematical File Address Space
//!
//! Core mathematical models, address spaces, pages, nodes, and reversibility invariants.

pub mod address;
pub mod enumeration;
pub mod error;
pub mod page;

pub use address::{Address, NodeType, CURRENT_VERSION, SCHEME_PREFIX};
pub use enumeration::{bytes_to_number, length_offset, number_to_bytes};
pub use error::{AddressError, CoreError, PageError};
pub use page::{
    split_into_pages, Page, RadixCoordinate, BYTES_PER_FLOOR, BYTES_PER_PAGE, BYTES_PER_ROOM,
    BYTES_PER_SHELF, BYTES_PER_VOLUME, BYTES_PER_WALL, PAGE_SIZE,
};
