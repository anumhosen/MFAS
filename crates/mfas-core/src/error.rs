use thiserror::Error;

/// Core error types for the MFAS system
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum CoreError {
    #[error("Address error: {0}")]
    Address(#[from] AddressError),
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
