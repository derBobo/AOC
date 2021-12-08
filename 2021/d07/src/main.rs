use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let path = "SAMPLEINPUT07.txt";
    let input = File::open(path).expect("file not found!");
    let buffered = BufReader::new(input);
    let mut positions: Vec<i64> = Vec::new();
    for line in buffered.lines() {
        positions = line.unwrap().split(",").map(|x| x.parse::<i64>().unwrap_or(0)).collect();
    }

    positions.sort();
    let mut median = 0;
    if positions.len() % 2 == 0 {
        median = (positions[positions.len() / 2] + positions[(positions.len() + 1) / 2]) / 2;
    } else {
    median = positions[(positions.len() + 1) / 2];
    }
    let part1:i64=positions.iter().map(|&x| (x-median).abs()).sum();

    let average =positions.iter().sum::<i64>() /positions.len()as i64;
    let part2=positions.iter().fold(0,|fuel ,x|(fuel+((average-x).abs())*((average-x).abs()+1)/2));
    println!("{}",part1);
    println!("{}",part2);
}
