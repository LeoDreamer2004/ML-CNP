use crate::{Color, Graph, Node};
use std::collections::HashSet;

pub trait ColorAlgorithm<G>
where
    G: Graph,
{
    /// Attempt to color the graph using `num` colors.
    /// Returns the vector of colors if successful, otherwise [`None`].
    fn color(num: usize, graph: &G) -> Option<Vec<Color>>;
}

pub struct NaiveColoring {
    available: Vec<HashSet<Color>>,
    colors: Vec<Option<Color>>,
}

impl NaiveColoring {
    fn create<G: Graph>(color_num: usize, graph: &G) -> Self {
        Self {
            available: vec![(0..color_num).collect(); graph.size()],
            colors: vec![None; graph.size()],
        }
    }

    fn color_on<G: Graph>(&mut self, node: Node, graph: &G) -> bool {
        if node == graph.size() {
            return true;
        }

        for color in self.available[node].clone() {
            self.colors[node] = Some(color);
            for &neighbor in graph.neighbors(node) {
                self.available[neighbor].remove(&color);
            }

            if self.color_on(node + 1, graph) {
                return true;
            }

            for &neighbor in graph.neighbors(node) {
                self.available[neighbor].insert(color);
            }
        }

        false
    }
}

impl<G: Graph> ColorAlgorithm<G> for NaiveColoring {
    fn color(color_num: usize, graph: &G) -> Option<Vec<Color>> {
        let mut algo = Self::create(color_num, graph);
        algo.color_on(0, graph);
        algo.colors.into_iter().collect()
    }
}
