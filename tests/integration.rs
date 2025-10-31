#[cfg(test)]
mod tests {
    use sdo_rust_tool::eds_parser::{load_eds, ObjectDict};
    use std::path::PathBuf;
    use tempfile::NamedTempFile;

    // Placeholder: Assumes sample_eds.ini in tests/ with basic [1018] Identity, etc.
    // Replace with your sample EDS content.

    #[test]
    fn test_eds_file_exists() {
        let path = PathBuf::from("tests/sample_eds.ini");
        assert!(path.exists(), "Sample EDS file must exist for tests");
    }

    #[test]
    fn test_node_id_parsing() {
        // From main.rs parse function
        let node_dec = super::main::parse_hex_or_dec("42", 1..=127).unwrap();
        assert_eq!(node_dec, 42);
        let node_hex = super::main::parse_hex_or_dec("0x2A", 1..=127).unwrap();
        assert_eq!(node_hex, 42);
        let invalid = std::panic::catch_unwind(|| super::main::parse_hex_or_dec("128", 1..=127));
        assert!(invalid.is_err());
    }

    #[test]
    fn test_object_id_parsing() {
        let obj_dec = super::main::parse_hex_or_dec("1018", 0..=0xFFFF).unwrap();
        assert_eq!(obj_dec, 0x1018);
        let obj_hex = super::main::parse_hex_or_dec("0x1018", 0..=0xFFFF).unwrap();
        assert_eq!(obj_hex, 0x1018);
    }

    #[test]
    fn test_subobject_id_parsing() {
        let sub_dec = super::main::parse_hex_or_dec("1", 0..=0xFF).unwrap();
        assert_eq!(sub_dec as u8, 1);
    }

    #[test]
    fn test_object_exists_in_eds() {
        let eds_path = PathBuf::from("tests/sample_eds.ini");
        let dict = load_eds(&eds_path, 0x01).unwrap();
        assert!(dict.lookup_object(0x1018).is_some(), "Identity object must exist");
    }

    #[test]
    fn test_message_length_and_structure() {
        use sdo_rust_tool::sdo_handler::{validate_sdo_message, SdoRequestType};
        let sample_obj = ObjectEntry { /* Mock for UNSIGNED32 */ data_type: canopen::Value::Unsigned32, ..Default::default() };
        let valid_data: [u8; 4] = [0x01, 0x02, 0x03, 0x04];
        validate_sdo_message(&sample_obj, SdoRequestType::Download, &valid_data).unwrap();

        let invalid_data: [u8; 2] = [0x01, 0x02];
        let err = validate_sdo_message(&sample_obj, SdoRequestType::Download, &invalid_data).unwrap_err();
        assert!(err.to_string().contains("length mismatch"));
    }

    #[test]
    fn test_hex_message_conversion() {
        let hex_str = "0x01020304";
        let bytes = hex::decode(&hex_str.replace("0x", "")).unwrap();
        assert_eq!(bytes, vec![1, 2, 3, 4]);
    }
}