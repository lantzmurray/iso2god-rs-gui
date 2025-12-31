# GUI Contribution Summary

## Overview

This contribution adds a graphical user interface (GUI) to the iso2god-rs project, providing an easy-to-use alternative to the command-line interface for converting Xbox 360 and original Xbox ISOs to Games-On-Demand format.

## Files Added

### New Files
- [`gui/Cargo.toml`](gui/Cargo.toml) - Cargo manifest for the GUI package
- [`gui/src/main.rs`](gui/src/main.rs) - Complete GUI application implementation (419 lines)
- [`gui/README.md`](gui/README.md) - Documentation for the GUI component

### Modified Files
- [`Cargo.toml`](Cargo.toml) - Added `gui` to workspace members
- [`README.md`](README.md) - Added GUI documentation and usage instructions
- [`.github/workflows/release.yml`](.github/workflows/release.yml) - Updated to build and release GUI binaries

## Features Implemented

### User Interface
- Native file dialogs for selecting ISO files and destination directories
- Optional custom game title input field
- Trim mode selection dropdown (trim unused space or keep full ISO)
- Real-time progress bar with percentage display
- Status messages showing current conversion stage
- Error reporting with clear error messages
- Success notifications

### Technical Implementation
- **eframe/egui**: Cross-platform GUI framework
- **rfd**: Native file dialog integration
- **rayon**: Multi-threaded conversion processing
- **Arc<Mutex<ConversionStatus>>**: Thread-safe status tracking
- Background thread execution to keep UI responsive during conversion

### Conversion Logic
The GUI uses the same conversion logic as the CLI tool:
1. Extracts ISO metadata and title information
2. Clears destination data directory
3. Writes GOD parts in parallel using rayon
4. Calculates MHT hash chain
5. Writes CON header with game title

## Build Instructions

### Development Build
```bash
cargo build --bin iso2god-gui
```

### Release Build
```bash
cargo build --release --bin iso2god-gui
```

### Running
```bash
cargo run --bin iso2god-gui
```

## Platform Support

The GUI supports all platforms that iso2god-rs supports:
- Linux (x86_64 and ARM64)
- macOS (x86_64 and Apple Silicon)
- Windows (x86_64)

## Release Workflow

The GitHub Actions workflow has been updated to build and distribute GUI binaries alongside CLI binaries. The release artifacts will include:

For each platform:
- `iso2god-*` - CLI binary
- `iso2god-gui-*` - GUI binary

## Dependencies Added

The GUI package adds the following dependencies:
- `eframe = "0.29.1"` - GUI framework
- `rfd = "0.15.0"` - File dialogs
- `anyhow = "1.0"` - Error handling
- `rayon = "1.10"` - Parallel processing
- `log = "0.4"` - Logging
- `env_logger = "0.11"` - Logger implementation

## Testing Recommendations

Before merging, test the GUI on each supported platform:

1. **Basic Functionality**
   - Select a valid ISO file
   - Select a destination directory
   - Start conversion
   - Verify progress bar updates correctly
   - Verify success message appears

2. **Error Handling**
   - Try selecting a non-ISO file
   - Try selecting a non-existent directory
   - Try converting with invalid permissions
   - Verify clear error messages are displayed

3. **Optional Features**
   - Test with custom game title
   - Test with empty game title (auto-detect)
   - Test both trim modes
   - Verify trim mode affects output size

4. **Concurrent Operations**
   - Start multiple conversions (if UI allows)
   - Verify status tracking works correctly

## Code Quality

The GUI implementation follows Rust best practices:
- Proper error handling with `anyhow::Context`
- Thread-safe state management with `Arc<Mutex>`
- Clean separation of UI and conversion logic
- Comprehensive documentation in comments
- Follows existing code style from the CLI tool

## Future Enhancements (Optional)

Potential improvements for future versions:
- Drag-and-drop file support
- Conversion history/log
- Batch conversion of multiple ISOs
- Settings persistence
- Dark/light theme toggle
- Internationalization support

## License

This contribution is licensed under the same MIT License as the main project.

## Contact

For questions or issues with the GUI contribution, please refer to the main project's issue tracker.
