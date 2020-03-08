use quicli::prelude::*;

pub struct DistanceMatrix {
    internal: Vec<Vec<f64>>,
    size: usize,
}

impl DistanceMatrix {
    pub fn new<T, F: Fn(&T, &T) -> f64>(nodes: &Vec<T>, distance_fn: F) -> Self {
        if nodes.is_empty() {
            return DistanceMatrix {
                internal: Vec::new(),
                size: 0,
            };
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
        return DistanceMatrix {
            internal: matrix,
            size: size,
        };
    }

    pub fn get(&self, from: usize, to: usize) -> Option<&f64> {
        if from == to {
            Some(&0.0)
        } else {
            let (lower, greater) = if from > to {
                (to, from)
            } else {
                (from, to)
            };
            self.internal.get(lower)
                .and_then(|vector| vector.get(self.size - 1 - greater))
        }
    }

    pub fn get_adjacent(&self, from: usize) -> Vec<(usize, &f64)> {
        let mut horizontal = self.internal.iter()
            .take(from)
            .enumerate()
            .map(|(i, vertical)| vertical.get(from - 1 - i)
                .map(|distance| (i, distance)))
            .collect::<Option<Vec<(usize, &f64)>>>()
            .unwrap_or(Vec::new());
        let mut vertical = self.internal.get(from)
            .map(|vertical| vertical.iter()
                .enumerate()
                .map(|(i, distance)| (self.size - 1 - i, distance))
                .rev()
                .collect())
            .unwrap_or(Vec::new());
        horizontal.append(&mut vertical);
        return horizontal;
    }
}
