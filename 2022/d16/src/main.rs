use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Deref;
use std::path::Path;

fn main() {
    let input = "SAMPLEINPUT16.txt";
    let valves = parse(input);
    part1(valves.clone());
    //part2(valves.clone());
}
fn part1(mut valves: HashMap<String, (u32, Vec<String>)>) {
    let max_min = 30;
    let mut cur = "AA".to_string();

    println!("part 1: {}", get_max_flow(&*cur, &valves, max_min));
}
fn get_max_flow(cur: &str, valves: &HashMap<String, (u32, Vec<String>)>, min_limit: u32) -> u32 {
    let released_pressure = Vec::new();
    let targets: Vec<String> = valves
        .keys()
        .clone()
        .into_iter()
        .filter(|x| valves.get(*x).unwrap().0 != 0)
        .map(|x| (*x).clone())
        .collect();
    let walking_times = breadth_search(cur, valves, &targets);
    for target in targets {
        let mut new_valves = valves.clone();
        let old = valves.get(&target).unwrap();
        new_valves.insert(target.clone(), (0, old.1.clone()));
        released_pressure()
    }
    released_pressure.iter().max().unwrap();
}

fn breadth_search(
    cur: &str,
    valves: &HashMap<String, (u32, Vec<String>)>,
    targets: &Vec<String>,
) -> HashMap<String, u32> {
    let mut min = 0;
    let mut expandend = HashSet::new();
    let mut to_expand = HashSet::new();
    let mut res = HashMap::new();
    to_expand.insert(cur);
    while !to_expand.is_empty() && res.len() != targets.len() {
        min += 1;
        let temp: Vec<String> = to_expand
            .clone()
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        to_expand = HashSet::new();
        for j in 0..temp.len() {
            let v = valves.get(&*temp[j]).unwrap();
            for i in 0..v.1.len() {
                if !expandend.contains(&v.1[i]) && !temp.contains(&v.1[i]) {
                    to_expand.insert(&*v.1[i]);
                    if targets.contains(&v.1[i]) && !res.contains_key(&*v.1[i]) {
                        res.insert(v.1[i].clone(), min + 1);
                    }
                }
            }
            expandend.insert(temp[j].clone());
        }
    }
    res
}
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn parse(path: &str) -> HashMap<String, (u32, Vec<String>)> {
    let lines = lines_from_file(path);
    let mut res = HashMap::new();
    for i in 0..lines.len() {
        let temp = lines[i]
            .replace("Valve ", "")
            .replace("has flow rate=", "")
            .replace(" tunnels lead to valves ", "")
            .replace(" tunnel leads to valve ", "");
        let split1: Vec<&str> = temp.split(";").collect();

        let split2: Vec<&str> = split1[0].split(" ").collect();
        let key = split2[0].to_string();
        res.insert(
            key,
            (
                split2[1].to_string().parse().unwrap(),
                split1[1]
                    .split(", ")
                    .map(|x| x.to_string().clone())
                    .collect(),
            ),
        );
    }
    res
}
