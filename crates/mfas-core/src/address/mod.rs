use crate::error::AddressError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

/// Magic bytes for binary address representation: ASCII "MFAS"
pub const BINARY_MAGIC: [u8; 4] = [0x4D, 0x46, 0x41, 0x53];
/// Current address specification version
pub const CURRENT_VERSION: u8 = 1;
/// Text protocol scheme prefix
pub const SCHEME_PREFIX: &str = "mfas";

/// Logical node type classifying an MFAS address
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NodeType {
    /// Literal raw byte content (Type ID: 0x01)
    Data = 1,
    /// Reference to another address / hash (Type ID: 0x02)
    Ref = 2,
    /// Ordered sequence of child addresses (Type ID: 0x03)
    Seq = 3,
    /// Repetition of a child node (Type ID: 0x04)
    Rep = 4,
    /// Sliced range of a child node (Type ID: 0x05)
    Slice = 5,
    /// Canonical 4096-byte logical page (Type ID: 0x06)
    Page = 6,
}

impl NodeType {
    /// Return the canonical string identifier of this node type
    pub fn as_str(&self) -> &'static str {
        match self {
            NodeType::Data => "data",
            NodeType::Ref => "ref",
            NodeType::Seq => "seq",
            NodeType::Rep => "rep",
            NodeType::Slice => "slice",
            NodeType::Page => "page",
        }
    }

    /// Return the 1-byte binary identifier
    pub fn type_id(&self) -> u8 {
        *self as u8
    }

    /// Parse from a 1-byte binary identifier
    pub fn from_type_id(id: u8) -> Result<Self, AddressError> {
        match id {
            1 => Ok(NodeType::Data),
            2 => Ok(NodeType::Ref),
            3 => Ok(NodeType::Seq),
            4 => Ok(NodeType::Rep),
            5 => Ok(NodeType::Slice),
            6 => Ok(NodeType::Page),
            _ => Err(AddressError::UnknownBinaryTypeId(id)),
        }
    }
}

impl FromStr for NodeType {
    type Err = AddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "data" => Ok(NodeType::Data),
            "ref" | "reference" => Ok(NodeType::Ref),
            "seq" | "sequence" => Ok(NodeType::Seq),
            "rep" | "repeat" => Ok(NodeType::Rep),
            "slice" => Ok(NodeType::Slice),
            "page" => Ok(NodeType::Page),
            _ => Err(AddressError::UnknownNodeType(s.to_string())),
        }
    }
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Strongly typed, canonical representation of an MFAS address
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Address {
    version: u8,
    node_type: NodeType,
    payload: Vec<u8>,
}

impl Address {
    /// Create a new Address with validated components
    pub fn new(node_type: NodeType, payload: Vec<u8>) -> Result<Self, AddressError> {
        if payload.is_empty() && node_type != NodeType::Data {
            return Err(AddressError::EmptyPayload);
        }
        Ok(Self {
            version: CURRENT_VERSION,
            node_type,
            payload,
        })
    }

    /// Construct an address from a raw hex payload string
    pub fn from_hex(node_type: NodeType, hex_str: &str) -> Result<Self, AddressError> {
        let cleaned = hex_str.trim();
        if cleaned.is_empty() {
            if node_type == NodeType::Data {
                return Self::new(node_type, Vec::new());
            }
            return Err(AddressError::EmptyPayload);
        }
        if cleaned.len() % 2 != 0 {
            return Err(AddressError::OddLengthHex(cleaned.len()));
        }
        let payload = hex::decode(cleaned)
            .map_err(|e| AddressError::InvalidHex(e.to_string()))?;
        Self::new(node_type, payload)
    }

    /// Specification version of the address
    pub fn version(&self) -> u8 {
        self.version
    }

    /// Node type of the address
    pub fn node_type(&self) -> NodeType {
        self.node_type
    }

    /// Reference to the underlying binary payload
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    /// Lowercase canonical hex string of the payload
    pub fn payload_hex(&self) -> String {
        hex::encode(&self.payload)
    }

    /// Emit the canonical text URI representation: `mfas:v1:<node_type>:<hex_payload>`
    pub fn to_uri(&self) -> String {
        format!("{}:v{}:{}:{}", SCHEME_PREFIX, self.version, self.node_type.as_str(), self.payload_hex())
    }

    /// Serialize into the canonical binary format:
    /// `[Magic 4B][Version 1B][TypeID 1B][LEB128 Length][Payload]`
    pub fn to_binary(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(4 + 1 + 1 + 5 + self.payload.len());
        out.extend_from_slice(&BINARY_MAGIC);
        out.push(self.version);
        out.push(self.node_type.type_id());
        encode_leb128(self.payload.len() as u64, &mut out);
        out.extend_from_slice(&self.payload);
        out
    }

    /// Deserialize from the canonical binary format
    pub fn from_binary(bytes: &[u8]) -> Result<Self, AddressError> {
        if bytes.len() < 6 {
            return Err(AddressError::MalformedBinaryPayload);
        }
        if bytes[0..4] != BINARY_MAGIC {
            let mut magic = [0u8; 4];
            magic.copy_from_slice(&bytes[0..4]);
            return Err(AddressError::InvalidBinaryMagic(magic));
        }
        let version = bytes[4];
        if version != CURRENT_VERSION {
            return Err(AddressError::UnsupportedBinaryVersion(version));
        }
        let type_id = bytes[5];
        let node_type = NodeType::from_type_id(type_id)?;

        let mut offset = 6;
        let payload_len = decode_leb128(bytes, &mut offset)? as usize;

        if bytes.len() - offset != payload_len {
            return Err(AddressError::MalformedBinaryPayload);
        }

        let payload = bytes[offset..offset + payload_len].to_vec();
        if payload.is_empty() && node_type != NodeType::Data {
            return Err(AddressError::EmptyPayload);
        }

        Ok(Self {
            version,
            node_type,
            payload,
        })
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_uri())
    }
}

impl FromStr for Address {
    type Err = AddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 4 {
            return Err(AddressError::MalformedUri(s.to_string()));
        }

        let scheme = parts[0].to_ascii_lowercase();
        if scheme != SCHEME_PREFIX {
            return Err(AddressError::InvalidPrefix(parts[0].to_string()));
        }

        let ver_str = parts[1].to_ascii_lowercase();
        if !ver_str.starts_with('v') {
            return Err(AddressError::InvalidVersion(parts[1].to_string()));
        }
        let version_num: u8 = ver_str[1..]
            .parse()
            .map_err(|_| AddressError::InvalidVersion(parts[1].to_string()))?;
        if version_num != CURRENT_VERSION {
            return Err(AddressError::InvalidVersion(parts[1].to_string()));
        }

        let node_type = NodeType::from_str(parts[2])?;
        let hex_payload = parts[3];

        if hex_payload.is_empty() {
            if node_type == NodeType::Data {
                return Ok(Self {
                    version: version_num,
                    node_type,
                    payload: Vec::new(),
                });
            }
            return Err(AddressError::EmptyPayload);
        }
        if hex_payload.len() % 2 != 0 {
            return Err(AddressError::OddLengthHex(hex_payload.len()));
        }

        let payload = hex::decode(hex_payload)
            .map_err(|e| AddressError::InvalidHex(e.to_string()))?;

        Ok(Self {
            version: version_num,
            node_type,
            payload,
        })
    }
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_uri())
    }
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Address::from_str(&s).map_err(serde::de::Error::custom)
    }
}

/// Encode a u64 into LEB128 variable-length bytes
fn encode_leb128(mut value: u64, out: &mut Vec<u8>) {
    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        out.push(byte);
        if value == 0 {
            break;
        }
    }
}

/// Decode a u64 from LEB128 variable-length bytes
fn decode_leb128(bytes: &[u8], offset: &mut usize) -> Result<u64, AddressError> {
    let mut result: u64 = 0;
    let mut shift = 0;
    while *offset < bytes.len() {
        let byte = bytes[*offset];
        *offset += 1;
        result |= ((byte & 0x7F) as u64)
            .checked_shl(shift)
            .ok_or(AddressError::MalformedBinaryPayload)?;
        if (byte & 0x80) == 0 {
            return Ok(result);
        }
        shift += 7;
        if shift > 63 {
            return Err(AddressError::MalformedBinaryPayload);
        }
    }
    Err(AddressError::MalformedBinaryPayload)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_address_roundtrip() {
        let raw_data = b"Hello, MFAS!";
        let addr = Address::new(NodeType::Data, raw_data.to_vec()).unwrap();
        let uri = addr.to_uri();
        assert_eq!(uri, "mfas:v1:data:48656c6c6f2c204d46415321");

        let parsed: Address = uri.parse().unwrap();
        assert_eq!(parsed, addr);
        assert_eq!(parsed.node_type(), NodeType::Data);
        assert_eq!(parsed.payload(), raw_data);
    }

    #[test]
    fn test_case_insensitivity_parsing() {
        let upper_uri = "MFAS:V1:DATA:48656C6C6F";
        let parsed: Address = upper_uri.parse().unwrap();
        assert_eq!(parsed.to_uri(), "mfas:v1:data:48656c6c6f");
    }

    #[test]
    fn test_odd_length_hex_rejected() {
        let bad_uri = "mfas:v1:data:48656c6c6";
        let err = bad_uri.parse::<Address>().unwrap_err();
        assert!(matches!(err, AddressError::OddLengthHex(9)));
    }

    #[test]
    fn test_invalid_prefix_and_version() {
        assert!(matches!(
            "http:v1:data:48".parse::<Address>().unwrap_err(),
            AddressError::InvalidPrefix(_)
        ));
        assert!(matches!(
            "mfas:v2:data:48".parse::<Address>().unwrap_err(),
            AddressError::InvalidVersion(_)
        ));
    }

    #[test]
    fn test_binary_serialization_roundtrip() {
        let addr = Address::new(NodeType::Ref, vec![0xAB, 0xCD, 0xEF, 0x01, 0x23]).unwrap();
        let binary = addr.to_binary();
        assert_eq!(&binary[0..4], &BINARY_MAGIC);
        assert_eq!(binary[4], 1); // version 1
        assert_eq!(binary[5], NodeType::Ref.type_id()); // type ID 2

        let restored = Address::from_binary(&binary).unwrap();
        assert_eq!(restored, addr);
    }

    #[test]
    fn test_all_node_types_roundtrip() {
        let types = [
            NodeType::Data,
            NodeType::Ref,
            NodeType::Seq,
            NodeType::Rep,
            NodeType::Slice,
            NodeType::Page,
        ];
        for nt in types {
            let addr = Address::new(nt, vec![0xCA, 0xFE]).unwrap();
            let uri = addr.to_uri();
            let parsed: Address = uri.parse().unwrap();
            assert_eq!(parsed.node_type(), nt);
            assert_eq!(parsed, addr);

            let bin = addr.to_binary();
            let restored = Address::from_binary(&bin).unwrap();
            assert_eq!(restored, addr);
        }
    }

    #[test]
    fn test_empty_payload_rejected() {
        assert!(matches!(
            Address::new(NodeType::Ref, vec![]).unwrap_err(),
            AddressError::EmptyPayload
        ));
        assert!(matches!(
            "mfas:v1:ref:".parse::<Address>().unwrap_err(),
            AddressError::EmptyPayload
        ));
    }

    #[test]
    fn test_empty_data_node_allowed() {
        let addr = Address::new(NodeType::Data, vec![]).unwrap();
        assert_eq!(addr.to_uri(), "mfas:v1:data:");
        let parsed: Address = "mfas:v1:data:".parse().unwrap();
        assert_eq!(parsed, addr);
        assert!(parsed.payload().is_empty());

        let bin = addr.to_binary();
        let restored = Address::from_binary(&bin).unwrap();
        assert_eq!(restored, addr);
    }

    #[test]
    fn test_total_ordering() {
        let a1 = Address::new(NodeType::Data, vec![0x01]).unwrap();
        let a2 = Address::new(NodeType::Data, vec![0x02]).unwrap();
        let a3 = Address::new(NodeType::Ref, vec![0x01]).unwrap();

        assert!(a1 < a2);
        assert!(a1 < a3); // Data (type 1) < Ref (type 2)
    }
}
