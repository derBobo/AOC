use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let input = "INPUT10.txt";
    let cycles = parse(input);
    part1(&cycles);
    part2(&cycles);
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
fn parse(path: &str) -> HashMap<u32, i32> {
    let mut res = HashMap::new();
    let mut x = 1;
    let mut cycle = 0;
    let lines = lines_from_file(path);
    for line in lines {
        if line.eq("noop") {
            cycle += 1;
            res.insert(cycle, x);
        } else {
            cycle += 2;
            res.insert(cycle - 1, x);
            res.insert(cycle, x);
            x += line.split(" ").last().unwrap().parse::<i32>().unwrap();
        }
    }

    res
}

fn part1(input: &HashMap<u32, i32>) {
    let res: i32 = [20, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|x| x * input[&(x as u32)])
        .sum();
    println!("part 1 :{}", res);
}
fn part2(cycles: &HashMap<u32, i32>) {
    println!("part2:");
    for y in 0..6 {
        for x in 1..41 {
            let cycle = x + y * 40;
            let cur_value = cycles[&(cycle as u32)];
            if x == cur_value || x - 2 == cur_value || x - 1 == cur_value {
                print!("#")
            } else {
                print!(".");
            }
        }
        println!();
    }
}
