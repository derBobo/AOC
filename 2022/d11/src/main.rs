use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Clone, Debug)]
struct Monkey {
    items: VecDeque<u64>,
    divisor: u64,
    pub operation: fn(u64) -> u64,
    target_true: usize,
    target_false: usize,
}

fn main() {
    let input = "INPUT11.txt";
    let monkeys = parse(input);
    part1(monkeys.clone());
    part2(monkeys.clone());
}

fn part1(mut monkey: Vec<Monkey>) {
    let mut counter = Vec::new();
    let mut kgv = 1;
    for i in 0..monkey.len() {
        counter.push(0);
        kgv = kgv * monkey[i].divisor;
    }
    for _ in 0..20 {
        for i in 0..monkey.len() {
            while !monkey[i].items.is_empty() {
                counter[i] += 1;
                let new_item =
                    (((monkey[i].operation)(monkey[i].items.pop_front().unwrap())) / 3) % kgv;
                if new_item % monkey[i].divisor == 0 {
                    let j = monkey[i].target_true;
                    monkey[j].items.push_back(new_item);
                } else {
                    let j = monkey[i].target_false;
                    monkey[j].items.push_back(new_item);
                }
            }
        }
    }
    counter.sort();
    counter.reverse();
    println!("{:?}", counter);
    println!("{}", counter[0] * counter[1])
}
fn part2(mut monkey: Vec<Monkey>) {
    let mut counter: Vec<u64> = Vec::new();
    let mut kgv = 1;
    for i in 0..monkey.len() {
        counter.push(0);
        kgv = kgv * monkey[i].divisor;
    }
    for _ in 0..10000 {
        for i in 0..monkey.len() {
            while !monkey[i].items.is_empty() {
                counter[i] += 1;
                let new_item = ((monkey[i].operation)(monkey[i].items.pop_front().unwrap())) % kgv;
                if new_item % monkey[i].divisor == 0 {
                    let j = monkey[i].target_true;
                    monkey[j].items.push_back(new_item);
                } else {
                    let j = monkey[i].target_false;
                    monkey[j].items.push_back(new_item);
                }
            }
        }
    }
    counter.sort();
    counter.reverse();
    println!("{:?}", counter);
    println!("{}", counter[0] * counter[1])
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
fn parse(path: &str) -> Vec<Monkey> {
    let lines = lines_from_file(path);
    let mut res = Vec::new();
    let mut items: Vec<u64> = Vec::new();
    let mut divisor: u64 = 0;
    let mut target_true: usize = 0;
    let mut target_false: usize = 0;
    let mut operation: fn(u64) -> u64 = { |x| x };

    for line in lines {
        if line.is_empty() {
            res.push(Monkey {
                items: VecDeque::from(items.clone()),
                divisor,
                operation,
                target_true,
                target_false,
            })
        } else if line.starts_with("  Starting items:") {
            let temp = line.replace("  Starting items:", "").replace(" ", "");
            items = temp
                .split(",")
                .into_iter()
                .map(|x| x.parse().unwrap())
                .collect();
        } else if line.starts_with("  Operation: new = ") {
            let temp = &line.replace("  Operation: new = ", "")[..];

            operation = match temp {
                "old * old" => |x| x * x,
                "old * 13" => |x| x * 13,
                "old * 19" => |x| x * 19,
                "old * 7" => |x| x * 7,
                "old + 6" => |x| x + 6,
                "old + 3" => |x| x + 3,
                "old + 2" => |x| x + 2,
                "old + 4" => |x| x + 4,
                "old + 8" => |x| x + 8,
                _ => {
                    panic!("could not parse Line {}", line);
                }
            }
        } else if line.starts_with("  Test: divisible by ") {
            divisor = line.replace("  Test: divisible by ", "").parse().unwrap();
        } else if line.starts_with("    If true: throw to monkey ") {
            target_true = line
                .replace("    If true: throw to monkey ", "")
                .parse()
                .unwrap();
        } else if line.starts_with("    If false: throw to monkey ") {
            target_false = line
                .replace("    If false: throw to monkey ", "")
                .parse()
                .unwrap();
        }
    }
    res.push(Monkey {
        items: VecDeque::from(items.clone()),
        divisor,
        operation,
        target_true,
        target_false,
    });
    res
}
