pub struct TSPIndividual {
    pub genotype: Vec<usize>,
    pub fitness: f64,
}

pub trait Individual {
}

impl Individual for TSPIndividual {
}