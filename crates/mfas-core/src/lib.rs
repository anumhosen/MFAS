//! MFAS Core - Mathematical File Address Space
//!
//! Core mathematical models, address spaces, pages, nodes, and reversibility invariants.

pub mod address;
pub mod enumeration;
pub mod error;

pub use address::{Address, NodeType, CURRENT_VERSION, SCHEME_PREFIX};
pub use enumeration::{bytes_to_number, length_offset, number_to_bytes};
pub use error::{AddressError, CoreError};
