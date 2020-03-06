use quicli::prelude::*;

use super::TSPInstance;
use super::dm::DistanceMatrix;
use std::path::PathBuf;

pub fn read_problem_instance(tsp_path: &PathBuf) -> Result<TSPInstance, Error> {
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
    let mut comment: Option<String> = None;
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
                    } else if line.starts_with("COMMENT") {
                        comment = Some(line[semicolon_index + 1..].trim_start().to_owned());
                        info!("parsed comment: {:?}", comment.as_ref().unwrap());
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
            struct EuclideanCoord {
                x: f64,
                y: f64,
            }
            let nodes: Vec<EuclideanCoord> = nodes_iter.map(|coord_line| {
                let values: Vec<&str> = coord_line.split_whitespace().collect();
                values[1].parse::<f64>().and_then(|x| {
                    values[2].parse::<f64>().map(|y| {
                        EuclideanCoord {
                            x: x,
                            y: y,
                        }
                    })
                })
            }).collect::<Result<Vec<EuclideanCoord>, std::num::ParseFloatError>>()?;
            info!("parsed the node coord section");
            (
                DistanceMatrix::new(&nodes, |a: &EuclideanCoord, b: &EuclideanCoord| -> f64 {
                    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
                }),
                nodes.len()
            )
        },
        CoordinateSystem::Geographical => {
            struct GeographicalCoord {
                latitude: f64,
                longitude: f64,
            }
            let nodes = nodes_iter.map(|coord_line| {
                let values: Vec<&str> = coord_line.split_whitespace().collect();
                values[1].parse::<f64>().and_then(|longitude_deg| {
                    values[2].parse::<f64>().map(|latitude_deg| {
                        GeographicalCoord {
                            latitude: latitude_deg.to_radians(),
                            longitude: longitude_deg.to_radians(),
                        }
                    })
                })
            }).collect::<Result<Vec<GeographicalCoord>, std::num::ParseFloatError>>()?;
            info!("parsed the node coord section");
            (
                DistanceMatrix::new(&nodes, |a: &GeographicalCoord, b: &GeographicalCoord| -> f64 {
                    let radius: f64 = 1.0;
                    radius * ((a.latitude - b.latitude).cos() * (a.longitude - b.longitude).cos()).acos()
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

    Ok(TSPInstance {
        name: name,
        comment: comment,
        dimension: dimension,
        dm: dm,
    })
}
