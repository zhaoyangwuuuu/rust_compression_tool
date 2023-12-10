mod huffman;

use huffman::{build_tree, encode_data, generate_codes};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let output_filename = format!("{}.compressed", filename);

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
    let mut output = File::create(&output_filename)?;
    output.write_all(&encoded_data)?;

    println!("File compressed successfully: {}", output_filename);

    Ok(())
}

fn count_frequencies(contents: &str) -> HashMap<u8, usize> {
    let mut frequencies = HashMap::new();
    for byte in contents.bytes() {
        let count = frequencies.entry(byte).or_insert(0);
        *count += 1;
    }
    frequencies
}
