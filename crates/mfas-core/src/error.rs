use thiserror::Error;

/// Core error types for the MFAS system
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum CoreError {
    #[error("Address error: {0}")]
    Address(#[from] AddressError),

    #[error("Page error: {0}")]
    Page(#[from] PageError),

    #[error("Node error: {0}")]
    Node(#[from] NodeError),

    #[error("Resolve error: {0}")]
    Resolve(#[from] ResolveError),
}

/// Errors occurring during Address parsing, formatting, or binary serialization
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum AddressError {
    #[error("Invalid scheme prefix: expected 'mfas', got '{0}'")]
    InvalidPrefix(String),

    #[error("Unsupported address version: expected 'v1', got '{0}'")]
    InvalidVersion(String),

    #[error("Unknown or invalid node type: '{0}'")]
    UnknownNodeType(String),

    #[error("Invalid hexadecimal encoding in payload: {0}")]
    InvalidHex(String),

    #[error("Hexadecimal payload length must be an even number of characters: got {0}")]
    OddLengthHex(usize),

    #[error("Address payload cannot be empty")]
    EmptyPayload,

    #[error("Malformed address URI: '{0}'")]
    MalformedUri(String),

    #[error("Invalid binary magic: expected 'MFAS' (0x4D464153), got {0:02X?}")]
    InvalidBinaryMagic([u8; 4]),

    #[error("Unsupported binary address version: {0}")]
    UnsupportedBinaryVersion(u8),

    #[error("Unknown binary type ID: 0x{0:02X}")]
    UnknownBinaryTypeId(u8),

    #[error("Malformed binary payload: unexpected end of stream or length mismatch")]
    MalformedBinaryPayload,
}

/// Errors occurring during Page creation, splitting, or reconstruction
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum PageError {
    #[error("Page size exceeds maximum 4096 bytes: got {0}")]
    PageSizeExceeded(usize),

    #[error("Page data cannot be empty")]
    EmptyPage,

    #[error("Target address is not a page node")]
    NotAPageAddress,

    #[error("Invalid page address payload length: expected at least 10 bytes, got {0}")]
    InvalidPayloadLength(usize),

    #[error("Logical length mismatch: header specifies {expected} bytes, but got {actual} bytes")]
    LogicalLengthMismatch { expected: usize, actual: usize },
}

/// Errors occurring during Node operations and deserialization
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum NodeError {
    #[error("Sequence node must contain at least one child address")]
    EmptySequence,

    #[error("Repeat count must be greater than zero")]
    ZeroRepeatCount,

    #[error("Slice length must be greater than zero: offset {offset}, length {length}")]
    InvalidSliceRange { offset: u64, length: u64 },

    #[error("Node payload corrupted or truncated: {0}")]
    InvalidPayload(String),

    #[error("Node type mismatch: expected {expected:?}, got {actual:?}")]
    TypeMismatch {
        expected: crate::address::NodeType,
        actual: crate::address::NodeType,
    },

    #[error("Address error: {0}")]
    Address(#[from] AddressError),

    #[error("Page error: {0}")]
    Page(#[from] PageError),
}

/// Errors occurring during recursive resolution and evaluation
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum ResolveError {
    #[error("Cycle detected in DAG address graph: {path}")]
    CycleDetected { path: String },

    #[error("Recursion depth limit exceeded: current {current} > limit {limit}")]
    MaxDepthExceeded { limit: usize, current: usize },

    #[error("Output bytes limit exceeded: total {current} > limit {limit}")]
    MaxOutputBytesExceeded { limit: u64, current: u64 },

    #[error("Node evaluation count limit exceeded: limit {limit}")]
    MaxNodeCountExceeded { limit: usize },

    #[error("Node not found in storage or address not self-describing: {0}")]
    NodeNotFound(String),

    #[error("I/O error during streaming resolution: {0}")]
    IoError(String),

    #[error("Address error: {0}")]
    Address(#[from] AddressError),

    #[error("Node error: {0}")]
    Node(#[from] NodeError),
}

impl From<std::io::Error> for ResolveError {
    fn from(err: std::io::Error) -> Self {
        ResolveError::IoError(err.to_string())
    }
}
