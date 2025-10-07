use ml_cnp::{ColorAlgorithm, HeuristicColoring, VecVecGraph};
use std::rc::Rc;

/// A simple parser for graph description in the following format:
/// - First line: number of nodes
/// - Subsequent lines: edges in the format "from to"
fn build_graph_from_str(desc: &str) -> Rc<VecVecGraph> {
    let mut lines = desc.lines();
    let size: usize = lines
        .next()
        .expect("Graph description must start with number of nodes")
        .trim()
        .parse()
        .expect("Invalid number of nodes");
    let mut graph = VecVecGraph::new(size);
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let from: usize = parts[0].parse().expect("Invalid node index");
            let to: usize = parts[1].parse().expect("Invalid node index");
            graph.add_edge(from, to);
        } else {
            panic!("Invalid edge description");
        }
    }
    Rc::new(graph)
}

fn main() {
    let graph = build_graph_from_str(include_str!("hard.txt"));
    let res = HeuristicColoring::color(6, graph);
    assert!(res.is_some());
}
