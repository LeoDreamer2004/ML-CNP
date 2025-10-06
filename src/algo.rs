mod heuristic;
mod naive;

use crate::{Color, Graph};
pub use heuristic::HeuristicColoring;
pub use naive::NaiveColoring;
use std::rc::Rc;

pub trait ColorAlgorithm<G>
where
    G: Graph,
{
    /// Attempt to color the graph using `num` colors.
    /// Returns the vector of colors if successful, otherwise [`None`].
    fn color(num: usize, graph: Rc<G>) -> Option<Vec<Color>>;
}
