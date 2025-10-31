// src/sdo_handler.rs
use crate::eds_parser::{EdsError, ObjectEntry};
use anyhow::Result;

#[derive(Debug, Clone)]
pub enum SdoRequestType {
    Upload,
    Download,
}

pub fn validate_sdo_message(
    obj: &ObjectEntry,
    req_type: SdoRequestType,
    data: &[u8],
) -> Result<()> {
    // Parse DataType like "0x0006" → u16
    let data_type_hex = obj.data_type.trim_start_matches("0x");
    let data_type = u16::from_str_radix(data_type_hex, 16)
        .map_err(|_| anyhow::anyhow!("Invalid DataType format: {}", obj.data_type))?;

    // Map CANopen DataType to byte size
    let expected_len = match data_type {
        0x0001 => 1, // BOOLEAN
        0x0002 => 1, // INTEGER8
        0x0003 => 2, // INTEGER16
        0x0004 => 4, // INTEGER32
        0x0005 => 1, // UNSIGNED8
        0x0006 => 2, // UNSIGNED16
        0x0007 => 4, // UNSIGNED32
        0x0008 => 4, // REAL32
        0x0009 => 8, // VISIBLE_STRING (we don't validate length)
        0x000A => 8, // OCTET_STRING
        0x000B => 8, // UNICODE_STRING
        0x0010 => 8, // TIME_OF_DAY
        0x0011 => 8, // TIME_DIFFERENCE
        0x0015 => 8, // REAL64
        _ => return Err(anyhow::anyhow!("Unsupported DataType: 0x{:04X}", data_type)),
    };

    if data.len() != expected_len as usize {
        anyhow::bail!(
            "Message length mismatch: expected {} {} for DataType 0x{:04X}, got {}",
            expected_len,
            if expected_len == 1 { "byte" } else { "bytes" },
            data_type,
            data.len()
        );
    }

    match req_type {
        SdoRequestType::Upload => {
            if !obj.access_rights.contains('r') {
                return Err(EdsError::Access("Read access denied".to_string()).into());
            }
        }
        SdoRequestType::Download => {
            if !obj.access_rights.contains('w') {
                return Err(EdsError::Access("Write access denied".to_string()).into());
            }
        }
    }

    Ok(())
}