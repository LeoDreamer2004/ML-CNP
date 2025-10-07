use crate::Node;

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
