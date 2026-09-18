use crate::commands::parse_address_str;
use mfas_core::address::{Address, NodeType};
use mfas_core::enumeration::{bytes_to_number, number_to_bytes};
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressDetailsDto {
    pub raw: String,
    pub canonical_hex: String,
    pub canonical_uri: String,
    pub version: u8,
    pub node_type: u8,
    pub node_type_name: String,
    pub payload_hex: String,
    pub payload_len: usize,
    pub leb128_hex: String,
    pub is_valid: bool,
}

#[tauri::command]
pub fn parse_address(raw: String) -> Result<AddressDetailsDto, String> {
    let addr = parse_address_str(&raw)?;
    let node_type = addr.node_type();
    let node_type_name = match node_type {
        NodeType::Data => "Data",
        NodeType::Ref => "Reference",
        NodeType::Seq => "Sequence",
        NodeType::Rep => "Repeat",
        NodeType::Slice => "Slice",
        NodeType::Page => "Page",
    }
    .to_string();

    let binary = addr.to_binary();
    let leb128_hex = binary.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ");

    Ok(AddressDetailsDto {
        raw: raw.clone(),
        canonical_hex: hex::encode(&binary),
        canonical_uri: addr.to_uri(),
        version: addr.version(),
        node_type: node_type.type_id(),
        node_type_name,
        payload_hex: addr.payload_hex(),
        payload_len: addr.payload().len(),
        leb128_hex,
        is_valid: true,
    })
}

#[tauri::command]
pub fn format_address(node_type_val: u8, payload_hex: String) -> Result<String, String> {
    let clean_hex = payload_hex.trim().trim_start_matches("0x");
    let payload = hex::decode(clean_hex).map_err(|e| format!("Invalid hex payload: {}", e))?;
    let node_type = NodeType::from_type_id(node_type_val).map_err(|e| e.to_string())?;
    let addr = Address::new(node_type, payload).map_err(|e| e.to_string())?;
    Ok(addr.to_uri())
}

#[tauri::command]
pub fn number_to_address(num_str: String) -> Result<String, String> {
    let clean = num_str.trim();
    let n = BigUint::from_str(clean).map_err(|e| format!("Invalid number: {}", e))?;
    let bytes = number_to_bytes(&n);
    let addr = Address::new(NodeType::Data, bytes).map_err(|e| e.to_string())?;
    Ok(addr.to_uri())
}

#[tauri::command]
pub fn address_to_number(raw_addr: String) -> Result<String, String> {
    let addr = parse_address_str(&raw_addr)?;
    let n = bytes_to_number(addr.payload());
    Ok(n.to_string())
}
