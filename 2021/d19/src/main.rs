use itertools::Itertools;
use nalgebra::{Matrix3, Point3};
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "INPUT19.txt";
    let input = File::open(path).expect("file not found!");
    let buffered = BufReader::new(input);
    let lines: Vec<String> = buffered
        .lines()
        .map(|x| x.unwrap().to_string())
        .filter(|x| x != "")
        .collect();
    let mut beaconclouds: Vec<BeaconCloud> = Vec::new();
    let mut beacons: Vec<Point3<i32>> = Vec::new();
    for line in lines {
        if line.starts_with("---") {
            if &beacons.len() > &0 {
                beaconclouds.push(BeaconCloud::new(beacons));
                beacons = Vec::new();
            }
        } else {
            let coord: Vec<i32> = line.split(",").map(|x| x.parse().unwrap()).collect();
            beacons.push(Point3::new(coord[0], coord[1], coord[2]))
        }
    }
    beaconclouds.push(BeaconCloud::new(beacons));
    let mut cloud = beaconclouds[0].clone();
    beaconclouds.remove(0);
    while beaconclouds.len() > 0 {
        let mut to_match = Vec::new();
        for cur_cloud in beaconclouds {
            if !cloud.try_merge(&cur_cloud) {
                to_match.push(cur_cloud);
            }
        }
        beaconclouds = to_match;
    }
    println!("{}", cloud.beacons.len());
    let mut distances: Vec<i32> = cloud
        .scanners
        .iter()
        .cartesian_product(&cloud.scanners)
        .map(|x| manhattan_distance(x.0, x.1))
        .collect();
    distances.sort();
    println!("{}", distances.pop().unwrap());
}
fn distance_squared(p1: &Point3<i32>, p2: &Point3<i32>) -> i32 {
    (p2.x - p1.x).pow(2) + (p2.y - p1.y).pow(2) + (p2.z - p1.z).pow(2)
}
fn manhattan_distance(p1: &Point3<i32>, p2: &Point3<i32>) -> i32 {
    (p2.x - p1.x).abs() + (p2.y - p1.y).abs() + (p2.z - p1.z).abs()
}

struct BeaconCloud {
    beacons: Vec<Point3<i32>>,
    scanners: Vec<Point3<i32>>,
}
impl Debug for BeaconCloud {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Scanner: {:?}\nPoints: {:?}",
            self.scanners, self.beacons
        )
    }
}
impl Clone for BeaconCloud {
    fn clone(&self) -> Self {
        BeaconCloud {
            beacons: self.beacons.clone(),
            scanners: self.scanners.clone(),
        }
    }
}

impl BeaconCloud {
    fn new(beacons: Vec<Point3<i32>>) -> BeaconCloud {
        BeaconCloud {
            beacons,
            scanners: vec![Point3::new(0, 0, 0)],
        }
    }
    fn try_merge(&mut self, other: &BeaconCloud) -> bool {
        let mut correlating_beacons = Vec::new();
        for (beacon_self, beacon_other) in self.beacons.iter().cartesian_product(&other.beacons) {
            let distances_self = self
                .beacons
                .iter()
                .map(|x| distance_squared(beacon_self, x));
            let distances_other = other
                .beacons
                .iter()
                .map(|x| distance_squared(beacon_other, x));
            let correlating_lengths: Vec<(i32, i32)> = distances_self
                .cartesian_product(distances_other)
                .filter(|x| x.0 == x.1)
                .collect();
            if correlating_lengths.len() > 11 {
                correlating_beacons.push((beacon_self, beacon_other));
            }
        }
        if correlating_beacons.len() > 11 {
            for x_maps_to in [0, 1, 2] {
                for y_maps_to in [0, 1, 2] {
                    if x_maps_to == y_maps_to {
                        continue;
                    }
                    let z_maps_to = match (x_maps_to, y_maps_to) {
                        (0, 1) | (1, 0) => 2,
                        (0, 2) | (2, 0) => 1,
                        _ => 0,
                    };
                    for x_dir in [1, -1] {
                        for y_dir in [1, -1] {
                            for z_dir in [1, -1] {
                                let mut transformation = Matrix3::new(0, 0, 0, 0, 0, 0, 0, 0, 0);
                                transformation[(x_maps_to, 0)] = x_dir;
                                transformation[(y_maps_to, 1)] = y_dir;
                                transformation[(z_maps_to, 2)] = z_dir;
                                let offset = correlating_beacons[0].0
                                    - transformation * correlating_beacons[0].1;
                                if correlating_beacons
                                    .iter()
                                    .all(|x| *x.0 == transformation * x.1 + offset)
                                {
                                    for scanner in &other.scanners {
                                        self.scanners.push(transformation * scanner + offset);
                                    }
                                    for beacon in &other.beacons {
                                        if !self
                                            .beacons
                                            .contains(&(transformation * beacon + offset))
                                        {
                                            self.beacons.push(transformation * beacon + offset);
                                        }
                                    }
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
        return false;
    }
}
