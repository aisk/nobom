# nobom

A simple utility to remove UTF-8 BOM (Byte Order Mark) from stdin and write to stdout.

## Usage

After installation:

```bash
echo -e "\xEF\xBB\xBFHello, world!" | nobom
```

Or with files:

```bash
nobom < file_with_bom.txt
```

### Development usage

```bash
cargo run < file_with_bom.txt
```

## Features

- Streaming processing - handles large files with minimal memory usage
- Removes UTF-8 BOM (EF BB BF) if present at the beginning
- Passes through data unchanged if no BOM is found
- Works with any text data

## Installation

### From crates.io (recommended)

```bash
cargo install nobom
```

### From source

```bash
git clone https://github.com/aisk/nobom.git
cd nobom
cargo build --release
```

The binary will be available at `target/release/nobom`.

## Example

```bash
# Create a file with BOM
printf '\xEF\xBB\xBFHello, world!' > test.txt

# Remove BOM (after installation)
nobom < test.txt > output.txt

# Or during development
cargo run < test.txt > output.txt
```
