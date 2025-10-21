# BIP39 Mnemonic Seed Phrase Generator

This is a command-line tool written in Rust to generate and validate BIP39 mnemonic seed phrases.

## Features

- Generates cryptographically secure 12-word BIP39-compatible mnemonic seed phrases.
- Uses 128 bits of entropy and includes proper checksum validation.
- Reads the standard BIP39 English wordlist from `bip39-english.txt`.
- Configurable number of phrases to generate.
- Option to save generated phrases to a file.
- Mnemonic validation functionality.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) programming language toolchain.

## Building and Running

1.  Build the project:
    ```sh
    cargo build --release
    ```
    The executable will be located at `target/release/bip39_generator`.

2.  Run the tool:
    ```sh
    cargo run -- [OPTIONS]
    ```

## Usage

### Options

-   `-c, --count <COUNT>`: Sets the number of phrases to generate. (default: 100)
-   `-o, --output[=<OUTPUT>]`: Saves the generated phrases to a file.
    -   If no filename is provided (e.g., `--output`), a timestamped filename like `mnemonics_<timestamp>.txt` will be used.
    -   If a filename is provided (e.g., `--output=my_phrases.txt`), that file will be used.
-   `-v, --validate <VALIDATE>`: Validates a given mnemonic phrase. The phrase should be enclosed in quotes.
-   `-h, --help`: Prints help information.
-   `-V, --version`: Prints version information.

### Examples

**Generate 10 phrases and print them to the console:**
```sh
cargo run -- --count 10
```

**Generate 50 phrases and save them to a file named `my_seeds.txt`:**
```sh
cargo run -- --count 50 --output=my_seeds.txt
```

**Generate 20 phrases and save them to a default timestamped file:**
```sh
cargo run -- --count 20 --output
```

**Validate a mnemonic phrase:**
```sh
cargo run -- --validate "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
```
