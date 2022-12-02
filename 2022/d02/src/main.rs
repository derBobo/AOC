use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
enum RPS {
    Rock,
    Paper,
    Scissor,
    None,
}

fn parse1(path: &str) -> Vec<(RPS, RPS)> {
    let lines = lines_from_file(path);
    lines
        .iter()
        .map(|line| match line.as_str() {
            "A X" => (RPS::Rock, RPS::Rock),
            "A Y" => (RPS::Rock, RPS::Paper),
            "A Z" => (RPS::Rock, RPS::Scissor),
            "B X" => (RPS::Paper, RPS::Rock),
            "B Y" => (RPS::Paper, RPS::Paper),
            "B Z" => (RPS::Paper, RPS::Scissor),
            "C X" => (RPS::Scissor, RPS::Rock),
            "C Y" => (RPS::Scissor, RPS::Paper),
            "C Z" => (RPS::Scissor, RPS::Scissor),
            &_ => (RPS::None, RPS::None),
        })
        .collect()
}
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
fn calculate(rounds: &Vec<(RPS, RPS)>, part: i32) {
    let sum = rounds.iter().fold(0, |mut sum, x| {
        sum += match x {
            (RPS::Rock, RPS::Rock) => 4,
            (RPS::Rock, RPS::Paper) => 8,
            (RPS::Rock, RPS::Scissor) => 3,
            (RPS::Paper, RPS::Rock) => 1,
            (RPS::Paper, RPS::Paper) => 5,
            (RPS::Paper, RPS::Scissor) => 9,
            (RPS::Scissor, RPS::Rock) => 7,
            (RPS::Scissor, RPS::Paper) => 2,
            (RPS::Scissor, RPS::Scissor) => 6,
            _ => 0,
        };
        sum
    });
    println!("result part {}: {}", part, sum);
}
fn part2(rounds: &Vec<(RPS, RPS)>) {
    //convert to correct result
    let rounds2 = rounds
        .iter()
        .map(|x| match x {
            (RPS::Rock, RPS::Rock) => (RPS::Rock, RPS::Scissor),
            (RPS::Rock, RPS::Paper) => (RPS::Rock, RPS::Rock),
            (RPS::Rock, RPS::Scissor) => (RPS::Rock, RPS::Paper),
            (RPS::Paper, RPS::Rock) => (RPS::Paper, RPS::Rock),
            (RPS::Paper, RPS::Paper) => (RPS::Paper, RPS::Paper),
            (RPS::Paper, RPS::Scissor) => (RPS::Paper, RPS::Scissor),
            (RPS::Scissor, RPS::Rock) => (RPS::Scissor, RPS::Paper),
            (RPS::Scissor, RPS::Paper) => (RPS::Scissor, RPS::Scissor),
            (RPS::Scissor, RPS::Scissor) => (RPS::Scissor, RPS::Rock),
            _ => (RPS::None, RPS::None),
        })
        .collect();
    calculate(&rounds2, 2);
}
fn main() {
    let input = "..\\INPUT02.txt";
    let rounds = parse1(input);
    calculate(&rounds, 1);
    part2(&rounds);
}
