//! MFAS Core - Mathematical File Address Space
//!
//! Core mathematical models, address spaces, pages, nodes, and reversibility invariants.

pub mod address;
pub mod codec;
pub mod enumeration;
pub mod error;
pub mod inspector;
pub mod node;
pub mod page;
pub mod resolver;
pub mod synthetic;

pub use address::{Address, NodeType, CURRENT_VERSION, SCHEME_PREFIX};
pub use codec::{
    decode, decode_to_file, decode_to_writer, encode, hash_file, verify_file, VerificationReport,
};
pub use enumeration::{bytes_to_number, length_offset, number_to_bytes};
pub use error::{AddressError, CoreError, NodeError, PageError, ResolveError};
pub use inspector::{inspect_address, InspectionReport};
pub use node::Node;
pub use page::{
    split_into_pages, Page, RadixCoordinate, BYTES_PER_FLOOR, BYTES_PER_PAGE, BYTES_PER_ROOM,
    BYTES_PER_SHELF, BYTES_PER_VOLUME, BYTES_PER_WALL, PAGE_SIZE,
};
pub use resolver::{resolve, resolve_to_writer, AddressStore, FileStore, Limits, MemoryStore};
pub use synthetic::SyntheticStream;
