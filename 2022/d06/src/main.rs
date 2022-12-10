use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::path::Path;

fn main() {
    let input = "INPUT06.txt";
    part1(input);
    part2(input);
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
fn chars_are_unique(chars: Vec<char>) -> bool {
    let set: HashSet<&char> = HashSet::from_iter(chars.iter());
    chars.len() == set.len()
}
fn part1(input: &str) {
    let lines = lines_from_file(input);
    let line = lines.first().unwrap();
    let chars: Vec<char> = line.chars().collect();
    for i in 4..chars.len() {
        if chars_are_unique((chars[i - 4..i]).to_vec()) {
            println!("part 1: {}", i);
            break;
        }
    }
}

fn part2(input: &str) {
    let lines = lines_from_file(input);
    let line = lines.first().unwrap();
    let chars: Vec<char> = line.chars().collect();
    for i in 14..chars.len() {
        if chars_are_unique((chars[i - 14..i]).to_vec()) {
            println!("part 2: {}", i);
            break;
        }
    }
}
