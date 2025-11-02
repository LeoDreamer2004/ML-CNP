use ml_cnp::algo::{ColorAlgorithm, HeuristicColoring};
use ml_cnp::constants::{DIST, INIT_POINTS};
use ml_cnp::graph::VecVecGraph;
use ml_cnp::search::{next_layer, prune_layer};
use rayon::prelude::*;
use std::rc::Rc;

fn main() {
    let layer1 = next_layer(&[INIT_POINTS.to_vec()]);
    let layer2 = next_layer(&layer1);
    let layer3 = next_layer(&layer2);
    let layer4 = next_layer(&layer3);
    let layer4 = prune_layer(layer4, 10);
    let layer5 = next_layer(&layer4);

    println!("{}", layer5.len());

    layer5.par_iter().for_each(|graph| {
        let graph = VecVecGraph::from_points(graph, DIST);
        let mut algo = HeuristicColoring::create(5, Rc::new(graph));
        let colors = algo.color();

        if colors.is_none() {
            println!("No solution found.");
        }
    });

    // TODO:
    // 1. Add heuristic to find the best point sets
    // 2. Check if the new point sets can be colored
}
