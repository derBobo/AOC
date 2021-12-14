use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "SAMPLEINPUT12.txt";
    let input = File::open(path).expect("file not found!");
    let buffered = BufReader::new(input);
    let connections: Vec<[String; 2]> = buffered
        .lines()
        .map(|x| {
            let a: Vec<String> = x.unwrap().split("-").map(|y| y.to_string()).collect();
            [a[0].to_string(), a[1].to_string()]
        })
        .collect();
    let part1 = find_ways_part1(&"start".to_string(), &connections);

    println!("{}", part1.len());
    println!("{}", part2.len());
}

fn find_ways_part1(current: &String, possible_connections: &Vec<[String; 2]>) -> Vec<String> {
    return if current == "end" {
        vec!["end".to_string()]
    } else {
        let mut ways = Vec::new();
        let selectedpaths: Vec<[String; 2]> = possible_connections
            .iter()
            .filter(|x| x.contains(&current))
            .map(|x| [x[0].to_string(), x[1].to_string()])
            .collect();
        let mut remaining_paths: Vec<[String; 2]> = Vec::new();
        if current.chars().all(|x| x.is_lowercase()) {
            remaining_paths = (*possible_connections
                .iter()
                .filter(|x| !x.contains(&current))
                .map(|x| [x[0].to_string(), x[1].to_string()])
                .collect::<Vec<[String; 2]>>())
            .to_owned();
        } else {
            remaining_paths = (*possible_connections
                .iter()
                .map(|x| [x[0].to_string(), x[1].to_string()])
                .collect::<Vec<[String; 2]>>())
            .to_owned();
        }
        for path in selectedpaths {
            for point in path {
                if point.ne(current) {
                    let ways_for_point = find_ways_part1(&point, &remaining_paths);
                    for way in ways_for_point {
                        ways.push(current.to_string() + "," + &way);
                    }
                }
            }
        }
        ways
    };
}
