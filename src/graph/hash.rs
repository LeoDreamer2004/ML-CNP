use crate::Node;
use crate::constants::WL_TEST_ROUNDS;
use crate::graph::Graph;
use nohash::IntMap;
use xxhash_rust::xxh3::xxh3_64;

/// A trait for a hashable graph.
pub trait Hashable<T> {
    /// Returns a hash of the graph.
    fn hash(&self) -> T;
}

impl<T: Graph> Hashable<u64> for T {
    fn hash(&self) -> u64 {
        let mut hashes = Vec::new();
        let mut labels: IntMap<Node, u64> = (0..self.size())
            .map(|node| (node, self.neighbors(node).len() as u64))
            .collect();

        for _ in 0..WL_TEST_ROUNDS {
            let mut new_labels = IntMap::default();
            for node in 0..self.size() {
                let mut new_label: Vec<u64> = self
                    .neighbors(node)
                    .iter()
                    .map(|neighbor| labels.get(neighbor).cloned().unwrap_or_default())
                    .chain([labels.get(&node).cloned().unwrap_or_default()])
                    .collect();
                new_label.sort();
                // transform new label to Vec<u8> and hash it
                let new_label: Vec<u8> = new_label.iter().flat_map(|&x| x.to_le_bytes()).collect();
                new_labels.insert(node, xxh3_64(&new_label));
            }

            labels = new_labels;
            let mut values: Vec<_> = labels.values().cloned().collect();
            values.sort();
            let values: Vec<u8> = values.iter().flat_map(|&x| x.to_le_bytes()).collect();
            hashes.extend_from_slice(&values);
        }

        xxh3_64(&hashes)
    }
}
