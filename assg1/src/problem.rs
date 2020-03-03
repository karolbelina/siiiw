use super::individual::{Individual, TSPIndividual};

trait Problem<T: Individual> {
    fn fitness(&self, individual: T) -> f64;
}

pub trait CoordinateSystem {
    fn length(a: &Self, b: &Self) -> f64;
}

pub struct Euclidean {
    x: f64,
    y: f64,
}

pub struct Geographical {
    latitude: f64,
    longitude: f64,
}

impl CoordinateSystem for Euclidean {
    fn length(a: &Euclidean, b: &Euclidean) -> f64 {
        ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
    }
}

impl CoordinateSystem for Geographical {
    fn length(a: &Geographical, b: &Geographical) -> f64 {
        // $$D = R\sqrt{(\delta\phi)^2 + (\cos{\phi_m}\delta\lambda)^2}$$
        // where $\phi$ is the latitude, $\lambda$ is the longitude, $R = 1$,
        // and $\phi_m$ is the mean latitude of the two points
        ((a.latitude - b.latitude).powi(2) + (((a.latitude + b.latitude) / 2.0).cos() * (a.longitude - b.longitude)).powi(2)).sqrt()
    }
}

pub struct TSPInstance<T: CoordinateSystem> {
    pub name: Option<String>,
    pub comment: Option<String>,
    pub dimension: usize,
    pub nodes: Vec<T>,
}

impl<T: CoordinateSystem> Problem<TSPIndividual> for TSPInstance<T> {
    fn fitness(&self, individual: TSPIndividual) -> f64 {
        let mut length: f64 = 0.0;

        for i in 0..self.dimension - 1 {
            let a_index = individual.genotype[i];
            let b_index = individual.genotype[i + 1];

            length += T::length(&self.nodes[a_index], &self.nodes[b_index]);
        }

        return length;
    }
}
