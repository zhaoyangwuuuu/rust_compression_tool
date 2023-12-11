use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

// Define HuffmanNode struct

pub struct HuffmanNode {
    byte: Option<u8>,
    freq: usize,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

impl HuffmanNode {
    fn new() -> Self {
        HuffmanNode {
            byte: None,
            freq: 0,
            left: None,
            right: None,
        }
    }
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

pub fn decode_data(encoded_data: &[u8], huffman_tree: &HuffmanNode) -> Vec<u8> {
    let mut current_node = huffman_tree;
    let mut decoded_data = Vec::new();

    for &byte in encoded_data {
        for i in (0..8).rev() {
            let bit = (byte >> i) & 1;

            current_node = if bit == 0 {
                current_node.left.as_ref().unwrap()
            } else {
                current_node.right.as_ref().unwrap()
            };

            if let Some(value) = current_node.byte {
                decoded_data.push(value);
                current_node = huffman_tree; // Reset to start of the tree for next character
            }
        }
    }

    decoded_data
}

pub fn build_tree_from_codes(codes: &HashMap<u8, Vec<u8>>) -> Option<Box<HuffmanNode>> {
    let mut root = Box::new(HuffmanNode::new_node(
        0,
        HuffmanNode::new(),
        HuffmanNode::new(),
    ));

    for (&byte, code) in codes {
        let mut current = &mut root;
        for &bit in code {
            current = if bit == 0 {
                if current.left.is_none() {
                    current.left = Some(Box::new(HuffmanNode::new_node(
                        0,
                        HuffmanNode::new(),
                        HuffmanNode::new(),
                    )));
                }
                current.left.as_mut().unwrap()
            } else {
                if current.right.is_none() {
                    current.right = Some(Box::new(HuffmanNode::new_node(
                        0,
                        HuffmanNode::new(),
                        HuffmanNode::new(),
                    )));
                }
                current.right.as_mut().unwrap()
            };
        }
        current.byte = Some(byte);
    }

    Some(root)
}
