mod dm;
pub mod reader;

use dm::DistanceMatrix;

pub struct ProblemInstance {
    pub name: Option<String>,
    pub comment: Option<String>,
    pub dimension: usize,
    pub dm: DistanceMatrix,
}
