use std::char;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let input = "INPUT12.txt";
    let area = parse(input);
    part1(&area);
    part2(&area);
}

fn part1(input: &((usize, usize), (usize, usize), Vec<Vec<u32>>)) {
    let heights = input.2.clone();
    let mut distances = HashMap::new();
    distances.insert(input.0, 0);
    let mut to_inspect = Vec::new();
    to_inspect.push(input.0);
    while !to_inspect.is_empty() {
        to_inspect.sort_by(|a, b| distances.get(a).unwrap().cmp(distances.get(b).unwrap()));
        to_inspect.reverse();
        let cur = to_inspect.pop().unwrap();
        let cur_dist = distances.get(&cur).unwrap();
        if distances.get(&input.1).is_none() || distances.get(&input.1).unwrap() < cur_dist {
            if cur.0 > 0 {
                handle(
                    cur,
                    (cur.0 - 1, cur.1),
                    &heights,
                    &mut to_inspect,
                    &mut distances,
                )
            }
            if cur.0 + 1 < heights.len() {
                handle(
                    cur,
                    (cur.0 + 1, cur.1),
                    &heights,
                    &mut to_inspect,
                    &mut distances,
                )
            }
            if cur.1 > 0 {
                handle(
                    cur,
                    (cur.0, cur.1 - 1),
                    &heights,
                    &mut to_inspect,
                    &mut distances,
                )
            }
            if cur.1 + 1 < heights[0].len() {
                handle(
                    cur,
                    (cur.0, cur.1 + 1),
                    &heights,
                    &mut to_inspect,
                    &mut distances,
                )
            }
        }
    }
    println!("part 1: {}", distances.get(&input.1).unwrap())
}
fn part2(input: &((usize, usize), (usize, usize), Vec<Vec<u32>>)) {
    let heights = input.2.clone();
    let mut distances = HashMap::new();
    let mut to_inspect = Vec::new();
    for i in 0..heights.len() {
        for j in 0..heights[i].len() {
            if heights[i][j] == 0 {
                to_inspect.push((i, j));
                distances.insert((i, j), 0);
            }
        }
    }
    while !to_inspect.is_empty() {
        to_inspect.sort_by(|a, b| distances.get(a).unwrap().cmp(distances.get(b).unwrap()));
        to_inspect.reverse();
        let cur = to_inspect.pop().unwrap();
        let cur_dist = distances.get(&cur).unwrap();
        if distances.get(&input.1).is_none() || distances.get(&input.1).unwrap() < cur_dist {
            if cur.0 > 0 {
                handle(
                    cur,
                    (cur.0 - 1, cur.1),
                    &heights,
                    &mut to_inspect,
                    &mut distances,
                )
            }
            if cur.0 + 1 < heights.len() {
                handle(
                    cur,
                    (cur.0 + 1, cur.1),
                    &heights,
                    &mut to_inspect,
                    &mut distances,
                )
            }
            if cur.1 > 0 {
                handle(
                    cur,
                    (cur.0, cur.1 - 1),
                    &heights,
                    &mut to_inspect,
                    &mut distances,
                )
            }
            if cur.1 + 1 < heights[0].len() {
                handle(
                    cur,
                    (cur.0, cur.1 + 1),
                    &heights,
                    &mut to_inspect,
                    &mut distances,
                )
            }
        }
    }
    println!("part 1: {}", distances.get(&input.1).unwrap())
}
fn handle(
    cur: (usize, usize),
    neighbour: (usize, usize),
    heights: &Vec<Vec<u32>>,
    to_inspect: &mut Vec<(usize, usize)>,
    distances: &mut HashMap<(usize, usize), u32>,
) {
    if (distances.get(&neighbour).is_none()
        || *distances.get(&neighbour).unwrap() > distances.get(&cur).unwrap() + 1)
        && heights[cur.0][cur.1] + 1 >= heights[neighbour.0][neighbour.1]
    {
        if !to_inspect.contains(&neighbour) {
            to_inspect.push(neighbour)
        }
        let cur_dist = distances.get(&cur).unwrap();
        distances.insert(neighbour, cur_dist + 1);
    }
}
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
fn parse(path: &str) -> ((usize, usize), (usize, usize), Vec<Vec<u32>>) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut area = Vec::new();
    let lines = lines_from_file(path);
    for i in 0..lines.len() {
        let mut height = Vec::new();
        let chars: Vec<char> = lines[i].chars().collect();
        for j in 0..chars.len() {
            let mut char = chars[j];
            if char == 'S' {
                start = (i, j);
                char = 'a';
            } else if char == 'E' {
                end = (i, j);
                char = 'z';
            }
            height.push(char.to_digit(36).unwrap() - 10);
        }
        area.push(height);
    }
    (start, end, area)
}
