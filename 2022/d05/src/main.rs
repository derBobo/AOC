use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone)]
struct Input {
    stacks: Vec<Vec<char>>,
    moves: Vec<(usize, usize, usize)>,
}
fn parse(path: &str) -> Input {
    let lines = lines_from_file(path);
    let mut moves = Vec::new();
    let mut stacks = Vec::new();
    let mut stacks_finished = false;
    for mut line in lines {
        if stacks_finished {
            line = line
                .replace("move ", "")
                .replace("from ", "")
                .replace("to ", "");
            let numbers = line.split(" ").collect::<Vec<&str>>();
            moves.insert(
                0,
                (
                    numbers[0].parse().unwrap(),
                    numbers[1].parse().unwrap(),
                    numbers[2].parse().unwrap(),
                ),
            )
        } else {
            if line.len() == 0 {
                stacks_finished = true;
            } else if line.contains("[") {
                if stacks.is_empty() {
                    for _ in 0..(line.len() + 1) / 4 {
                        stacks.push(Vec::new());
                    }
                }
                let chars: Vec<char> = line.chars().collect();
                for i in 0..stacks.len() {
                    if chars[i * 4 + 1].ne(&' ') {
                        stacks[i].insert(0, chars[i * 4 + 1]);
                    }
                }
            }
        }
    }

    Input { stacks, moves }
}
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn part1(mut input: Input) {
    let mut stacks = input.stacks.clone();
    while !input.moves.is_empty() {
        let mut cur = input.moves.pop().unwrap();
        while cur.0 != 0 {
            let ch = stacks[cur.1 - 1].pop().unwrap();
            stacks[cur.2 - 1].push(ch);
            cur.0 = cur.0 - 1;
        }
    }
    print_stacks(&stacks);
    print!("part1 :");
    for i in 0..stacks.len(){
        print!("{}",stacks[i].last().unwrap());
    }
    println!();
}
fn part2(mut input: Input) {
    let mut stacks = input.stacks.clone();
    while !input.moves.is_empty() {
        let mut cur = input.moves.pop().unwrap();
        let insert_point=stacks[cur.2 - 1].len();
        while cur.0 != 0 {
            let ch = stacks[cur.1 - 1].pop().unwrap();
            stacks[cur.2 - 1].insert(insert_point,ch);
            cur.0 = cur.0 - 1;
        }
    }
    print_stacks(&stacks);
    print!("part2 :");
    for i in 0..stacks.len(){
        print!("{}",stacks[i].last().unwrap());
    }
    println!();
}
fn print_stacks(stacks: &Vec<Vec<char>>) {
    let max = stacks.iter().map(|x| x.len()).max().unwrap();
    for i in (0..max).rev() {
        for j in 0..stacks.len() {
            match stacks[j].get(i) {
                None => {
                    print!("    ")
                }
                Some(x) => {
                    print!("[{}] ", x)
                }
            }
        }
        println!();
    }
    for i in 1..stacks.len() + 1 {
        print!(" {}  ", i);
    }
    println!();
}
fn main() {
    let input = "INPUT05.txt";
    let taskinput = parse(input);
    part1(taskinput.clone());
    part2(taskinput.clone());

}
