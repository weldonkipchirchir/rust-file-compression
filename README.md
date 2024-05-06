# Rust File Compression

This Rust program allows you to compress and decompress files using the Gzip compression algorithm.

## Features

- **Compression**: Compress files using Gzip compression.
- **Decompression**: Decompress Gzip-compressed files.

## Prerequisites

- [Rust](https://www.rust-lang.org/) installed on your system.

## Installation

Clone this repository to your local machine:

```bash
git clone https://github.com/weldonkipchirchir/rust-file-compression.git
```

Navigate to the project directory:

```bash
cd rust-file-compression
```

## Usage

### Compression

To compress a file, run the following command:

```bash
cargo run --release -- -c <source_file>
```

Replace `<source_file>` with the path to the file you want to compress.

The compressed file will be saved with a `.gz` extension in the same directory as the source file.

### Decompression

To decompress a Gzip-compressed file, run the following command:

```bash
cargo run --release -- -d <source_file>
```

Replace `<source_file>` with the path to the Gzip-compressed file you want to decompress.

The decompressed file will be saved in the same directory as the source file without the `.gz` extension.

## Example

Compress a file named `example.txt`:

```bash
cargo run --release -- -c example.txt
```

Decompress the compressed file `example.txt.gz`:

```bash
cargo run --release -- -d example.txt.gz
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

