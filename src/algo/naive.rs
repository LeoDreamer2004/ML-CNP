use crate::{Color, ColorAlgorithm, Graph, Node};
use std::rc::Rc;

/// A naive backtracking algorithm for graph coloring.
/// It tries to color the graph node by node, backtracking when a conflict arises.
pub struct NaiveColoring<G: Graph> {
    color_num: usize,
    graph: Rc<G>,
    colors: Vec<Option<Color>>,
}

impl<G: Graph> NaiveColoring<G> {
    pub fn create(color_num: usize, graph: Rc<G>) -> Self {
        let colors = vec![None; graph.size()];
        Self {
            color_num,
            graph,
            colors,
        }
    }

    fn search(&mut self, node: Node) -> bool {
        if node == self.graph.size() {
            return true;
        }

        for color in 0..self.color_num {
            if self
                .graph
                .neighbors(node)
                .iter()
                .any(|&n| self.colors[n] == Some(color))
            {
                continue;
            }

            self.colors[node] = Some(color);
            if self.search(node + 1) {
                return true;
            }
            self.colors[node] = None;
        }

        false
    }
}

impl<G: Graph> ColorAlgorithm<G> for NaiveColoring<G> {
    fn color(&mut self, _color_num: usize) -> Option<Vec<Color>> {
        self.search(0);
        self.colors.clone().into_iter().collect()
    }

    fn graph(&self) -> &G {
        &self.graph
    }
}
