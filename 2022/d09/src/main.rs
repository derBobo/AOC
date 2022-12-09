use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
fn main() {
    let input = "..\\INPUT09.txt";
    let moves = parse(input);
    part1(moves.clone());
    part2(moves);
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn parse(path: &str) -> Vec<(Option<Direction>, i32)> {
    let lines = lines_from_file(path);
    lines
        .iter()
        .map(|line| {
            let split: Vec<&str> = line.split(" ").collect();
            (
                match split[0] {
                    "R" => Some(Direction::Right),
                    "L" => Some(Direction::Left),
                    "U" => Some(Direction::Up),
                    "D" => Some(Direction::Down),
                    &_ => None,
                },
                split[1].parse::<i32>().unwrap(),
            )
        })
        .rev()
        .collect()
}

fn part1(mut moves: Vec<(Option<Direction>, i32)>) {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);
    while !moves.is_empty() {
        let mut cur = moves.pop().unwrap();
        while cur.1 > 0 {
            match cur.0 {
                None => {
                    panic!("Could not parse Direction")
                }
                Some(Direction::Down) => head.1 -= 1,
                Some(Direction::Left) => head.0 -= 1,
                Some(Direction::Right) => head.0 += 1,
                Some(Direction::Up) => head.1 += 1,
            }
            if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                tail = move_tail(head, tail);
                visited.insert(tail);
            }
            cur.1 -= 1;
        }
    }
    println!("part 1: {}", visited.len());
}
fn part2(mut moves: Vec<(Option<Direction>, i32)>) {
    let mut visited_by_tail: HashSet<(i32, i32)> = HashSet::new();
    visited_by_tail.insert((0, 0));
    let mut knots: Vec<(i32, i32)> = vec![
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];
    while !moves.is_empty() {
        let mut cur = moves.pop().unwrap();
        while cur.1 > 0 {
            match cur.0 {
                None => {
                    panic!("Could not parse Direction")
                }
                Some(Direction::Down) => knots[0].1 -= 1,
                Some(Direction::Left) => knots[0].0 -= 1,
                Some(Direction::Right) => knots[0].0 += 1,
                Some(Direction::Up) => knots[0].1 += 1,
            }
            for i in 1..knots.len() {
                if (knots[i - 1].0 - knots[i].0).abs() > 1
                    || (knots[i - 1].1 - knots[i].1).abs() > 1
                {
                    knots[i] = move_tail(knots[i - 1], knots[i]);
                    if i == knots.len() - 1 {
                        visited_by_tail.insert(knots[i]);
                    }
                }
            }
            cur.1 -= 1;
        }
    }
    println!("part 2: {}", visited_by_tail.len());
}

fn move_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let mut new_tail = tail;
    if (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1 {
        new_tail
    } else if tail.0 == head.0 {
        if head.1 > tail.1 {
            new_tail.1 += 1
        } else {
            new_tail.1 -= 1
        }
        new_tail
    } else if tail.1 == head.1 {
        if head.0 > tail.0 {
            new_tail.0 += 1
        } else {
            new_tail.0 -= 1
        }
        new_tail
    } else {
        if head.1 > tail.1 {
            new_tail.1 += 1
        } else {
            new_tail.1 -= 1
        }
        if head.0 > tail.0 {
            new_tail.0 += 1
        } else {
            new_tail.0 -= 1
        }
        new_tail
    }
}
