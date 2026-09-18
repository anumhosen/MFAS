use crate::commands::parse_address_str;
use mfas_core::page::{Page, RadixCoordinate, PAGE_SIZE};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadixCoordinatesDto {
    pub byte_offset: u64,
    pub page_index: u64,
    pub floor: u64,
    pub room: u8,
    pub wall: u8,
    pub shelf: u8,
    pub volume: u8,
    pub page: u8,
    pub offset_in_page: u16,
    pub formatted: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageDetailsDto {
    pub page_index: u64,
    pub logical_len: usize,
    pub is_full: bool,
    pub hex_preview: String,
    pub ascii_preview: String,
    pub total_size: usize,
    pub address_hex: String,
}

#[tauri::command]
pub fn get_radix_coordinates(input_val: u64, is_page_index: bool) -> Result<RadixCoordinatesDto, String> {
    let byte_offset = if is_page_index {
        input_val.saturating_mul(PAGE_SIZE as u64)
    } else {
        input_val
    };

    let coord = RadixCoordinate::from_byte_offset(byte_offset);
    let page_index = byte_offset >> 12;

    Ok(RadixCoordinatesDto {
        byte_offset,
        page_index,
        floor: coord.floor,
        room: coord.room,
        wall: coord.wall,
        shelf: coord.shelf,
        volume: coord.volume,
        page: coord.page,
        offset_in_page: coord.offset_in_page,
        formatted: coord.format_coordinate(),
    })
}

#[tauri::command]
pub fn inspect_page_data(raw_addr: String) -> Result<PageDetailsDto, String> {
    let addr = parse_address_str(&raw_addr)?;
    let (page_index, page) = Page::from_address(&addr).map_err(|e| e.to_string())?;
    
    let data = page.data();
    let preview_len = data.len().min(128);
    let hex_preview = hex::encode(&data[..preview_len]);
    
    let ascii_preview: String = data[..preview_len]
        .iter()
        .map(|&b| if b >= 32 && b <= 126 { b as char } else { '.' })
        .collect();

    Ok(PageDetailsDto {
        page_index,
        logical_len: page.logical_len(),
        is_full: page.is_full(),
        hex_preview,
        ascii_preview,
        total_size: PAGE_SIZE,
        address_hex: addr.to_uri(),
    })
}
