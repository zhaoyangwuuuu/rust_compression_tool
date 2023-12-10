mod huffman;

use huffman::{build_tree, encode_data, generate_codes};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};

fn main() -> io::Result<()> {
    // TODO: user input, and extract file
    let filename = "input.txt";
    let output_filename = "output.bin";

    // Read the file contents into a string
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Count the frequency of each byte
    let frequencies = count_frequencies(&contents);

    // Build the Huffman tree
    let huffman_tree = build_tree(&frequencies);

    // Generate Huffman codes
    let mut codes = HashMap::new();
    generate_codes(&huffman_tree, vec![], &mut codes);

    // Encode the data
    let encoded_data = encode_data(&contents, &codes);

    // Write the compress data to a file
    let mut output = File::create(output_filename)?;
    output.write_all(&encoded_data)
}

fn count_frequencies(contents: &str) -> HashMap<u8, usize> {
    let mut frequencies = HashMap::new();
    for byte in contents.bytes() {
        let count = frequencies.entry(byte).or_insert(0);
        *count += 1;
    }
    frequencies
}
