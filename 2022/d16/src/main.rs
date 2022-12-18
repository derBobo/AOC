use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let input = "INPUT16.txt";
    let valves = parse(input);
    let start = u16::from_str_radix("AA", 36).unwrap();
    let mut targets = get_valid_targets(&valves);
    targets.push(start);
    let mut walking_distances: HashMap<(u16, u16), u32> = HashMap::new();
    for i in 0..targets.len() {
        let distance = breadth_search(targets[i], &valves, &targets);
        for d in distance {
            walking_distances.insert((targets[i], d.0), d.1);
        }
        //print!("{:?}", walking_distances);
    }
    part1(valves.clone(), &walking_distances, start);
    part2(valves.clone(), &walking_distances, start);
}
fn part2(
    valves: HashMap<u16, (u32, Vec<u16>)>,
    walking_distances: &HashMap<(u16, u16), u32>,
    start: u16,
) {
    let max_min = 26;
    println!(
        "part 2: {}",
        get_max_flow_part2(
            vec![start],
            vec![start],
            max_min,
            &valves,
            walking_distances,
        )
    );
}
fn part1(
    valves: HashMap<u16, (u32, Vec<u16>)>,
    walking_distances: &HashMap<(u16, u16), u32>,
    start: u16,
) {
    let max_min = 30;
    println!(
        "part 1: {}",
        get_max_flow_part1(start, max_min, &valves, walking_distances).0
    );
}
fn get_max_flow_part2(
    my_way: Vec<u16>,
    elephant_way: Vec<u16>,
    min_limit: u32,
    valves: &HashMap<u16, (u32, Vec<u16>)>,
    walking_distances: &HashMap<(u16, u16), u32>,
) -> u32 {
    let mut res = vec![0 as u32];
    let targets = get_valid_targets(&valves);
    let my_path = my_way
        .iter()
        .fold((0, 0), |last, x| {
            if last == (0, 0) {
                (*x, 0)
            } else {
                (
                    *x,
                    last.1 + 1 + walking_distances.get(&(last.0, *x)).unwrap(),
                )
            }
        })
        .1;
    let elephant_path = elephant_way
        .iter()
        .fold((0, 0), |last, x| {
            if last == (0, 0) {
                (*x, 0)
            } else {
                (
                    *x,
                    last.1 + 1 + walking_distances.get(&(last.0, *x)).unwrap(),
                )
            }
        })
        .1;
    if elephant_path < my_path {
        let start = *elephant_way.last().unwrap();
        for target in targets {
            let time = *walking_distances.get(&(start, target)).unwrap();
            if time < min_limit {
                let mut new_valves = valves.clone();
                let old = valves.get(&target).unwrap();
                new_valves.insert(target.clone(), (0, old.1.clone()));
                let pressure = old.0 * (min_limit - 1 - time);
                let mut new_way = elephant_way.clone();
                new_way.push(target);
                res.push(
                    pressure
                        + get_max_flow_part2(
                            my_way.clone(),
                            new_way,
                            26 - min(my_path, elephant_path + 1 + time),
                            &new_valves,
                            walking_distances,
                        ),
                )
            }
        }
    } else {
        let start = *my_way.last().unwrap();
        for target in targets {
            let time = *walking_distances.get(&(start, target)).unwrap();
            if time < min_limit {
                let mut new_valves = valves.clone();
                let old = valves.get(&target).unwrap();
                new_valves.insert(target.clone(), (0, old.1.clone()));
                let pressure = old.0 * (min_limit - 1 - time);
                let mut new_way = my_way.clone();
                new_way.push(target);
                res.push(
                    pressure
                        + get_max_flow_part2(
                            new_way,
                            elephant_way.clone(),
                            26 - min(my_path + 1 + time, elephant_path),
                            &new_valves,
                            walking_distances,
                        ),
                )
            }
        }
    }
    res.into_iter().max().unwrap()
}

fn get_max_flow_part1(
    start: u16,
    min_limit: u32,
    valves: &HashMap<u16, (u32, Vec<u16>)>,
    walking_distances: &HashMap<(u16, u16), u32>,
) -> (u32, Vec<(u16, u32)>) {
    let mut res: HashMap<u32, Vec<(u16, u32)>> = HashMap::new();
    res.insert(0, Vec::new());
    let targets = get_valid_targets(valves);
    for target in targets {
        let time = *walking_distances.get(&(start, target)).unwrap();
        if time < min_limit {
            let mut new_valves = valves.clone();
            let old = valves.get(&target).unwrap();
            new_valves.insert(target.clone(), (0, old.1.clone()));
            let pressure = old.0 * (min_limit - 1 - time);
            let best_of_rest =
                get_max_flow_part1(target, min_limit - time - 1, &new_valves, walking_distances);
            let mut new_vec: Vec<(u16, u32)> = best_of_rest
                .1
                .into_iter()
                .map(|x| (x.0, x.1 + time))
                .collect();
            new_vec.insert(0, (target, time));
            res.insert(pressure + best_of_rest.0, new_vec);
        }
    }
    let max = res.keys().max().unwrap();
    let best = res.get(max).unwrap();
    return (*max, best.clone());
}

fn breadth_search(
    start: u16,
    valves: &HashMap<u16, (u32, Vec<u16>)>,
    targets: &Vec<u16>,
) -> HashMap<u16, u32> {
    let mut min = 0;
    let mut expandend = HashSet::new();
    let mut to_expand = HashSet::new();
    let mut res = HashMap::new();
    to_expand.insert(start);
    while !to_expand.is_empty() && res.len() != targets.len() - 1 {
        min += 1;
        let temp: Vec<u16> = to_expand.into_iter().collect();
        to_expand = HashSet::new();
        for j in 0..temp.len() {
            let v = valves.get(&temp[j]).unwrap();
            for i in 0..v.1.len() {
                if !expandend.contains(&v.1[i]) && !temp.contains(&v.1[i]) {
                    to_expand.insert(v.1[i]);
                    if targets.contains(&v.1[i]) && !res.contains_key(&v.1[i]) {
                        res.insert(v.1[i].clone(), min);
                    }
                }
            }
            expandend.insert(temp[j].clone());
        }
    }
    res
}

fn get_valid_targets(valves: &HashMap<u16, (u32, Vec<u16>)>) -> Vec<u16> {
    valves
        .keys()
        .clone()
        .into_iter()
        .filter(|x| valves.get(*x).unwrap().0 != 0)
        .map(|x| (*x).clone())
        .collect()
}
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn parse(path: &str) -> HashMap<u16, (u32, Vec<u16>)> {
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
        let key = u16::from_str_radix(split2[0], 36).unwrap();
        res.insert(
            key,
            (
                split2[1].to_string().parse().unwrap(),
                split1[1]
                    .split(", ")
                    .map(|x| u16::from_str_radix(x, 36).unwrap())
                    .collect(),
            ),
        );
    }
    res
}
