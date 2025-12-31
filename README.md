# iso2god-rs
A tool to convert Xbox 360 and original Xbox ISOs into an Xbox 360 compatible Games-On-Demand file format

This is an optimized rewrite of https://github.com/eliecharra/iso2god-cli, with a few extra features.

## Features

- **Command-line interface**: Fast and efficient conversion with multi-threaded processing
- **Graphical User Interface**: Easy-to-use GUI for users who prefer a visual interface

## Command-line Usage

```
Usage: iso2god [OPTIONS] <SOURCE_ISO> <DEST_DIR>

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

## GUI Usage

A graphical user interface is also available for users who prefer a visual interface. See the [gui](./gui) directory for more information.

To run the GUI:

```bash
cargo run --bin iso2god-gui
```

Or build and run the release binary:

```bash
cargo build --release --bin iso2god-gui
./target/release/iso2god-gui
```

The GUI provides:
- File selection dialogs for source ISO and destination directory
- Optional game title input
- Trim mode selection (trim unused space or keep full ISO)
- Real-time progress tracking during conversion
- Error reporting and success notifications
