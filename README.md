# iso2god-rs with GUI

**Fork of:** https://github.com/iliazeus/iso2god-rs

**Original Author:** Ilia Pozdnyakov (@iliazeus)

**Fork Maintainer:** Lantz Murray (@lantzmurray)

---

## About This Fork

This fork adds a **graphical user interface (GUI)** to the excellent iso2god-rs project created by [@iliazeus](https://github.com/iliazeus).

**Important Note:** This GUI contribution was prepared to be submitted as a pull request to the original repository, but due to lacking direct push permissions to the upstream repository, it has been published in this fork for the community to use and for the original author to review and potentially merge.

## What's New in This Fork

### GUI Features
- **Native file dialogs** for selecting ISO files and destination directories
- **Optional custom game title** input with automatic title lookup from game database
- **Trim mode selection** - choose to trim unused space from ISO images or keep the full ISO
- **Real-time progress tracking** with visual progress bar and status updates
- **Multi-threaded conversion** utilizing all available CPU cores
- **Error handling** with clear error messages and success notifications

### Technical Implementation
- Built with **eframe/egui** for cross-platform GUI support
- Uses **rfd** for native file picker dialogs
- Thread-safe status management with `Arc<Mutex>`
- Same conversion logic as the CLI tool, running in background threads
- Supports Linux, macOS, and Windows

## Installation

### Building from Source

```bash
# Clone this repository
git clone https://github.com/lantzmurray/iso2god-rs-gui.git
cd iso2god-rs

# Build the GUI
cargo build --release --bin iso2god-gui

# Run the GUI
./target/release/iso2god-gui
```

### Building the CLI

```bash
# Build the command-line tool
cargo build --release --bin iso2god

# Run the CLI
./target/release/iso2god --help
```

## Usage

### GUI Usage

1. **Select Source ISO**: Click the "Select ISO..." button and choose your Xbox ISO file
2. **Select Destination**: Click the "Select Folder..." button and choose where to save the GOD files
3. **Optional - Set Game Title**: Enter a custom game title in the text field (will auto-detect if left blank)
4. **Choose Trim Mode**:
   - "Trim Unused Space (From End)": Removes unused data from the end of the ISO, reducing file size
   - "Do Not Trim": Keeps the full ISO image
5. **Convert**: Click the "Convert" button to start the conversion process
6. **Monitor Progress**: Watch the progress bar and status updates during conversion

### CLI Usage

The original CLI tool is unchanged and fully functional:

```bash
iso2god [OPTIONS] <SOURCE_ISO> <DEST_DIR>

Arguments:
  <SOURCE_ISO>  ISO file to convert
  <DEST_DIR>    A folder to write resulting GOD files to

Options:
      --dry-run             Do not convert anything, just print the title info
      --game-title <TITLE>  Set game title
      --trim                Trim off unused space from the ISO image
  -j, --num-threads <N>     Number of worker threads to use
  -h, --help                Print help
  -V, --version             Print version
```

## Credits

### Original Project
- **Author:** Ilia Pozdnyakov (@iliazeus)
- **Repository:** https://github.com/iliazeus/iso2god-rs
- **Description:** An optimized rewrite of https://github.com/eliecharra/iso2god-cli

### GUI Addition
- **Developer:** Lantz Murray (@lantzmurray)
- **Purpose:** Add user-friendly graphical interface to the excellent iso2god-rs tool

## License

This project, including the GUI addition, is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

The original iso2god-rs project is also licensed under the MIT License by Ilia Pozdnyakov.

## Contributing

### To the Original Project

If you're the original author (@iliazeus) or a contributor to the original repository, please review the GUI implementation. The changes are organized as follows:

**New Files:**
- `gui/Cargo.toml` - GUI package manifest
- `gui/src/main.rs` - Complete GUI implementation (419 lines)
- `gui/README.md` - GUI documentation

**Modified Files:**
- `Cargo.toml` - Added `gui` to workspace members
- `README.md` - Added GUI documentation and usage instructions
- `.github/workflows/release.yml` - Updated to build GUI binaries
- `Cargo.lock` - Updated with new dependencies

**Documentation:**
- `CONTRIBUTION.md` - Detailed contribution summary
- `PUSH_INSTRUCTIONS.md` - Instructions for pushing the contribution

### To This Fork

If you'd like to contribute improvements to the GUI or other features:

1. Fork this repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Acknowledgments

- Huge thanks to **Ilia Pozdnyakov (@iliazeus)** for creating the excellent iso2god-rs tool
- Thanks to the original iso2god-cli project by **eliecharra** for the inspiration
- Thanks to the **eframe/egui** team for the excellent GUI framework
- Thanks to the Rust community for all the amazing libraries used

## Status

This fork is maintained and will be kept in sync with the upstream repository. The GUI feature is stable and ready for use.

**Note:** The goal is to have this GUI contribution merged into the original repository. Once merged, users should use the official repository for the most up-to-date version.

## Links

- **Original Repository:** https://github.com/iliazeus/iso2god-rs
- **This Fork:** https://github.com/lantzmurray/iso2god-rs-gui
- **Original CLI:** https://github.com/eliecharra/iso2god-cli

## Contact

For issues or questions specifically about the GUI:
- Open an issue in this fork repository
- Mention @lantzmurray in the issue

For issues with the core iso2god functionality:
- Check the original repository at https://github.com/iliazeus/iso2god-rs
- Open an issue there if needed

---

**@iliazeus** - Thank you for creating such a great tool! I hope this GUI addition is useful to the community and worthy of being merged into your excellent project.
