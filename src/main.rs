#![allow(dead_code)]
mod huffman;

use huffman::{build_tree, build_tree_from_codes, decode_data, encode_data, generate_codes};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use std::{env, vec};

//TODO: unit test
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // TODO add help
    if args.len() < 3 {
        eprintln!("Usage: <compress|extract> <filename>");
        std::process::exit(1);
    }

    let mode = &args[1];
    let filename = &args[2];

    match mode.as_str() {
        "compress" => compress(filename),
        "extract" => extract(filename),
        _ => {
            eprintln!("Unknown mode: {}", mode);
            std::process::exit(1);
        }
    }
}

fn count_frequencies(contents: &str) -> HashMap<u8, usize> {
    let mut frequencies = HashMap::new();
    for byte in contents.bytes() {
        let count = frequencies.entry(byte).or_insert(0);
        *count += 1;
    }
    frequencies
}

fn compress(filename: &str) -> io::Result<()> {
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
    let output_filename = format!("{}.compressed", filename);
    let mut output = File::create(&output_filename)?;

    let encoded_table = serialize_encoding_table(&codes)?;

    output.write_all(&encoded_table)?;
    output.write_all(&encoded_data)?;

    Ok(())
}

fn extract(filename: &str) -> io::Result<()> {
    let mut file = File::open(filename)?;
    let mut compressed_data = Vec::new();
    file.read_to_end(&mut compressed_data)?;

    let (codes, encoded_data) = deserialize_encoding_table(&compressed_data)?;

    let huffman_tree = build_tree_from_codes(&codes).expect("Failed to reconstruct Huffman tree");

    let decoded_data = decode_data(&encoded_data, &huffman_tree);

    let output_filename = format!("{}.decompressed", filename);
    let mut output = File::create(&output_filename)?;
    output.write_all(&decoded_data)?;

    Ok(())
}

fn serialize_encoding_table(codes: &HashMap<u8, Vec<u8>>) -> io::Result<Vec<u8>> {
    let mut encoded_table = Vec::new();
    for (&byte, code) in codes {
        encoded_table.push(byte);
        encoded_table.push(code.len() as u8);
        encoded_table.extend(code);
    }
    Ok(encoded_table)
}

fn deserialize_encoding_table(data: &[u8]) -> io::Result<(HashMap<u8, Vec<u8>>, &[u8])> {
    let mut codes = HashMap::new();
    let mut i = 0;
    while i < data.len() {
        let byte = data[i];
        let len = data[i] as usize;
        let code = data[i + 2..i + 2 + len].to_vec();
        codes.insert(byte, code);
        i += 2 + len;
    }
    Ok((codes, &data[i..]))
}
