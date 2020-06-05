use crate::log::Log;

pub struct Discoverer {
    currents: Option<(u32, u32)>,
    bests: Vec<u32>,
    pub best_solution: Vec<usize>,
}

impl Log<(Vec<usize>, u32)> for Discoverer {
    fn log(&mut self, value: &(Vec<usize>, u32)) {
        let (solution, measure) = value;
        match &self.currents {
            Some((current_best, current_worst)) => {
                let best = if measure < current_best { self.best_solution = solution.clone(); measure } else { current_best };
                let worst = if measure > current_worst { measure } else { current_worst };
                self.currents = Some((*best, *worst));
            },
            None => self.currents = Some((*measure, *measure))
        }
    }
}

impl Discoverer {
    pub fn new() -> Discoverer {
        Discoverer {
            currents: None,
            bests: Vec::new(),
            best_solution: Vec::new(),
        }
    }

    pub fn carry(&mut self) {
        let (best, _) = self.currents.unwrap();
        self.bests.push(best);
    }

    pub fn print(&self) {
        let count = self.bests.len();
        let best: u32 = *self.bests.iter().min().unwrap();
        let worst: u32 = self.currents.unwrap().1;
        let avg: f64 = self.bests.iter().sum::<u32>() as f64 / count as f64;
        let variance = self.bests.iter().map(|value| {
            let diff = avg - (*value as f64);
            diff * diff
        }).sum::<f64>() / count as f64;
        let std = variance.sqrt();

        println!("{} & {} & {} & {} \\\\", best, worst, avg, std);
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
            // format!("{};", avg)
        })
        .collect::<Vec<String>>()
        .join("\n");

        write_to_file(&path, &output)
    }
}
