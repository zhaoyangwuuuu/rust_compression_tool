use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
// use huffman::{build};

fn main() -> io::Result<()> {
    let filename = "input.txt";
    let output_filename = "output.bin";

    // Read the file contents into a string
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Count the frequency of each byte
    let frequencies = count_frequencies(&contents);
}

fn count_frequencies(contents: &str) -> HashMap<u8, usize> {
    let mut frequencies = HashMap::new();
    for byte in contents.bytes() {
        let count = frequencies.entry(byte).or_insert(0);
        *count += 1;
    }
    frequencies
}
