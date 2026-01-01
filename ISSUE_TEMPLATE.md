# Issue Template for Notifying Original Author

Copy this content and create an issue at: https://github.com/iliazeus/iso2god-rs/issues/new

---

**Title:** GUI Addition - Fork with Graphical Interface

**Body:**

Hi @iliazeus,

I've created a fork of your excellent iso2god-rs project and added a graphical user interface. I'd love for you to review it and potentially merge it into the main repository.

## Fork Repository
https://github.com/lantzmurray/iso2god-rs-gui

## Features Added

### GUI Features
- **Native file dialogs** for selecting ISO files and destination directories
- **Optional custom game title** input with automatic title lookup from game database
- **Trim mode selection** - choose to trim unused space from ISO images or keep the full ISO
- **Real-time progress tracking** with visual progress bar and status updates
- **Multi-threaded conversion** utilizing all available CPU cores
- **Error handling** with clear error messages and success notifications

### Technical Details
- Built with **eframe/egui** for cross-platform support (Linux, macOS, Windows)
- Uses **rfd** for native file dialogs
- Thread-safe status management with `Arc<Mutex>`
- Same conversion logic as the CLI tool, running in background threads

## Files Changed

### New Files
- `gui/Cargo.toml` - GUI package manifest with dependencies
- `gui/src/main.rs` - Complete GUI implementation (419 lines)
- `gui/README.md` - Comprehensive GUI documentation

### Modified Files
- `Cargo.toml` - Added `gui` to workspace members
- `README.md` - Added GUI documentation and usage instructions
- `.github/workflows/release.yml` - Updated to build GUI binaries for all platforms
- `Cargo.lock` - Updated with new GUI dependencies

### Documentation
- `CONTRIBUTION.md` - Detailed contribution summary with testing recommendations
- `PUSH_INSTRUCTIONS.md` - Instructions for pushing the contribution
- `CREATE_REPO_AND_PUSH.md` - Step-by-step guide for creating a fork

## Why This Fork?

I prepared this GUI contribution to be submitted as a pull request, but due to lacking direct push permissions to the upstream repository, I've published it in this fork for the community to use and for you to review.

## Testing

The GUI has been tested with:
- File selection dialogs
- Progress tracking during conversion
- Error handling for invalid inputs
- Both trim modes (trim unused space vs keep full ISO)
- Custom and auto-detected game titles

## Build Instructions

```bash
# Clone the fork
git clone https://github.com/lantzmurray/iso2god-rs-gui.git
cd iso2god-rs

# Build the GUI
cargo build --release --bin iso2god-gui

# Run the GUI
./target/release/iso2god-gui
```

## Commit Message

```
Add graphical user interface for iso2god

This commit adds a GUI alternative to the command-line interface,
providing an easy-to-use way to convert Xbox 360 and original Xbox
ISOs to Games-On-Demand format.

Features:
- Native file dialogs for ISO and directory selection
- Optional custom game title input with auto-detection
- Trim mode selection (trim unused space or keep full ISO)
- Real-time progress bar and status tracking
- Multi-threaded conversion using rayon
- Error reporting and success notifications

Technical details:
- Built with eframe/egui for cross-platform support
- Uses rfd for native file dialogs
- Thread-safe status management with Arc<Mutex>
- Same conversion logic as CLI tool

Changes:
- Added gui/ directory with GUI implementation
- Updated Cargo.toml to include gui workspace member
- Updated README.md with GUI documentation
- Updated release workflow to build GUI binaries
- Added CONTRIBUTION.md with contribution details
```

## Next Steps

1. **Review the fork** - Check the implementation at https://github.com/lantzmurray/iso2god-rs-gui
2. **Provide feedback** - Let me know if you'd like any changes
3. **Submit a PR** - I can submit a pull request if you'd like to merge it directly

## Credits

- **Original iso2god-rs by @iliazeus** - Thank you for creating such a great tool!
- **GUI addition by @lantzmurray** - Hope this is useful to the community

Let me know if you have any questions or if you'd like me to submit a pull request!

---

**@lantzmurray**
