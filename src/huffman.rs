use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

// Define HuffmanNode struct

#[derive(Debug)]
pub struct HuffmanNode {
    byte: Option<u8>,
    freq: usize,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

impl HuffmanNode {
    fn new_leaf(byte: u8, freq: usize) -> Self {
        HuffmanNode {
            byte: Some(byte),
            freq,
            left: None,
            right: None,
        }
    }

    fn new_node(freq: usize, left: HuffmanNode, right: HuffmanNode) -> Self {
        HuffmanNode {
            byte: None,
            freq,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
}

impl Eq for HuffmanNode {}

impl PartialEq for HuffmanNode {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.freq.cmp(&self.freq)
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn build_tree(frequencies: &HashMap<u8, usize>) -> Option<Box<HuffmanNode>> {
    let mut heap = BinaryHeap::new();

    for (&byte, &freq) in frequencies.iter() {
        heap.push(Box::new(HuffmanNode::new_leaf(byte, freq)));
    }

    while heap.len() > 1 {
        let right = heap.pop().unwrap();
        let left = heap.pop().unwrap();

        let merged = Box::new(HuffmanNode::new_node(left.freq + right.freq, *left, *right));
        heap.push(merged);
    }

    heap.pop()
}

pub fn generate_codes(
    node: &Option<Box<HuffmanNode>>,
    prefix: Vec<u8>,
    codes: &mut HashMap<u8, Vec<u8>>,
) {
    if let Some(node) = node {
        if let Some(byte) = node.byte {
            codes.insert(byte, prefix);
        } else {
            let mut left_prefix = prefix.clone();
            let mut right_prefix = prefix;

            left_prefix.push(0);
            right_prefix.push(1);

            generate_codes(&node.left, left_prefix, codes);
            generate_codes(&node.right, right_prefix, codes);
        }
    }
}
pub fn encode_data(contents: &str, codes: &HashMap<u8, Vec<u8>>) -> Vec<u8> {
    let mut encoded_data = Vec::new();
    for byte in contents.bytes() {
        if let Some(code) = codes.get(&byte) {
            encoded_data.extend(code);
        }
    }

    encoded_data
}
