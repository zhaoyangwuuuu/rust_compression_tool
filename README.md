# Huffman Coding Compression Tool

This project implements a file compression and decompression tool using Huffman coding, a well-known and widely used method for lossless data compression.

## Project Overview
This tool is developed as a small project for the sake of learning Rust. It serves as a practical example to understand how Rust can be used for implementing data compression algorithms and working with file I/O, data structures, and algorithms.

## Features

- **File Compression:** Compresses text files using Huffman coding.
- **File Decompression:** Decompresses files that were compressed using this tool.
- **Huffman Tree Construction:** Builds a Huffman tree based on byte frequencies in the input file for compression and reconstructs it from encoded data for decompression.

## Installation

To use this tool, you need to have Rust installed on your system. If you don't have Rust installed, please follow the instructions at [The Rust Programming Language website](https://www.rust-lang.org/learn/get-started).

## Usage

To compress a file:

```
cargo run -- compress <input_file>
```

This will create a compressed file named `<input_file>.compressed`.

To decompress a file:

```
cargo run -- extract <input_file>.compressed
```

This will create a decompressed file named `<input_file>.decompressed`.

## How It Works

The tool reads the input file and counts the frequency of each byte. It then builds a Huffman tree based on these frequencies. Each byte is assigned a unique binary code corresponding to the path from the root of the Huffman tree to the leaf node representing that byte. The file is then compressed using these codes.

For decompression, the tool reconstructs the Huffman tree from the encoded data and then uses this tree to decode the compressed file back to its original form.

## Limitations

- Currently, the tool is optimized for text files.
- The encoding table is stored in a simple format at the beginning of the compressed file, which may not be the most space-efficient method.

## Important Warning

- **In-Complete & Testing Needed:** This project is currently in an incomplete state and requires further development and testing. Users should be aware that the tool might not handle all edge cases and could produce unexpected results. Thorough testing and validation are highly recommended before using it in a production environment.

## Contributions

Contributions to this project are welcome. Please feel free to fork the repository and submit pull requests.

## License

This project is licensed under the [MIT License](LICENSE).
