use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "INPUT10.txt";
    let input = File::open(path).expect("file not found!");
    let buffered = BufReader::new(input);
    let mut part1 = 0;
    let mut part2: Vec<u64> = Vec::new();
    'lines: for line in buffered.lines() {
        let mut open_chunks: Vec<char> = Vec::new();
        'chars: for c in line.unwrap().chars() {
            match c {
                '(' | '[' | '{' | '<' => open_chunks.push(c),
                ')' => {
                    if open_chunks.pop().unwrap() != '(' {
                        part1 += 3;
                        println!("found unexpected )");
                        continue 'lines;
                    }
                }
                ']' => {
                    if open_chunks.pop().unwrap() != '[' {
                        part1 += 57;
                        println!("found unexpected ]");
                        continue 'lines;
                    }
                }
                '}' => {
                    if open_chunks.pop().unwrap() != '{' {
                        part1 += 1197;
                        println!("found unexpected }}");
                        continue 'lines;
                    }
                }
                '>' => {
                    if open_chunks.pop().unwrap() != '<' {
                        part1 += 25137;
                        println!("found unexpected >");
                        continue 'lines;
                    }
                }
                _ => {}
            }
        }
        open_chunks.reverse();
        let score:u64 = open_chunks.iter().fold(0, |score, x| {
            match x {
                '(' => score * 5 + 1,
                '[' => score * 5 + 2,
                '{' => score * 5 + 3,
                '<' => score * 5 + 4,
                _ => score
            }
        });
        part2.push(score);
    }
    part2.sort();
    println!("Solution for Part 1: {}", part1);
    println!("Solution for Part 2: {}", part2[part2.len()/2]);
}

