use std::cmp::{max, min, Ordering};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;
use std::path::Path;

fn main() {
    let input = "INPUT15.txt";
    let pairs = parse(input);
    part1(&pairs);
    part2(&pairs);
}

fn part2(pairs: &Vec<((i32, i32), (i32, i32))>) {
    let max = 4000000;
    let mut beacon = (0, 0);
    for y in 0..max + 1 {
        let mut temp_ranges: Vec<Range<i32>> = pairs
            .iter()
            .map(|x| get_x_range(x.0, x.1, y))
            .map(|x| range_clip(&x, &(0..max)))
            .filter(|r| r.is_some())
            .map(|r| r.unwrap())
            .collect();
        temp_ranges.sort_by(sort_range);
        let mut ranges = Vec::new();
        'outer: for i in temp_ranges {
            for j in 0..ranges.len() {
                if ranges_overlap(&i, &ranges[j]) {
                    ranges[j] = range_merge(&i, &ranges[j]);
                    continue 'outer;
                }
            }
            ranges.push(i);
        }
        if ranges.len() == 2 {
            beacon = (ranges.first().unwrap().end + 1, y)
        }
    }

    println!("part 2 : {}", beacon.0 as u64 * 4000000 + beacon.1 as u64);
}
fn part1(pairs: &Vec<((i32, i32), (i32, i32))>) {
    let row = 2000000;
    let mut temp_ranges: Vec<Range<i32>> = pairs
        .iter()
        .map(|x| get_x_range(x.0, x.1, row))
        .filter(|r| r.is_some())
        .map(|r| r.unwrap())
        .collect();
    temp_ranges.sort_by(sort_range);
    let mut ranges = Vec::new();
    'outer: for i in temp_ranges {
        for j in 0..ranges.len() {
            if ranges_overlap(&i, &ranges[j]) {
                ranges[j] = range_merge(&i, &ranges[j]);
                continue 'outer;
            }
        }
        ranges.push(i);
    }
    let res: i32 = ranges
        .iter()
        .map(|r| (r.end + 1 - r.start) as i32)
        .sum::<i32>()
        - pairs
            .iter()
            .filter(|x| x.1 .1 == row)
            .map(|x| x.1)
            .collect::<HashSet<(i32, i32)>>()
            .len() as i32;
    println!("part 1 : {}", res);
}
fn sort_range(r1: &Range<i32>, r2: &Range<i32>) -> Ordering {
    let temp = r1.start.cmp(&r2.start);
    if temp == Ordering::Equal {
        r1.end.cmp(&r2.end)
    } else {
        temp
    }
}
fn range_clip(r: &Option<Range<i32>>, boundry: &Range<i32>) -> Option<Range<i32>> {
    if r.is_some() {
        let range = r.clone().unwrap();
        if range.start <= boundry.end && boundry.start <= range.end {
            return Some(max(range.start, boundry.start)..min(range.end, boundry.end));
        }
    }
    None
}
fn range_merge(r1: &Range<i32>, r2: &Range<i32>) -> Range<i32> {
    min(r1.start, r2.start)..max(r1.end, r2.end)
}
fn ranges_overlap(r1: &Range<i32>, r2: &Range<i32>) -> bool {
    r1.start <= r2.end + 1 && r2.start <= r1.end + 1
}
fn manhattan_distance(p1: &(i32, i32), p2: &(i32, i32)) -> i32 {
    (p1.1 - p2.1).abs() + (p1.0 - p2.0).abs()
}
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
fn get_x_range(signal: (i32, i32), beacon: (i32, i32), y: i32) -> Option<Range<i32>> {
    let man = manhattan_distance(&signal, &beacon);
    let dist_to_row = (signal.1 - y).abs();
    if man < dist_to_row {
        None
    } else {
        Some(signal.0 - (man - dist_to_row)..signal.0 + (man - dist_to_row))
    }
}
/*fn no_beacon_in_range(
    pairs: &Vec<((i32, i32), (i32, i32))>,
    x_range: Range<i32>,
    y_range: Range<i32>,
) -> HashSet<(i32, i32)> {
    let mut no_beacons = HashSet::new();
    for i in 0..pairs.len() {
        let man = manhattan_distance(&pairs[i].0, &pairs[i].1);
        let x_min = max(pairs[i].0 .0 - man, x_range.start);
        let x_max = min(pairs[i].0 .0 + man + 1, x_range.end);
        for x in x_min..x_max {
            let n = (man - (x - pairs[i].0 .0).abs()).abs();
            let y_min = max(pairs[i].0 .1 - n, y_range.start);
            let y_max = min(pairs[i].0 .1 + n + 1, y_range.end);
            for y in y_min..y_max {
                no_beacons.insert((x, y));
            }
        }
    }
    for i in 0..pairs.len() {
        no_beacons.remove(&(pairs[i].1 .0, pairs[i].1 .1));
    }
    no_beacons
}*/
fn parse(path: &str) -> Vec<((i32, i32), (i32, i32))> {
    let lines = lines_from_file(path);
    let signals_and_beacons: Vec<((i32, i32), (i32, i32))> = lines
        .iter()
        .map(|x| {
            let temp = x
                .replace("Sensor at x=", "")
                .replace(" y=", "")
                .replace(": closest beacon is at x=", ",");
            let split: Vec<&str> = temp.split(",").collect();
            (
                (
                    split[0].parse::<i32>().unwrap(),
                    split[1].parse::<i32>().unwrap(),
                ),
                (
                    split[2].parse::<i32>().unwrap(),
                    split[3].parse::<i32>().unwrap(),
                ),
            )
        })
        .collect();
    signals_and_beacons
}
