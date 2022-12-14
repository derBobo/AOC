use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let input = "INPUT14.txt";
    let cave = parse(input);

    part1(cave.clone());
    part2(cave);
}

fn part2(mut cave: Vec<Vec<char>>) {
    let mut counter = 0;
    let mut ground = vec![Vec::new(), Vec::new()];
    for _ in 0..cave[0].len() {
        ground[0].push('.');
        ground[1].push('#');
    }
    cave.append(&mut ground);
    loop {
        let maybe_new_sand = new_sand_coord(&cave);
        if maybe_new_sand.is_none() {
            print_cave(&cave);
            break;
        } else {
            let new_sand = maybe_new_sand.unwrap();
            if new_sand.0 > 0 && new_sand.0 < cave[new_sand.1 as usize].len() as i32 - 1 {
                cave[new_sand.1 as usize][new_sand.0 as usize] = 'O';
                counter += 1;
            } else if new_sand.0 <= 0 {
                for i in 0..cave.len() - 1 {
                    cave[i].insert(0, '.');
                }
                let len = cave.len() - 1;
                cave[len].insert(0, '#');
            } else {
                for i in 0..cave.len() - 1 {
                    cave[i].push('.');
                }
                let len = cave.len() - 1;
                cave[len].push('#');
            }
        }
    }
    println!("part 2: {}", counter);
}
fn part1(mut cave: Vec<Vec<char>>) {
    let mut counter = 0;
    loop {
        let new_sand = new_sand_coord(&cave).unwrap();
        if new_sand.0 > 0
            && new_sand.0 < cave[new_sand.1 as usize].len() as i32 - 1
            && new_sand.1 < cave.len() as i32 - 1
        {
            cave[new_sand.1 as usize][new_sand.0 as usize] = 'O';
            counter += 1;
        } else {
            print_cave(&cave);
            break;
        }
    }
    println!("part 1: {}", counter);
}
fn get_sand_spawn(cave: &Vec<Vec<char>>) -> Option<(i32, i32)> {
    for y in 0..cave.len() {
        for x in 0..cave[y].len() {
            if cave[y][x] == '+' {
                return Some((x as i32, y as i32));
            }
        }
    }
    None
}
fn new_sand_coord(cave: &Vec<Vec<char>>) -> Option<(i32, i32)> {
    let spawn = get_sand_spawn(cave);
    if spawn.is_none() {
        None
    } else {
        let mut new_sand = spawn.unwrap();
        while new_sand.1 < cave.len() as i32 - 1
            && new_sand.0 > 0
            && new_sand.0 < cave[new_sand.1 as usize].len() as i32 - 1
        {
            if cave[new_sand.1 as usize + 1][new_sand.0 as usize] == '.' {
                new_sand = (new_sand.0, new_sand.1 + 1);
            } else if cave[new_sand.1 as usize + 1][new_sand.0 as usize - 1] == '.' {
                new_sand = (new_sand.0 - 1, new_sand.1 + 1);
            } else if cave[new_sand.1 as usize + 1][new_sand.0 as usize + 1] == '.' {
                new_sand = (new_sand.0 + 1, new_sand.1 + 1);
            } else {
                break;
            }
        }
        Some(new_sand)
    }
}

fn print_cave(cave: &Vec<Vec<char>>) {
    for line in cave {
        println!(
            "{}",
            line.iter().fold("".to_string(), |mut s, c| {
                s.push(*c);
                s
            })
        )
    }
}
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
fn parse(path: &str) -> Vec<Vec<char>> {
    let sand = (500, 0);
    let lines = lines_from_file(path);
    let rocks: Vec<Vec<(usize, usize)>> = lines
        .iter()
        .map(|line| {
            let pairs: Vec<&str> = line.split(" -> ").collect();
            let points: Vec<(usize, usize)> = pairs
                .iter()
                .map(|pair| {
                    let split: Vec<&str> = pair.split(",").collect();
                    (split[0].parse().unwrap(), split[1].parse().unwrap())
                })
                .collect();
            points
        })
        .collect();
    let x_offset = rocks
        .iter()
        .map(|line| line.into_iter().map(|point| point.0).min().unwrap())
        .min()
        .unwrap();
    let x_max = rocks
        .iter()
        .map(|line| line.into_iter().map(|point| point.0).max().unwrap())
        .max()
        .unwrap()
        + 1;
    let y_max = rocks
        .iter()
        .map(|line| line.into_iter().map(|point| point.1).max().unwrap())
        .max()
        .unwrap()
        + 1;
    let mut row = Vec::new();
    for _ in 0..x_max - x_offset {
        row.push('.');
    }
    let mut cave = Vec::new();
    for _ in 0..y_max {
        cave.push(row.clone());
    }
    cave[sand.1][sand.0 - x_offset] = '+';
    for line in rocks {
        let mut last = line[0];
        for cur in &line[1..] {
            for x in min(cur.0, last.0)..max(cur.0, last.0) + 1 {
                for y in min(cur.1, last.1)..max(cur.1, last.1) + 1 {
                    cave[y][x - x_offset] = '#';
                }
            }
            last = *cur;
        }
    }

    cave
}
