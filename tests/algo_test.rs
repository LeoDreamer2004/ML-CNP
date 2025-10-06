use ml_cnp::*;
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
    let mut graph = ml_cnp::VecVecGraph::new(size);
    for line in lines {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() == 2 {
            let from: usize = parts[0].parse().expect("Invalid node index");
            let to: usize = parts[1].parse().expect("Invalid node index");
            graph.add_edge(from, to);
        }
    }
    Rc::new(graph)
}

#[test]
fn test_graph() {
    let graph = build_graph_from_str(include_str!("easy.txt"));
    assert_eq!(graph.size(), 5);
    assert_eq!(graph.neighbors(0), &[1, 4]);
    assert_eq!(graph.neighbors(1), &[0, 2]);
    assert_eq!(graph.neighbors(2), &[1, 3]);
}

#[test]
fn test_naive() {
    let graph = build_graph_from_str(include_str!("easy.txt"));
    let res = NaiveColoring::color(2, graph.clone());
    assert_eq!(res, None);

    let res = NaiveColoring::color(3, graph);
    assert!(matches!(res, Some(_)));
}

#[test]
fn test_heuristic() {
    let graph = build_graph_from_str(include_str!("easy.txt"));
    let res = HeuristicColoring::color(2, graph.clone());
    assert_eq!(res, None);

    let res = HeuristicColoring::color(3, graph);
    assert!(matches!(res, Some(_)));
}
