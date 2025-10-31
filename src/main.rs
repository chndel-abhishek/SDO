// src/main.rs
mod eds_parser;
mod sdo_handler;

use anyhow::{Context, Result};
use clap::Parser;
use inquire::Text;
use crate::eds_parser::load_eds;
use crate::sdo_handler::{validate_sdo_message, SdoRequestType};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "Rust CANopen SDO Utility Tool")]
struct Args {
    #[arg(short, long, default_value = "can0")]
    can_device: String,

    #[arg(short, long, required = true)]
    eds_file: PathBuf,

    #[arg(short, long, required = true)]
    node_id: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // ---- node ID (u32 → u8) ---------------------------------------
    let node_id_u32 = parse_hex_or_dec(&args.node_id, 1..=127)?;
    let node_id: u8 = node_id_u32
        .try_into()
        .map_err(|_| anyhow::anyhow!("Node ID {} out of u8 range", node_id_u32))?;

    // ---- load EDS -------------------------------------------------
    let dict = load_eds(&args.eds_file, node_id)
        .context("Failed to load EDS file")?;

    println!(
        "Loaded EDS: Device Type={:#010X}, Vendor ID={:#010X}",
        dict.device_type, dict.vendor_id
    );

    // ---- interactive loop -----------------------------------------
    loop {
        let object_input = Text::new("Enter Object ID (decimal, 0xHEX, or 'quit' to exit):")
            .prompt()
            .context("Input failed")?;

        if object_input.trim().eq_ignore_ascii_case("quit") {
            println!("Goodbye!");
            break;
        }

        let object_id = match parse_hex_or_dec(&object_input, 0..=0xFFFF) {
            Ok(id) => id as u16,
            Err(_) => {
                println!("Invalid Object ID. Use 0x0000–0xFFFF or decimal.");
                continue;
            }
        };

        let obj = match dict.lookup_object(object_id) {
            Some(o) => o,
            None => {
                println!("Object {:#06X} not found", object_id);
                continue;
            }
        };

        println!(
            "Object {:#06X}: Name='{}', Access='{}', DataType='{}'",
            object_id, obj.name, obj.access_rights, obj.data_type
        );

        let sub_input = Text::new("Enter Sub-Index (decimal, 0xHEX, or 'skip'): ")
            .prompt()?;

        if !sub_input.trim().eq_ignore_ascii_case("skip") {
            let sub_index = match parse_hex_or_dec(&sub_input, 0..=0xFF) {
                Ok(idx) => idx as u8,
                Err(_) => {
                    println!("Invalid Sub-Index.");
                    continue;
                }
            };

            if let Some(sub) = obj.sub_objects.get(&sub_index) {
                let val = sub.value.as_deref().unwrap_or("None");
                let def = sub.default_value.as_deref().unwrap_or("None");
                println!(
                    "  Sub {:#04X}: Value='{}', Default='{}' (type={}, access={})",
                    sub_index, val, def, sub.data_type, sub.access
                );
            } else {
                println!("  Sub-index {:#04X} not found", sub_index);
            }
        }

        let req_type_str = Text::new("SDO Request Type (upload/download): ")
            .prompt()?
            .to_lowercase();

        let req_type = match req_type_str.as_str() {
            "upload" => SdoRequestType::Upload,
            "download" => SdoRequestType::Download,
            _ => {
                println!("Invalid type. Use 'upload' or 'download'.");
                continue;
            }
        };

        let msg_input = Text::new("Enter Message Data (HEX string, e.g., 0x01020304): ")
            .prompt()?;

        let message_bytes = match hex::decode(msg_input.trim().trim_start_matches("0x")) {
            Ok(b) => b,
            Err(_) => {
                println!("Invalid hex.");
                continue;
            }
        };

        match validate_sdo_message(obj, req_type.clone(), &message_bytes) {
            Ok(()) => {
                println!(
                    "Valid SDO {} for {:#06X} ({} bytes)",
                    if matches!(req_type, SdoRequestType::Upload) { "upload" } else { "download" },
                    object_id,
                    message_bytes.len()
                );
            }
            Err(e) => println!("Invalid SDO: {}", e),
        }

        println!("{}", "─".repeat(50));
    }

    Ok(())
}

/* --------------------------------------------------------------------- */
fn parse_hex_or_dec(input: &str, range: std::ops::RangeInclusive<u32>) -> Result<u32> {
    let s = input.trim().to_lowercase();
    let val = if s.starts_with("0x") {
        u32::from_str_radix(&s[2..], 16)
    } else {
        s.parse::<u32>()
    }
    .context("Parse error")?;

    if !range.contains(&val) {
        anyhow::bail!("Out of range");
    }
    Ok(val)
}