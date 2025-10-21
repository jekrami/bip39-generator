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

## Python Module

This project can also be built as a Python-callable native module.

### Building and Installing

1.  **Install Maturin:**
    It is recommended to use `maturin` to build and install the module.
    ```sh
    pip install maturin
    ```

2.  **Build and Install the Module:**
    Run the following command from the `bip39_generator` directory:
    ```sh
    maturin develop
    ```
    This will build the module and install it in your current Python environment.

### Python Usage

Here is an example of how to use the module in Python:

```python
import bip39_generator_lib

# Generate a new mnemonic phrase
new_phrase = bip39_generator_lib.generate_mnemonic_py()
print(f"Generated Phrase: {new_phrase}")

# Validate an existing phrase
is_valid = bip39_generator_lib.validate_mnemonic_py(new_phrase)
print(f"Is the phrase valid? {is_valid}")

# Validate an invalid phrase
invalid_phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon"
is_valid = bip39_generator_lib.validate_mnemonic_py(invalid_phrase)
print(f"Is the invalid phrase valid? {is_valid}")
```
