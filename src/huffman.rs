use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

// Define HuffmanNode struct

#[derive(Debug)]
struct HuffmanNode {
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
