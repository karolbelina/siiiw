use crate::logger::Log;

pub struct Discoverer {
    best_solution: Option<(Vec<usize>, f64)>,
}

impl Log<(Vec<usize>, f64)> for Discoverer {
    fn log(&mut self, value: &(Vec<usize>, f64)) {
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

    pub fn get_best_fitness(&self) -> Option<f64> {
        return self.best_solution.to_owned().map(|(_, fitness)| fitness);
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
