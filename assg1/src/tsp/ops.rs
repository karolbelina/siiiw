pub mod initialize {
    use crate::ea::{Individual, Initialize};
    use super::super::TSP;

    pub struct Random<'a> {
        problem: &'a TSP,
    }

    impl Random<'_> {
        pub fn new<'a>(problem: &'a TSP) -> Random<'a> {
            Random {
                problem: problem,
            }
        }
    }

    impl Initialize for Random<'_> {
        type Problem = TSP;

        fn initialize(&self, pop_size: usize) -> Vec<Individual<TSP>> {
            use rand::seq::SliceRandom;
            use crate::problem::Problem;

            let mut population: Vec<Individual<TSP>> = Vec::new();
            for _ in 0..pop_size {
                let mut genotype: Vec<usize> = (0..self.problem.dimension).collect();
                genotype.shuffle(&mut rand::thread_rng());

                let individual = Individual::<TSP> {
                    fitness: self.problem.fitness(&genotype),
                    genotype: genotype,
                };
                population.push(individual);
            }
            return population;
        }
    }
}

pub mod select {
    use crate::ea::{Individual, Select};
    use super::super::TSP;

    pub struct Tournament {
        tour_size: usize,
    }

    impl Select for Tournament {
        type Problem = TSP;

        fn select<'a>(&self, population: &'a Vec<Individual<TSP>>) -> &'a Individual<TSP> {
            use rand::seq::SliceRandom;
            use std::cmp::Ordering;

            let mut tournament: Vec<&Individual<TSP>> = Vec::new();
            for _ in 0..self.tour_size {
                tournament.push(population.choose(&mut rand::thread_rng()).unwrap());
            }
            return tournament.iter()
                .min_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap_or(Ordering::Equal))
                .unwrap();
        }
    }

    pub struct RouletteWheel;

    impl Select for RouletteWheel {
        type Problem = TSP;

        fn select<'a>(&self, population: &'a Vec<Individual<TSP>>) -> &'a Individual<TSP> {
            use rand::distributions::{Distribution, WeightedIndex};
            
            let distribution = WeightedIndex::new(
                population.iter().map(|individual| 1.0 / individual.fitness)
            ).unwrap();
            return population.get(distribution.sample(&mut rand::thread_rng())).unwrap();
        }
    }
}

pub mod mutate {
    use crate::ea::{Individual, Mutate};
    use std::borrow::Cow;
    use super::super::TSP;

    struct Swap<'a> {
        problem: &'a TSP,
        probability: f64,
    }

    impl Mutate for Swap<'_> {
        type Problem = TSP;

        fn mutate<'a>(&self, individual: &'a Individual<Self::Problem>)
            -> Cow<'a, Individual<Self::Problem>>
        {
            use rand::Rng;
            use crate::problem::Problem;

            if rand::thread_rng().gen_range(0.0, 1.0) < self.probability {
                use rand::distributions::{Distribution, Uniform};

                let mut genotype = individual.genotype.clone();
                let distribution = Uniform::from(0..individual.genotype.len());
                for gene in 0..genotype.len() {
                    if rand::thread_rng().gen_range(0.0, 1.0) < self.probability {
                        let random_gene = distribution.sample(&mut rand::thread_rng());
                        genotype.swap(gene, random_gene);
                    }
                }
                if genotype != individual.genotype {
                    return Cow::Owned(Individual {
                        fitness: self.problem.fitness(&genotype),
                        genotype: genotype,
                    })
                } else {
                    return Cow::Borrowed(individual);
                }
            } else {
                return Cow::Borrowed(individual);
            }
        }
    }
}
