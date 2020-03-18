use quicli::prelude::*;

use super::TSP;
use super::dm::DistanceMatrix;
use std::path::PathBuf;

pub fn parse_problem_instance(tsp_path: &PathBuf) -> Result<TSP, Error> {
    enum CoordinateSystem {
        Euclidean,
        Geographical,
    }

    use std::fmt;

    impl fmt::Debug for CoordinateSystem {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", match &self {
                Self::Euclidean => "Euclidean2D",
                Self::Geographical => "Geographical",
            })
        }
    }

    let mut name: Option<String> = None;
    let mut dimension: Option<usize> = None;
    let mut coord_system: Option<CoordinateSystem> = None;

    use std::time::Instant;
    
    let start = Instant::now();
    
    let file = read_file(&tsp_path)?;
    let mut lines = file.lines();
    loop {
        match lines.next() {
            Some("NODE_COORD_SECTION") => break,
            Some(line) => {
                if let Some(semicolon_index) = line.find(':') {
                    if line.starts_with("NAME") {
                        name = Some(line[semicolon_index + 1..].trim_start().to_owned());
                        info!("parsed name: {:?}", name.as_ref().unwrap());
                    } else if line.starts_with("DIMENSION") {
                        dimension = Some(line[semicolon_index + 1..].trim().parse::<usize>()?);
                        info!("parsed dimension: {:?}", dimension.unwrap());
                    } else if line.starts_with("EDGE_WEIGHT_TYPE") {
                        coord_system = match line[semicolon_index + 1..].trim() {
                            "EUC_2D" => Some(CoordinateSystem::Euclidean),
                            "GEO" => Some(CoordinateSystem::Geographical),
                            _ => return Err(format_err!("unknown edge weight type"))
                        };
                        info!("parsed edge weight type: {:?}", coord_system.as_ref().unwrap());
                    }
                }
            }
            None => return Err(format_err!("node coord section missing"))
        }
    }
    let coord_system: CoordinateSystem = coord_system
        .ok_or_else(|| format_err!("edge weight type missing"))?;

    let nodes_iter = lines.take_while(|line| *line != "EOF");
    let (dm, inferred_dimension) = match coord_system {
        CoordinateSystem::Euclidean => {
            let nodes: Vec<(f64, f64)> = nodes_iter.map(|coord_line| {
                let values: Vec<&str> = coord_line.split_whitespace().collect();
                values[1].parse::<f64>().and_then(|x| {
                    values[2].parse::<f64>().map(|y| (x, y))
                })
            }).collect::<Result<Vec<(f64, f64)>, std::num::ParseFloatError>>()?;
            info!("parsed the node coord section");
            (
                DistanceMatrix::new(&nodes, |a: &(f64, f64), b: &(f64, f64)| -> u32 {
                    let xd = a.0 - b.0;
                    let yd = a.1 - b.1;
                    (xd * xd + yd * yd).sqrt().round() as u32
                }),
                nodes.len()
            )
        },
        CoordinateSystem::Geographical => {
            let nodes = nodes_iter.map(|coord_line| {
                let values: Vec<&str> = coord_line.split_whitespace().collect();
                values[1].parse::<f64>().and_then(|longitude| {
                    values[2].parse::<f64>().map(|latitude| (latitude, longitude))
                })
            }).collect::<Result<Vec<(f64, f64)>, std::num::ParseFloatError>>()?;
            info!("parsed the node coord section");
            (
                DistanceMatrix::new(&nodes, |a: &(f64, f64), b: &(f64, f64)| -> u32 {
                    fn geo(x: &(f64, f64)) -> (f64, f64) {
                        use std::f64::consts::PI;
                        let deg = x.0.round();
                        let min = x.0 - deg;
                        let latitude = PI * (deg + 5.0 * min / 3.0) / 180.0;
                        let deg = x.1.round();
                        let min = x.1 - deg;
                        let longitude = PI * (deg + 5.0 * min / 3.0) / 180.0;
                        return (latitude, longitude);
                    }

                    let a = geo(a);
                    let b = geo(b);
                    let radius = 6378.388; // Earth's radius
                    let q1 = (a.1 - b.1).cos();
                    let q2 = (a.0 - b.0).cos();
                    let q3 = (a.0 + b.0).cos();
                    return (radius * (0.5 * ((1.0 + q1) * q2 - (1.0 - q1) * q3)).acos() + 1.0) as u32;
                }),
                nodes.len()
            )
        }
    };

    let dimension: usize = dimension.unwrap_or_else(|| {
        warn!("dimension not provided explicitly, inferred {}", inferred_dimension);
        inferred_dimension
    });

    let duration = start.elapsed();
    info!("parsed the problem instance in {:?}", duration);

    Ok(TSP {
        name: name,
        dimension: dimension,
        dm: dm,
    })
}
