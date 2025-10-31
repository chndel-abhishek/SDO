# 🧰 SDO-Rust-Tool  
### Interactive CANopen SDO Tool for EDS Files  

**Version:** `v0.1.0`  
**Target:** `arm32v7` *(BeagleBoard Black, Raspberry Pi, etc.)*  
**Built with:** `Rust`, `ini`, `clap`, `inquire`, `hex`  
**Tested on:** `OPTE6_20201028.eds` *(Vacon CANopen Option Board)*  

---

## 📖 Table of Contents
1. [Overview](#overview)  
2. [Features](#features)  
3. [Prerequisites](#prerequisites)  
4. [Installation](#installation)  
   - [Option 1: Docker (Recommended)](#option-1-docker-recommended)  
   - [Option 2: Native Build on ARM32v7](#option-2-native-build-on-arm32v7)  
5. [Usage](#usage)  
6. [Interactive Mode](#interactive-mode)  
7. [Supported Data Types](#supported-data-types)  
8. [Troubleshooting](#troubleshooting)  
9. [Future Extensions](#future-extensions)  
10. [License](#license)  

---

## 1. Overview

**SDO-Rust-Tool** is a lightweight, **interactive CLI** utility for CANopen developers and testers.  
It focuses on **parsing and validating EDS files** (Electronic Data Sheets) without requiring a live CAN bus.

### 🔍 What It Does
- Parses and validates `.eds` files  
- Displays CANopen object dictionary  
- Validates **SDO Upload/Download** messages based on `DataType`  
- Runs interactively for human-friendly debugging  
- Designed for **Raspberry Pi (arm32v7)**  

> 💡 *No CAN hardware required — purely an EDS validation and simulation tool.*

---

## 2. Features

| Feature | Status |
|----------|--------|
| Parse real `.eds` files (Vacon, Beckhoff, etc.) | ✅ Done |
| Skip metadata (`[FileInfo]`, `[DeviceInfo]`, etc.) | ✅ Done |
| Extract `Device Type`, `Vendor ID` | ✅ Done |
| Browse objects by index (`0x6040`, `1018`, etc.) | ✅ Done |
| View sub-objects (`1018sub1`, `6046sub1`) | ✅ Done |
| Validate SDO message length by `DataType` | ✅ Done |
| Interactive CLI using `inquire` | ✅ Done |
| Cross-compiled for ARM32v7 | ✅ Done |

---

## 3. Prerequisites

### 🧱 Hardware
- Raspberry Pi (3, 4, Zero 2 W) or any ARM32v7 device  
- Internet access *(for Docker build or source fetch)*  

### 🧰 Software
- **Docker** *(recommended)*  
- Or: **Rust Toolchain (rustup, cargo)**  

---

## 4. Installation

### ⚙️ Option 1: Docker (Recommended)
```bash
# Clone the repository
git clone https://github.com/yourname/sdo-rust-tool.git
cd sdo-rust-tool

# Build for ARM32v7
docker buildx build --platform linux/arm/v7 -t sdo-tool .

# Run interactively
docker run --rm -it --platform linux/arm/v7 sdo-tool
```

---

### 🧩 Option 2: Native Build on ARM32v7
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Clone and build
git clone https://github.com/yourname/sdo-rust-tool.git
cd sdo-rust-tool
cargo build --release
```

**Binary Output:**  
```bash
./target/release/sdo-rust-tool
```

---

## 5. Usage

```bash
./target/release/sdo-rust-tool -e <EDS_FILE> -n <NODE_ID>
```

### Required Arguments

| Flag | Description | Example |
|------|--------------|----------|
| `-e`, `--eds-file` | Path to `.eds` file | `-e OPTE6_20201028.eds` |
| `-n`, `--node-id`  | Node ID (hex or decimal) | `-n 0x21` or `-n 33` |

### Optional Arguments

| Flag | Description | Default |
|------|--------------|----------|
| `-c`, `--can-device` | CAN interface | `can0` |

---

## 6. Interactive Mode

After the EDS file is loaded, the tool opens an interactive prompt:

```text
Loaded EDS: Device Type=0x00010192, Vendor ID=0x00000090
? Enter Object ID (decimal, 0xHEX, or 'quit' to exit):
```

### Example Session
```text
? Enter Object ID: 0x6040
Object 0x6040: Name='Controlword', Access='rww', DataType='0x0006'

? Enter Sub-Index (decimal, 0xHEX, or 'skip'): skip

? SDO Request Type (upload/download): download

? Enter Message Data (HEX string, e.g., 0x01020304): 0x0600

Valid SDO download for 0x6040 (2 bytes)
──────────────────────────────────────────────
```

---

## 7. Supported Data Types

| DataType | Hex | Size | Example |
|-----------|-----|------|----------|
| `BOOLEAN` | `0x0001` | 1 byte | `0x01` |
| `INTEGER8` | `0x0002` | 1 byte | `0xFF` |
| `INTEGER16` | `0x0003` | 2 bytes | `0x1234` |
| `INTEGER32` | `0x0004` | 4 bytes | `0x12345678` |
| `UNSIGNED8` | `0x0005` | 1 byte | `0x2A` |
| `UNSIGNED16` | `0x0006` | 2 bytes | `0x0600` |
| `UNSIGNED32` | `0x0007` | 4 bytes | `0xDEADBEEF` |
| `REAL32` | `0x0008` | 4 bytes | `0x40400000` |

✅ **Valid:** `0x0600` → 2 bytes → *Accepted*  
❌ **Invalid:** `0x06` → 1 byte → *Rejected*

---

## 8. Troubleshooting

| Error | Solution |
|-------|-----------|
| `Bad index FileInfo` | Use latest `eds_parser.rs` (skips metadata) |
| `Message length mismatch` | Enter correct byte count (e.g., `0x0600` for `UNSIGNED16`) |
| `Vendor ID = 0` | Fixed in latest parser (reads from `[DeviceInfo]`) |
| `No such file or directory` | Run `cargo build --release` first |
| `Permission denied (can0)` | Run with `sudo` or fix `udev` rules |

---

## 9. Future Extensions

| Feature | Status |
|----------|--------|
| Real CAN transmission (`socketcan`) | ⏳ Started |
| SDO upload/download over CAN | ⏳ Not Started |
| DCF file generation | ⏳ Not Started |
| Object autocomplete (tab) | ⏳ Not Started |
| JSON export of object dictionary | ⏳ Not Started |
| Web UI (WASM) | ⏳ Not Started |

---
