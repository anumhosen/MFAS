pub mod address;
pub mod codec;
pub mod dag;
pub mod gpu;
pub mod page;
pub mod window;

use mfas_core::address::{Address, NodeType};

pub fn parse_address_str(raw: &str) -> Result<Address, String> {
    let s = raw.trim();
    // 1. Try URI parsing (e.g. mfas:v1:data:48656c6c6f)
    if let Ok(addr) = s.parse::<Address>() {
        return Ok(addr);
    }
    // 2. Try hex decode
    let clean_hex = s.trim_start_matches("0x");
    if let Ok(bytes) = hex::decode(clean_hex) {
        // Full binary format with MFAS magic
        if let Ok(addr) = Address::from_binary(&bytes) {
            return Ok(addr);
        }
        // Compact hex format: [version (1B), type_id (1B), payload (NB)]
        if bytes.len() >= 2 && bytes[0] == 1 {
            if let Ok(node_type) = NodeType::from_type_id(bytes[1]) {
                if let Ok(addr) = Address::new(node_type, bytes[2..].to_vec()) {
                    return Ok(addr);
                }
            }
        }
    }
    s.parse::<Address>().map_err(|e| e.to_string())
}
