use ml_cnp::algo::{ColorAlgorithm, HeuristicColoring};
use ml_cnp::build_graph_from_str;

#[test]
fn test_heuristic() {
    let graph = build_graph_from_str(include_str!("easy.txt"));
    let mut algo = HeuristicColoring::create(2, graph.clone());
    let res = algo.color();
    assert_eq!(res, None);

    let mut algo = HeuristicColoring::create(3, graph.clone());
    let res = algo.color();
    assert!(res.is_some());
    assert!(algo.validate(&res.unwrap()));
}
