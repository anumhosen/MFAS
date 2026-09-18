use clap::{Args, Subcommand};
use mfas_core::{split_into_pages, Address, Page, RadixCoordinate, PAGE_SIZE};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct PageArgs {
    #[command(subcommand)]
    pub action: PageAction,
}

#[derive(Subcommand, Debug)]
pub enum PageAction {
    /// Inspect a page from a canonical Page address URI or from a local file
    Inspect {
        /// Address URI (mfas:v1:page:...) or path to file
        target: String,

        /// Page index to inspect when target is a file (defaults to 0)
        #[arg(short, long, default_value_t = 0)]
        index: u64,

        /// Dump raw page payload as hexadecimal
        #[arg(long)]
        hex: bool,

        /// Dump raw page payload as text
        #[arg(long)]
        text: bool,
    },
    /// Calculate and inspect the Radix-16 spatial hierarchy coordinate for any byte offset
    Coordinate {
        /// Byte offset in decimal (or hex with '0x' prefix)
        offset: String,
    },
}

#[derive(Serialize)]
struct PageInspectOutput {
    page_index: u64,
    logical_len: usize,
    max_page_size: usize,
    is_full: bool,
    coordinate: RadixCoordinate,
    coordinate_formatted: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    hex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
}

#[derive(Serialize)]
struct CoordinateOutput {
    byte_offset: u64,
    byte_offset_hex: String,
    coordinate: RadixCoordinate,
    coordinate_formatted: String,
}

pub fn handle_page_cmd(
    args: PageArgs,
    json: bool,
    quiet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match args.action {
        PageAction::Inspect {
            target,
            index,
            hex: show_hex,
            text: show_text,
        } => {
            let (page_index, page) = if target.starts_with("mfas:") {
                let addr: Address = target.parse()?;
                Page::from_address(&addr)?
            } else {
                let file_path = PathBuf::from(&target);
                let bytes = fs::read(&file_path)?;
                let pages = split_into_pages(&bytes);
                if pages.is_empty() {
                    return Err("Target file is empty".into());
                }
                if index as usize >= pages.len() {
                    return Err(format!(
                        "Page index {} out of bounds (file has {} pages)",
                        index,
                        pages.len()
                    )
                    .into());
                }
                (index, pages[index as usize].clone())
            };

            let byte_offset = page_index * PAGE_SIZE as u64;
            let coord = RadixCoordinate::from_byte_offset(byte_offset);
            let hex_data = if show_hex || json {
                Some(hex::encode(page.data()))
            } else {
                None
            };
            let text_data = if show_text || json {
                String::from_utf8(page.data().to_vec()).ok()
            } else {
                None
            };

            if json {
                let out = PageInspectOutput {
                    page_index,
                    logical_len: page.logical_len(),
                    max_page_size: PAGE_SIZE,
                    is_full: page.is_full(),
                    coordinate: coord,
                    coordinate_formatted: coord.format_coordinate(),
                    hex: hex_data,
                    text: text_data,
                };
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else if quiet {
                println!("{}", coord.format_coordinate());
            } else {
                println!("MFAS Logical Page");
                println!("──────────────────────────────────────────");
                println!("Page Index:       {}", page_index);
                println!("Logical Length:   {} / {} bytes", page.logical_len(), PAGE_SIZE);
                println!("Is Full Page:     {}", page.is_full());
                println!("Coordinate:       {}", coord.format_coordinate());
                println!("  Floor (16^8):   {}", coord.floor);
                println!("  Room (16^7):    {:X}", coord.room);
                println!("  Wall (16^6):    {:X}", coord.wall);
                println!("  Shelf (16^5):   {:X}", coord.shelf);
                println!("  Volume (16^4):  {:X}", coord.volume);
                println!("  Page (16^3):    {:X}", coord.page);

                if show_hex {
                    println!("\nPage Hex Payload:\n{}", hex::encode(page.data()));
                }
                if show_text {
                    if let Ok(s) = String::from_utf8(page.data().to_vec()) {
                        println!("\nPage Text Content:\n{}", s);
                    }
                }
            }
        }
        PageAction::Coordinate { offset } => {
            let offset_num = parse_u64(&offset)?;
            let coord = RadixCoordinate::from_byte_offset(offset_num);

            if json {
                let out = CoordinateOutput {
                    byte_offset: offset_num,
                    byte_offset_hex: format!("0x{:X}", offset_num),
                    coordinate: coord,
                    coordinate_formatted: coord.format_coordinate(),
                };
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else if quiet {
                println!("{}", coord.format_coordinate());
            } else {
                println!("Radix-16 Hierarchy Coordinates");
                println!("──────────────────────────────────────────");
                println!("Byte Offset:      {} (0x{:X})", offset_num, offset_num);
                println!("Hierarchical:     {}", coord.format_coordinate());
                println!("  Floor  (4 GiB):   {}", coord.floor);
                println!("  Room   (256 MiB): 0x{:X}", coord.room);
                println!("  Wall   (16 MiB):  0x{:X}", coord.wall);
                println!("  Shelf  (1 MiB):   0x{:X}", coord.shelf);
                println!("  Volume (64 KiB):  0x{:X}", coord.volume);
                println!("  Page   (4 KiB):   0x{:X}", coord.page);
                println!("  Offset in Page:   {} (0x{:03X})", coord.offset_in_page, coord.offset_in_page);
            }
        }
    }
    Ok(())
}

fn parse_u64(s: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let s = s.trim();
    if s.starts_with("0x") || s.starts_with("0X") {
        u64::from_str_radix(&s[2..], 16).map_err(|e| e.into())
    } else {
        s.parse::<u64>().map_err(|e| e.into())
    }
}
