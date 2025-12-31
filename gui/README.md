# Iso2God GUI

A graphical user interface for iso2god-rs, providing an easy-to-use way to convert Xbox 360 and original Xbox ISOs into Games-On-Demand format.

## Features

- **File Selection**: Native file dialogs for selecting source ISO files and destination directories
- **Game Title Support**: Optional custom game title input, with automatic title lookup from game database
- **Trim Options**: Choose to trim unused space from ISO images or keep the full ISO
- **Real-time Progress**: Visual progress bar and status updates during conversion
- **Multi-threaded Processing**: Utilizes all available CPU cores for faster conversion
- **Error Handling**: Clear error messages and success notifications

## Building

### Prerequisites

- Rust 1.85.1 or later
- Cargo package manager

### Build from Source

```bash
# Build in debug mode (faster compilation, slower execution)
cargo build --bin iso2god-gui

# Build in release mode (slower compilation, faster execution)
cargo build --release --bin iso2god-gui
```

### Running

```bash
# Run directly with cargo
cargo run --bin iso2god-gui

# Or run the compiled binary
./target/debug/iso2god-gui          # Debug build
./target/release/iso2god-gui        # Release build
```

## Usage

1. **Select Source ISO**: Click the "Select ISO..." button and choose your Xbox ISO file
2. **Select Destination**: Click the "Select Folder..." button and choose where to save the GOD files
3. **Optional - Set Game Title**: Enter a custom game title in the text field (will auto-detect if left blank)
4. **Choose Trim Mode**:
   - "Trim Unused Space (From End)": Removes unused data from the end of the ISO, reducing file size
   - "Do Not Trim": Keeps the full ISO image
5. **Convert**: Click the "Convert" button to start the conversion process
6. **Monitor Progress**: Watch the progress bar and status updates during conversion

## Technical Details

The GUI is built using:
- **eframe**: Framework for creating cross-platform GUI applications with egui
- **egui**: Immediate mode GUI library for Rust
- **rfd**: Rust File Dialog for native file picker dialogs
- **rayon**: Data parallelism library for multi-threaded conversion
- **anyhow**: Error handling and context

The conversion logic is the same as the command-line tool, running in a background thread to keep the UI responsive.

## Platform Support

The GUI supports all platforms that iso2god-rs supports:
- Linux (x86_64 and ARM64)
- macOS (x86_64 and Apple Silicon)
- Windows (x86_64)

## License

This GUI component is part of the iso2god-rs project and is licensed under the MIT License. See the main project LICENSE file for details.
