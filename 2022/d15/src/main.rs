use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;
use std::path::Path;

fn main() {
    let input = "SAMPLEINPUT15.txt";
    let pairs = parse(input);
    part1(&pairs);
    part2(&pairs);
}

fn part2(pairs: &Vec<((i32, i32), (i32, i32))>) {

    let beacon = (0,)
    println!("part 2 : {}", beacon.0 * 4000000 + beacon.1);
}
fn part1(pairs: &Vec<((i32, i32), (i32, i32))>) {
    let row = 2000000;
    let x_min = pairs
        .iter()
        .map(|x| x.0 .0 - manhattan_distance(&x.0, &x.1))
        .min()
        .unwrap();
    let x_max = pairs
        .iter()
        .map(|x| x.0 .0 + manhattan_distance(&x.0, &x.1))
        .max()
        .unwrap();
    println!(
        "part 1 : {}",
        no_beacon_in_range(pairs, x_min..x_max + 1, row..row + 1).len()
    );
}

fn manhattan_distance(p1: &(i32, i32), p2: &(i32, i32)) -> i32 {
    (p1.1 - p2.1).abs() + (p1.0 - p2.0).abs()
}
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
fn no_beacon_in_range(
    pairs: &Vec<((i32, i32), (i32, i32))>,
    x_range: Range<i32>,
    y_range: Range<i32>,
) -> HashSet<(i32, i32)> {
    let mut no_beacons = HashSet::new();
    for i in 0..pairs.len() {
        let man = manhattan_distance(&pairs[i].0, &pairs[i].1);
        let x_min = max(pairs[i].0 .0 - man, x_range.start);
        let x_max = min(pairs[i].0 .0 + man + 1, x_range.end);
        for x in x_min..x_max {
            let n = (man - (x - pairs[i].0 .0).abs()).abs();
            let y_min = max(pairs[i].0 .1 - n, y_range.start);
            let y_max = min(pairs[i].0 .1 + n + 1, y_range.end);
            for y in y_min..y_max {
                no_beacons.insert((x, y));
            }
        }
    }
    for i in 0..pairs.len() {
        no_beacons.remove(&(pairs[i].1 .0, pairs[i].1 .1));
    }
    no_beacons
}
fn parse(path: &str) -> Vec<((i32, i32), (i32, i32))> {
    let lines = lines_from_file(path);
    let signals_and_beacons: Vec<((i32, i32), (i32, i32))> = lines
        .iter()
        .map(|x| {
            let temp = x
                .replace("Sensor at x=", "")
                .replace(" y=", "")
                .replace(": closest beacon is at x=", ",");
            let split: Vec<&str> = temp.split(",").collect();
            (
                (
                    split[0].parse::<i32>().unwrap(),
                    split[1].parse::<i32>().unwrap(),
                ),
                (
                    split[2].parse::<i32>().unwrap(),
                    split[3].parse::<i32>().unwrap(),
                ),
            )
        })
        .collect();
    signals_and_beacons
}
