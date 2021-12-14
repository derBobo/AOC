use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "INPUT08.txt";
    let input = File::open(path).expect("file not found!");
    let buffered = BufReader::new(input);
    let mut part1 = 0;
    let mut part2 = 0;
    for line in buffered.lines() {
        let l = line.unwrap();
        let signals = l.split(" ").filter(|&x| x.ne("|")).collect::<Vec<&str>>();

        let mut mapping: HashMap<i8, Vec<char>> = HashMap::new();
        let mut unknown_signals: VecDeque<Vec<char>> = VecDeque::new();

        for i in 0..10 {
            let mut signal_sorted = signals[i].chars().collect::<Vec<char>>();
            signal_sorted.sort();
            match signal_sorted.len() {
                2 => {
                    mapping.insert(1, signal_sorted);
                }
                3 => {
                    mapping.insert(7, signal_sorted);
                }
                4 => {
                    mapping.insert(4, signal_sorted);
                }
                7 => {
                    mapping.insert(8, signal_sorted);
                }
                _ => unknown_signals.push_back(signal_sorted),
            }
        }
        while unknown_signals.len() > 0 {
            let current_signal = unknown_signals.pop_front().unwrap();
            match current_signal.len() {
                //2,3,5
                5 => {
                    if contains_all(&current_signal, mapping.get(&1).unwrap()) {
                        //if it contains all signals from 1 its a 3
                        mapping.insert(3, current_signal);
                    } else if mapping.contains_key(&9) {
                        // 5 fits into 6 and 9 since using 6 would be cyclic use 9 instead
                        if contains_all(mapping.get(&9).unwrap(), &current_signal) {
                            mapping.insert(5, current_signal);
                        } else {
                            mapping.insert(2, current_signal);
                        }
                    } else {
                        //if doesn't contain 9 try again later
                        unknown_signals.push_back(current_signal);
                    }
                }
                6 => {
                    //0,6,9
                    if contains_all(&current_signal, mapping.get(&4).unwrap()) {
                        // if it contains all signals from 4 ist a 9
                        mapping.insert(9, current_signal);
                    } else if mapping.contains_key(&5) {
                        //5 fits into 6
                        if contains_all(&current_signal, mapping.get(&5).unwrap()) {
                            mapping.insert(6, current_signal);
                        } else {
                            mapping.insert(0, current_signal);
                        }
                    } else {
                        unknown_signals.push_back(current_signal)
                    }
                }
                _ => {
                    println!("failed");
                }
            }
        }
        let mut reversed_mapping: HashMap<&Vec<char>, i8> = HashMap::new();
        for i in 0..10 {
            reversed_mapping.insert(mapping.get(&i).unwrap(), i);
        }
        let u = 0;
        let mut display: i32 = 0;
        for i in 10..signals.len() {
            match signals[i].len() {
                2 | 3 | 4 | 7 => part1 += 1,
                _ => {}
            }
            let mut sorted_signal = signals[i].chars().collect::<Vec<char>>();
            sorted_signal.sort();
            display = display * 10 + *reversed_mapping.get(&sorted_signal).unwrap() as i32;
        }
        part2 += display;
    }
    println!("{}", part1);
    println!("{}", part2);
}

fn contains_all(a: &Vec<char>, b: &Vec<char>) -> bool {
    b.iter().all(|object| a.contains(object))
}
