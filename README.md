# tmenu

[![Crates.io](https://img.shields.io/crates/v/tmenu.svg)](https://crates.io/crates/tmenu)

A simple command-line fuzzy finder application written in Rust using the `ratatui` library, inspired by `dmenu`.
It reads newline-separated options from standard input,
allows filtering and selection using a terminal user interface (TUI),
and prints the selected option to standard output.

![tmenu.gif](https://github.com/user-attachments/assets/cdc8ef46-84bf-4ff9-9d17-23f2f97982e0)

## Installation

You can install `tmenu` either via `cargo install` from crates.io or by building from source.

### Via cargo install (Recommended)

```bash
cargo install tmenu
```

### Building from source

```bash
git clone https://github.com/m1dsolo/tmenu.git
cd tmenu
cargo build --release
```

The executable will be located at `./target/release/tmenu`.
You can manually copy it to a directory in your system's PATH
or run it directly using cargo run --release.

## Usage

Pipe a list of newline-separated strings to the standard input of the program.
The TUI will appear on your terminal's standard error stream,
allowing you to interactively select an item.
The selected item will be printed to standard output when you press `Enter`.

Key bindings:
- `Ctrl+j` to move down the list
- `Ctrl+k` to move up the list
- `Enter` to select an item
- `Esc` to exit without selection

## Examples

```bash
# Example 1: List files in current directory
ls -1 | tmenu

# Example 2: Find files in the current directory (max depth 1)
find . -maxdepth 1 | tmenu

# Example 3: Filter a list from a file
cat your_list.txt | tmenu
```

## TODO

- vim keybindings for input
- faster filtering
- more examples

## License

[MIT](LICENSE) © m1dsolo
