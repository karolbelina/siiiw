mod individual;
mod problem;

use problem::{TSPInstance, Euclidean};

#[derive(Debug, PartialEq)]
struct Config {
    problem: String,
}

fn main() {
    TSPInstance::<Euclidean> {
        name: None,
        comment: None,
        dimension: 11,
        nodes: Vec::new(),
    };
}
