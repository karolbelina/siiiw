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

    impl Tournament {
        pub fn new(tour_size: usize) -> Tournament {
            Tournament {
                tour_size: tour_size,
            }
        }
    }

    impl Select for Tournament {
        type Problem = TSP;

        fn select<'a>(&self, population: &'a Vec<Individual<TSP>>) -> &'a Individual<TSP> {
            use rand::seq::SliceRandom;

            let mut tournament: Vec<&Individual<TSP>> = Vec::new();
            for _ in 0..self.tour_size {
                tournament.push(population.choose(&mut rand::thread_rng()).unwrap());
            }
            return tournament.iter().min_by(|a, b| a.fitness.cmp(&b.fitness)).unwrap();
        }
    }

    pub struct RouletteWheel;

    impl Select for RouletteWheel {
        type Problem = TSP;

        fn select<'a>(&self, population: &'a Vec<Individual<TSP>>) -> &'a Individual<TSP> {
            use rand::distributions::{Distribution, WeightedIndex};
            
            let distribution = WeightedIndex::new(
                population.iter().map(|individual| 1.0 / individual.fitness as f64)
            ).unwrap();
            return population.get(distribution.sample(&mut rand::thread_rng())).unwrap();
        }
    }
}

pub mod crossover {
    use crate::ea::{Individual, Crossover};
    use std::borrow::Cow;
    use super::super::TSP;

    pub struct OX<'a> {
        problem: &'a TSP,
        probability: f64,
    }

    impl OX<'_> {
        pub fn new<'a>(problem: &'a TSP, probability: f64) -> OX<'a> {
            OX {
                problem: problem,
                probability: probability,
            }
        }
    }

    impl Crossover for OX<'_> {
        type Problem = TSP;

        fn crossover<'a>(&self, a: &'a Individual<TSP>, b: &'a Individual<TSP>)
            -> Cow<'a, Individual<TSP>>
        {
            use rand::Rng;
            use crate::problem::Problem;

            assert_eq!(a.genotype.len(), b.genotype.len(), "mismatched genotype lengths");

            if rand::thread_rng().gen_range(0.0, 1.0) < self.probability {
                use rand::distributions::{Distribution, Uniform};

                let distribution = Uniform::from(0..a.genotype.len());
                let first = distribution.sample(&mut rand::thread_rng());
                let second = distribution.sample(&mut rand::thread_rng());

                let (lower, greater) = if first > second {
                    (second, first)
                } else {
                    (first, second)
                };

                let subsequence = &a.genotype[lower..=greater];
                let mut genotype = b.genotype.clone();
                genotype.retain(|x| !subsequence.contains(&x));

                let mut tail = genotype.split_off(lower);
                genotype.extend_from_slice(subsequence);
                genotype.append(&mut tail);

                return Cow::Owned(Individual {
                    fitness: self.problem.fitness(&genotype),
                    genotype: genotype,
                });
            } else {
                return Cow::Borrowed(a);
            }
        }
    }
}

pub mod mutate {
    use crate::ea::{Individual, Mutate};
    use std::borrow::Cow;
    use super::super::TSP;

    pub struct Swap<'a> {
        problem: &'a TSP,
        probability: f64,
    }

    impl Swap<'_> {
        pub fn new<'a>(problem: &'a TSP, probability: f64) -> Swap<'a> {
            Swap {
                problem: problem,
                probability: probability,
            }
        }
    }

    impl Mutate for Swap<'_> {
        type Problem = TSP;

        fn mutate<'a>(&self, individual: &'a Individual<TSP>) -> Cow<'a, Individual<TSP>> {
            use rand::Rng;
            use rand::distributions::{Distribution, Uniform};
            use crate::problem::Problem;

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
                });
            } else {
                return Cow::Borrowed(individual);
            }
        }
    }

    pub struct Inversion<'a> {
        problem: &'a TSP,
        probability: f64,
    }

    impl Inversion<'_> {
        pub fn new<'a>(problem: &'a TSP, probability: f64) -> Inversion<'a> {
            Inversion {
                problem: problem,
                probability: probability,
            }
        }
    }

    impl Mutate for Inversion<'_> {
        type Problem = TSP;

        fn mutate<'a>(&self, individual: &'a Individual<TSP>) -> Cow<'a, Individual<TSP>> {
            use rand::Rng;
            use crate::problem::Problem;

            if rand::thread_rng().gen_range(0.0, 1.0) < self.probability {
                use rand::distributions::{Distribution, Uniform};

                let distribution = Uniform::from(0..individual.genotype.len());
                let first = distribution.sample(&mut rand::thread_rng());
                let second = distribution.sample(&mut rand::thread_rng());

                if first != second {
                    let (lower, greater) = if first > second {
                        (second, first)
                    } else {
                        (first, second)
                    };
                    let mut genotype = individual.genotype.clone();
                    genotype[lower..=greater].reverse();
                    return Cow::Owned(Individual {
                        fitness: self.problem.fitness(&genotype),
                        genotype: genotype,
                    });
                } else {
                    return Cow::Borrowed(individual);
                }
            } else {
                return Cow::Borrowed(individual);
            }
        }
    }
}
