use crate::{Color, ColorAlgorithm, Graph, Node};
use nohash::IntSet;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub struct HeuristicColoring<G: Graph> {
    graph: Rc<G>,
    colors: Vec<Option<Color>>,
    domains: Vec<IntSet<Color>>,
}

impl<G: Graph> HeuristicColoring<G> {
    pub fn create(color_num: usize, graph: Rc<G>) -> Self {
        let size = graph.size();
        let domains = vec![(0..color_num).collect(); size];

        Self {
            graph,
            colors: vec![None; size],
            domains,
        }
    }

    /// Check if a node is already colored
    fn colored(&self, node: Node) -> bool {
        self.colors[node].is_some()
    }

    /// Order colors using LCV (Least Constraining Value)
    fn order_colors(&self, node: Node, unused_colors: &IntSet<Color>) -> Vec<Color> {
        let mut domain: Vec<_> = if unused_colors.is_empty() {
            self.domains[node].iter().cloned().collect()
        } else {
            // we only need to search ONE unused color, because all unused colors have the same impact
            let unused = *unused_colors.iter().next().unwrap();
            // self.domains[node] - unused_colors + unused
            self.domains[node]
                .difference(unused_colors)
                .cloned()
                .chain(std::iter::once(unused))
                .collect()
        };

        domain.sort_by_key(|color| {
            self.graph
                .neighbors(node)
                .iter()
                .filter(|&&neighbor| !self.colored(neighbor))
                .filter(|&&neighbor| self.domains[neighbor].contains(color))
                .count()
        });
        domain
    }

    fn backtrack(&mut self, removals: HashMap<Node, HashSet<Color>>) {
        for (node, colors) in removals {
            self.domains[node].extend(colors);
        }
    }

    /// Forward Checking: After assigning a color to a node, remove that color from the domains of its uncolored neighbors.
    fn forward_check(&mut self, node: Node, color: Color) -> (bool, HashMap<Node, HashSet<Color>>) {
        let mut removals = HashMap::new();

        for &neighbor in self.graph.neighbors(node) {
            if !self.colored(neighbor) && self.domains[neighbor].contains(&color) {
                removals
                    .entry(neighbor)
                    .or_insert_with(HashSet::new)
                    .insert(color);

                self.domains[neighbor].remove(&color);

                if self.domains[neighbor].is_empty() {
                    // backtrack
                    self.backtrack(removals);
                    return (false, HashMap::new());
                }
            }
        }

        (true, removals)
    }

    fn search(
        &mut self,
        node: Node,
        unused_colors: &mut IntSet<Color>,
        unused_nodes: &mut Vec<Node>,
    ) -> bool {
        if unused_nodes.is_empty() {
            return true;
        }

        let ordered_colors = self.order_colors(node, unused_colors);

        for color in ordered_colors {
            // if some neighbor has the same color, skip
            if self
                .graph
                .neighbors(node)
                .iter()
                .any(|&neighbor| self.colors[neighbor] == Some(color))
            {
                continue;
            }

            let (avail, removals) = self.forward_check(node, color);

            if !avail {
                continue;
            }
            let removed = unused_colors.remove(&color);
            self.colors[node] = Some(color);

            unused_nodes.sort_by_key(|node| -(self.domains[*node].len() as isize));
            let next = unused_nodes.last().cloned().unwrap();
            unused_nodes.remove(unused_nodes.len() - 1);

            if self.search(next, unused_colors, unused_nodes) {
                return true;
            }
            self.colors[node] = None;
            if removed {
                unused_colors.insert(color);
            }
            unused_nodes.push(next);

            self.backtrack(removals);
        }

        false
    }
}

impl<G: Graph> ColorAlgorithm<G> for HeuristicColoring<G> {
    fn color(&mut self, color_num: usize) -> Option<Vec<Color>> {
        let size = self.graph.size();
        self.search(0, &mut (0..color_num).collect(), &mut (0..size).collect());
        self.colors.clone().into_iter().collect()
    }

    fn graph(&self) -> &G {
        &self.graph
    }
}
