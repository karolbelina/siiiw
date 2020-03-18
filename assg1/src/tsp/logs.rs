use crate::log::Log;

pub struct Discoverer {
    best_solution: Option<(Vec<usize>, u32)>,
}

impl Log<(Vec<usize>, u32)> for Discoverer {
    fn log(&mut self, value: &(Vec<usize>, u32)) {
        let (_, measure) = value;
        match &self.best_solution {
            Some((_, best_measure)) if measure > best_measure => (),
            _ => self.best_solution = Some(value.to_owned())
        }
    }
}

impl Discoverer {
    pub fn new() -> Discoverer {
        Discoverer {
            best_solution: None,
        }
    }

    pub fn get_best_solution(&self) -> Option<Vec<usize>> {
        return self.best_solution.to_owned().map(|(solution, _)| solution);
    }

    pub fn get_best_fitness(&self) -> Option<u32> {
        return self.best_solution.to_owned().map(|(_, fitness)| fitness);
    }
}

pub struct Cohorter {
    logged: usize,
    generation_size: usize,
    generations: usize,
    currents: Option<(u32, u32)>,
    current_bests: Vec<u32>,
    current_worsts: Vec<u32>,
    accumulated_bests: Vec<Vec<u32>>,
    worsts: Option<Vec<u32>>,
}

impl Log<(Vec<usize>, u32)> for Cohorter {
    fn log(&mut self, value: &(Vec<usize>, u32)) {
        let (_, measure) = value;
        match &self.currents {
            Some((current_best, current_worst)) => {
                let best = if measure < current_best { measure } else { current_best };
                let worst = if measure > current_worst { measure } else { current_worst };
                self.currents = Some((*best, *worst));
            },
            None => self.currents = Some((*measure, *measure))
        }
        self.logged += 1;
        if self.logged == self.generation_size {
            let (current_best, current_worst) = self.currents.unwrap();
            self.current_bests.push(current_best);
            self.current_worsts.push(current_worst);
            self.logged = 0;
            self.currents = None;
        }
    }
}

use quicli::prelude::*;
use std::path::PathBuf;

impl Cohorter {
    pub fn new(generation_size: usize, generations: usize) -> Cohorter {
        Cohorter {
            logged: 0,
            generation_size: generation_size,
            generations: generations,
            currents: None,
            current_bests: Vec::new(),
            current_worsts: Vec::new(),
            accumulated_bests: Vec::new(),
            worsts: None,
        }
    }

    pub fn carry(&mut self) {
        assert_eq!(self.current_bests.len(), self.generations);
        assert_eq!(self.current_worsts.len(), self.generations);

        self.accumulated_bests.push(self.current_bests.drain(0..).collect());
        self.worsts = match &self.worsts {
            Some(worsts) => {
                let worsts = Some(worsts.iter().zip(self.current_worsts.iter())
                    .map(|(worst, current_worst)| {
                        if current_worst > worst {
                            *current_worst
                        } else {
                            *worst
                        }
                    }).collect());
                self.current_worsts.clear();
                worsts
            },
            None => Some(self.current_worsts.drain(0..).collect())
        };
    }

    pub fn dump(&self, path: &PathBuf) -> Result<(), Error> {
        let bests: Vec<u32> = (0..self.generations).map(|i| self.accumulated_bests.iter()
                .min_by(|a, b| a[i].cmp(&b[i])).unwrap()[i])
            .collect();
        let avgs: Vec<f64> = (0..self.generations).map(|i| self.accumulated_bests.iter()
                .map(|accumulated_best| accumulated_best[i])
                .sum::<u32>() as f64 / self.accumulated_bests.len() as f64)
            .collect();
        let worsts: &Vec<u32> = &self.worsts.as_ref().unwrap();
        let output = bests.iter().zip(avgs.iter()).zip(worsts.iter())
            .enumerate()
            .map(|(i, ((best, avg), worst))| {
            format!("{};{};{};{}", i, best, avg, worst)
        })
        .collect::<Vec<String>>()
        .join("\n");

        write_to_file(&path, &output)
    }
}

// pub struct Logger<P: Problem> {
//     best: Option<P::Measure>,
//     worst: Option<P::Measure>,
//     mean: f64,
//     M2: f64,
//     count: usize,
// }

// impl<P: Problem> Log<P> for Logger<P> {
//     fn log(&mut self, value: &P::Measure) {
//         self.count += 1;
//         let val: f64 = value.into();
//         let delta: f64 = value.into() - self.mean;
//         self.mean += delta / self.count as f64;
//         let delta2: f64 = value.into() - self.mean;
//         self.M2 += delta * delta2;
//     }
// }

// impl<P: Problem> Logger<P> {
//     pub fn new() -> Logger<P> {
//         Logger {
//             best: None,
//             worst: None,
//             mean: 0.0,
//             M2: 0.0,
//             count: 0,
//         }
//     }

//     pub fn finalize(&self) -> Option<(f64, f64, f64, f64)> {
//         if self.count != 0 {
//             Some((
//                 self.best.unwrap().into(),
//                 self.worst.unwrap().into(),
//                 self.mean,
//                 (self.M2 / self.count as f64).sqrt()
//             ))
//         } else {
//             None
//         }
//     }
// }
