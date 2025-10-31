// src/eds_parser.rs
use anyhow::Result;
use ini::Ini;
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EdsError {
    #[error("EDS parse error: {0}")]
    Parse(String),
    #[error("Missing mandatory object 0x1000")]
    MissingDeviceType,
    #[error("Missing Identity object 0x1018")]
    MissingIdentity,
    #[error("Access denied: {0}")]
    Access(String),
}

#[derive(Clone, Debug)]
pub struct SubObjectEntry {
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub data_type: String,
    pub access: String,
}

#[derive(Clone, Debug)]
pub struct ObjectEntry {
    pub name: String,
    pub data_type: String,
    pub access_rights: String,
    pub sub_objects: HashMap<u8, SubObjectEntry>,
}

#[derive(Clone, Debug)]
pub struct ObjectDict {
    pub device_type: u32,
    pub vendor_id: u32,
    pub objects: HashMap<u16, ObjectEntry>,
}

impl ObjectDict {
    pub fn lookup_object(&self, id: u16) -> Option<&ObjectEntry> {
        self.objects.get(&id)
    }
}

fn parse_hex_or_dec(s: &str) -> Result<u32, EdsError> {
    let s = s.trim().trim_start_matches("0x");
    u32::from_str_radix(s, 16)
        .or_else(|_| s.parse::<u32>())
        .map_err(|e| EdsError::Parse(e.to_string()))
}

pub fn load_eds(eds_path: &Path, _node_id: u8) -> Result<ObjectDict, EdsError> {
    let ini = Ini::load_from_file(eds_path)
        .map_err(|e| EdsError::Parse(e.to_string()))?;

    // --- Mandatory objects ---
    let dev_type = ini
        .section(Some("1000"))
        .and_then(|s| s.get("DefaultValue"))
        .ok_or(EdsError::MissingDeviceType)
        .and_then(parse_hex_or_dec)?;

    let identity = ini
        .section(Some("1018"))
        .ok_or(EdsError::MissingIdentity)?;
    let vendor_id = identity
        .get("Sub1")
        .or_else(|| identity.get("1"))
        .and_then(|s| parse_hex_or_dec(s).ok())
        .unwrap_or(0);

    let mut objects = HashMap::new();

    // --- Walk all sections ---
    for (sec, props) in ini.iter() {
        let sec = sec.as_deref().unwrap_or("");
        if sec.is_empty() { continue; }

        // === SKIP NON-OBJECT SECTIONS ===
        if ["FileInfo", "DeviceInfo", "DummyUsage", "Comments", "MandatoryObjects", "OptionalObjects", "ManufacturerObjects", "Dummy"].contains(&sec) {
            continue;
        }

        // === Parse index and subindex ===
        let (index_hex, sub) = if let Some(stripped) = sec.strip_suffix("sub") {
            (stripped.trim_end_matches(char::is_numeric), Some(stripped))
        } else if let Some((idx, sub_part)) = sec.split_once(" Sub ") {
            (idx, Some(sub_part))
        } else {
            (sec, None)
        };

        // === Skip if index is not valid hex ===
        let index = match u16::from_str_radix(index_hex, 16) {
            Ok(i) => i,
            Err(_) => continue,
        };

        // --- Object entry (no subindex) ---
        if sub.is_none() {
            let name = props.get("ParameterName").unwrap_or("Unnamed").to_string();
            let data_type = props.get("DataType").unwrap_or("0x0005").to_string();
            let access = props.get("AccessType").unwrap_or("ro").to_string();

            objects.insert(
                index,
                ObjectEntry {
                    name,
                    data_type,
                    access_rights: access,
                    sub_objects: HashMap::new(),
                },
            );
            continue;
        }

        // --- Sub-object ---
        let sub_idx_str = sub.unwrap();
        let sub_idx = if sub_idx_str.starts_with("0x") {
            match u8::from_str_radix(&sub_idx_str[2..], 16) {
                Ok(i) => i,
                Err(_) => continue,
            }
        } else {
            match sub_idx_str.parse::<u8>() {
                Ok(i) => i,
                Err(_) => continue,
            }
        };

        let entry = objects.entry(index).or_insert_with(|| ObjectEntry {
            name: "Unnamed".into(),
            data_type: "UNKNOWN".into(),
            access_rights: "rw".into(),
            sub_objects: HashMap::new(),
        });

        let sub = SubObjectEntry {
            value: props.get("Value").map(|s| s.to_string()),
            default_value: props.get("DefaultValue").map(|s| s.to_string()),
            data_type: props.get("DataType").unwrap_or("0x0005").to_string(),
            access: props.get("AccessType").unwrap_or("rw").to_string(),
        };
        entry.sub_objects.insert(sub_idx, sub);
    }

    Ok(ObjectDict {
        device_type: dev_type,
        vendor_id,
        objects,
    })
}