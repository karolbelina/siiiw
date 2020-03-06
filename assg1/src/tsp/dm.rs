use quicli::prelude::*;

pub struct DistanceMatrix(Vec<Vec<f64>>);

impl DistanceMatrix {
    pub fn new<T, F: Fn(&T, &T) -> f64>(nodes: &Vec<T>, distance_fn: F) -> Self {
        if nodes.is_empty() {
            return DistanceMatrix(Vec::new());
        }
        let size = nodes.len();
        let mut matrix: Vec<Vec<f64>> = Vec::new();
        for x in 0..size - 1 {
            let mut vector: Vec<f64> = Vec::new();
            for y in 0..size - x - 1 {
                vector.push(distance_fn(&nodes[x], &nodes[size - y - 1]));
            }
            matrix.push(vector);
        }
        info!("created the distance matrix");
        return DistanceMatrix(matrix);
    }

    pub fn get(&self, from: usize, to: usize) -> Option<f64> {
        if from == to {
            Some(0.0)
        } else {
            let (lower, greater) = if from > to {
                (to, from)
            } else {
                (from, to)
            };
            self.0.get(lower)
                .and_then(|vector| vector.get(greater))
                .map(|ref_distance| *ref_distance)
        }
    }
}