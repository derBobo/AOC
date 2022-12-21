use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let input = "INPUT18.txt";
    let blocks = parse(input);
    part1(&blocks);
    part2(&blocks)
}

fn part2(blocks: &HashSet<(i32, i32, i32)>) {
    let x_max = blocks.iter().map(|x| x.0).max().unwrap() + 1;
    let x_min = blocks.iter().map(|x| x.0).min().unwrap() - 1;
    let y_max = blocks.iter().map(|x| x.1).max().unwrap() + 1;
    let y_min = blocks.iter().map(|x| x.1).min().unwrap() - 1;
    let z_max = blocks.iter().map(|x| x.2).max().unwrap() + 1;
    let z_min = blocks.iter().map(|x| x.2).min().unwrap() - 1;
    let mut sum = 0;
    let mut expanded = HashSet::new();
    let mut to_expand = Vec::new();
    to_expand.push((x_min, y_min, z_min));
    while let Some((x, y, z)) = to_expand.pop() {
        sum += get_neighbours(x, y, z, blocks).len();
        for i in [-1, 1] {
            if !blocks.contains(&(x + i, y, z))
                && !to_expand.contains(&(x + i, y, z))
                && !expanded.contains(&(x + i, y, z))
                && x_min <= x + i
                && x + i <= x_max
                && y_min <= y
                && y <= y_max
                && z_min <= z
                && z <= z_max
            {
                to_expand.push((x + i, y, z));
            }
            if !blocks.contains(&(x, y + i, z))
                && !to_expand.contains(&(x, y + i, z))
                && !expanded.contains(&(x, y + i, z))
                && x_min <= x
                && x <= x_max
                && y_min <= y + i
                && y + i <= y_max
                && z_min <= z
                && z <= z_max
            {
                to_expand.push((x, y + i, z));
            }
            if !blocks.contains(&(x, y, z + i))
                && !to_expand.contains(&(x, y, z + i))
                && !expanded.contains(&(x, y, z + i))
                && x_min <= x
                && x <= x_max
                && y_min <= y
                && y <= y_max
                && z_min <= z + i
                && z + i <= z_max
            {
                to_expand.push((x, y, z + i));
            }
        }
        expanded.insert((x, y, z));
    }
    println!("part 2: {}", sum)
}

fn part1(blocks: &HashSet<(i32, i32, i32)>) {
    let mut sum = 0;
    for j in blocks {
        sum += 6 - get_neighbours(j.0, j.1, j.2, blocks).len();
    }
    println!("part 1: {}", sum)
}
fn get_neighbours(
    x: i32,
    y: i32,
    z: i32,
    blocks: &HashSet<(i32, i32, i32)>,
) -> Vec<(i32, i32, i32)> {
    let mut res = Vec::new();
    for i in [-1, 1] {
        if blocks.contains(&(x + i, y, z)) {
            res.push((x + i, y, z));
        }
        if blocks.contains(&(x, y + 1, z)) {
            res.push((x, y + i, z));
        }
        if blocks.contains(&(x, y, z + i)) {
            res.push((x, y, z + i));
        }
    }
    res
}
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse Line"))
        .collect()
}

fn parse(path: &str) -> HashSet<(i32, i32, i32)> {
    let lines = lines_from_file(path);
    lines
        .iter()
        .map(|x| {
            let split: Vec<&str> = x.split(",").collect();
            (
                split[0].parse().unwrap(),
                split[1].parse().unwrap(),
                split[2].parse().unwrap(),
            )
        })
        .collect()
}
