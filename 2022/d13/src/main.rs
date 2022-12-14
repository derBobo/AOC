use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let input = "INPUT13.txt";
    let pairs = parse(input);
    part1(&pairs);
    part2(&pairs);
}

fn part2(pairs: &Vec<(String, String)>) {
    let mut packets = pairs.into_iter().fold(Vec::new(), |mut vec, pair| {
        vec.push(pair.0.clone());
        vec.push(pair.1.clone());
        vec
    });
    packets.push("[[2]]".to_string());
    packets.push("[[6]]".to_string());
    packets.sort_by(|a, b| compare(a, b));
    println!(
        "part 2:{}",
        (packets.iter().position(|x| x.eq("[[2]]")).unwrap() + 1)
            * (packets.iter().position(|x| x.eq("[[6]]")).unwrap() + 1)
    );
}

fn part1(pairs: &Vec<(String, String)>) {
    let mut sum = 0;
    for i in 0..pairs.len() {
        if compare(&pairs[i].0, &pairs[i].1) == Ordering::Less {
            sum += i + 1;
        }
    }
    println!("part 1: {}", sum);
}

fn compare(left: &String, right: &String) -> Ordering {
    if left.eq(right) {
        Ordering::Equal
    } else if left.contains("[") && right.contains("[") {
        if left.eq("[]") {
            Ordering::Less
        } else if right.eq("[]") {
            Ordering::Greater
        } else {
            let lsplit = split_list(left);
            let rsplit = split_list(right);
            compare_list(&lsplit, &rsplit)
        }
    } else if left.contains("[") && !right.contains("[") {
        let mut new_right = "[".to_string();
        new_right.push_str(right.as_str());
        new_right.push_str("]");
        compare(left, &new_right)
    } else if !left.contains("[") && right.contains("[") {
        let mut new_left = "[".to_string();
        new_left.push_str(left.as_str());
        new_left.push_str("]");
        compare(&new_left, right)
    } else {
        if left.eq("") {
            Ordering::Less
        } else if right.eq("") {
            Ordering::Greater
        } else {
            let ileft: u32 = left.parse().unwrap();
            let iright: u32 = right.parse().unwrap();
            ileft.cmp(&iright)
        }
    }
}

fn compare_list(left: &Vec<String>, right: &Vec<String>) -> Ordering {
    if left.len() < right.len() {
        for i in 0..left.len() {
            if compare(&left[i], &right[i]) != Ordering::Equal {
                return compare(&left[i], &right[i]);
            }
        }
        Ordering::Less
    } else if left.len() > right.len() {
        for i in 0..right.len() {
            if compare(&left[i], &right[i]) != Ordering::Equal {
                return compare(&left[i], &right[i]);
            }
        }
        Ordering::Greater
    } else {
        for i in 0..right.len() {
            if compare(&left[i], &right[i]) != Ordering::Equal {
                return compare(&left[i], &right[i]);
            }
        }
        Ordering::Equal
    }
}

fn split_list(list: &String) -> Vec<String> {
    let mut split = Vec::new();
    let mut start = 1;
    let mut in_list = 0;
    let chars: Vec<char> = list.chars().collect();
    for i in 1..chars.len() - 1 {
        if i == chars.len() - 2 {
            split.push(list[start..i + 1].to_string());
        } else if chars[i] == ',' && in_list == 0 {
            split.push(list[start..i].to_string());
            start = i + 1;
        } else if chars[i] == '[' {
            in_list += 1;
        } else if chars[i] == ']' {
            in_list -= 1;
        }
    }
    split
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
fn parse(path: &str) -> Vec<(String, String)> {
    let lines = lines_from_file(path);
    let packets: Vec<Vec<String>> = lines.split(|x| x.eq("")).map(|x| x.to_vec()).collect();
    packets
        .into_iter()
        .map(|x| (x[0].clone(), x[1].clone()))
        .collect()
}
