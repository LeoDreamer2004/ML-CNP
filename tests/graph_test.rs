use ml_cnp::build_graph_from_str;
use ml_cnp::graph::Graph;

#[test]
fn test_graph() {
    let graph = build_graph_from_str(include_str!("easy.txt"));
    assert_eq!(graph.size(), 5);
    assert_eq!(graph.neighbors(0), &[1, 4]);
    assert_eq!(graph.neighbors(1), &[0, 2]);
    assert_eq!(graph.neighbors(2), &[1, 3]);
}
