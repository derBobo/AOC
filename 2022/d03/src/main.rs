use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn part1(path: &str) {
    let lines = lines_from_file(path);
    let mut sum: u32 = 0;
    for line in lines {
        let half = line.split_at(line.len() / 2);
        let items1: HashSet<char> = half.0.chars().fold(HashSet::new(), |mut set, ch| {
            set.insert(ch);
            set
        });
        let items2: HashSet<char> = half.1.chars().fold(HashSet::new(), |mut set, ch| {
            set.insert(ch);
            set
        });
        let intersection: Vec<&char> = items1.intersection(&items2).collect();
        sum += char_to_prio(intersection.first().unwrap())
    }
    println!("part 1: {}", sum);
}
fn part2(path: &str) {
    //sadly im too stupid to intersect 3 HashSets
    let mut lines = lines_from_file(path)
        .iter()
        .map(|line| {
            let mut chs = line.chars().collect::<Vec<char>>();
            chs.sort();
            VecDeque::from(chs)
        })
        .collect::<Vec<VecDeque<char>>>();

    let mut sum: u32 = 0;
    for i in 0..lines.len() / 3 {
        loop {
            let values = vec![lines[i * 3][0], lines[i * 3 + 1][0], lines[i * 3 + 2][0]];
            let max = values.iter().max().unwrap();
            if lines[i * 3][0] == *max && lines[i * 3 + 1][0] == *max && lines[i * 3 + 2][0] == *max
            {
                sum += char_to_prio(max);
                break;
            } else {
                for j in 0..3 {
                    if lines[i * 3 + j][0] != *max {
                        lines[i * 3 + j].pop_front();
                    }
                }
            }
        }
    }
    println!("part 2 {}", sum);
}
fn char_to_prio(ch: &char) -> u32 {
    let mut bytes = [0, 1];
    ch.encode_utf8(&mut bytes);
    match bytes[0] >= 97 {
        true => bytes[0] as u32 - 96,
        false => bytes[0] as u32 - 38,
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
fn main() {
    part1("INPUT03.txt");
    part2("INPUT03.txt");
}
