mod heuristic;
mod naive;

use crate::{Color, Graph};
pub use heuristic::HeuristicColoring;
pub use naive::NaiveColoring;

pub trait ColorAlgorithm<G>
where
    G: Graph,
{
    /// Attempt to color the graph using `num` colors.
    /// Returns the vector of colors if successful, otherwise [`None`].
    fn color(&mut self, num: usize) -> Option<Vec<Color>>;

    /// Validate the coloring.
    fn validate(&self, color: &[Color]) -> bool {
        for (from, to) in self.graph().edges() {
            if color[from] == color[to] {
                return false;
            }
        }
        true
    }

    fn graph(&self) -> &G;
}
