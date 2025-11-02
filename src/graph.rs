pub mod hash;

use crate::Node;
use crate::constants::EPS;
use approx::AbsDiffEq;
use nalgebra::Point3;

pub trait Graph {
    fn size(&self) -> usize;
    fn neighbors(&self, node: Node) -> &[Node];
    fn edges(&self) -> Vec<(Node, Node)>;
}

pub struct VecVecGraph {
    size: usize,
    edges: Vec<Vec<Node>>,
}

impl VecVecGraph {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            edges: vec![Vec::new(); size],
        }
    }

    pub fn add_edge(&mut self, from: Node, to: Node) {
        self.edges[from].push(to);
        self.edges[to].push(from);
    }

    pub fn from_points(points: &[Point3<f64>], dist: f64) -> Self {
        let mut graph = VecVecGraph::new(points.len());
        for i in 0..points.len() {
            for j in 0..i {
                if (points[i] - points[j]).norm().abs_diff_eq(&dist, EPS) {
                    graph.add_edge(i, j);
                }
            }
        }

        graph
    }
}

impl Graph for VecVecGraph {
    fn size(&self) -> usize {
        self.size
    }

    fn neighbors(&self, node: Node) -> &[Node] {
        &self.edges[node]
    }

    fn edges(&self) -> Vec<(Node, Node)> {
        let mut edges = vec![];
        for (from, neighbors) in self.edges.iter().enumerate() {
            for to in neighbors {
                edges.push((from, *to));
            }
        }
        edges
    }
}
