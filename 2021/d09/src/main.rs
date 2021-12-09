use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::thread::current;


fn main() {
    let path = "SAMPLEINPUT09.txt";
    let input = File::open(path).expect("file not found!");
    let buffered = BufReader::new(input);
    let temp_map: Vec<Vec<u8>> = buffered.lines().map(|y| y.unwrap().chars().map(|y| (y.to_digit(10).unwrap() as u8)).collect()).collect();
    let mut map: Vec<Vec<u8>> = Vec::new();
    let y_len = temp_map.len() + 1;
    let x_len = temp_map.first().unwrap().len() + 1;
    for y in 0..=y_len {
        map.push(Vec::new());
        for x in 0..=x_len {
            match (y, x) {
                (0, _) => map[y].push(9),
                (yl, _) if yl == y_len => map[y].push(9),
                (_, 0) => map[y].push(9),
                (_, xl) if xl == x_len => map[y].push(9),
                _ => map[y].push(temp_map[y - 1][x - 1]),
            }
        }
    }
    let mut low_points: Vec<(usize, usize)> = Vec::new();
    let mut part1 = 0;
    for y in 1..y_len {
        for x in 1..x_len {
            if map[y][x] < map[y + 1][x] && map[y][x] < map[y - 1][x] && map[y][x] < map[y][x + 1] && map[y][x] < map[y][x - 1] {
                part1 += 1 + map[y][x] as u32;
                low_points.push((y, x));
            }
        }
    }
    let mut basins: Vec<u32> = low_points.iter().map(|&x| {
        let mut points_to_check = Vec::new();
        points_to_check.push(x);
        let mut checked_points: Vec<(usize, usize)> = Vec::new();
        while points_to_check.len() != 0 {
            let current = points_to_check.pop().unwrap();
            if map[current.0 + 1][current.1] != 9 && !points_to_check.contains(&(current.0 + 1, current.1)) && !checked_points.contains(&(current.0 + 1, current.1)) {
                    points_to_check.push((current.0 + 1, current.1));
                }
                if map[current.0 - 1][current.1] != 9 && !points_to_check.contains(&(current.0 - 1, current.1)) && !checked_points.contains(&(current.0 - 1, current.1)) {
                    points_to_check.push((current.0 - 1, current.1));
                }
                if map[current.0][current.1 + 1] != 9 && !points_to_check.contains(&(current.0, current.1 + 1)) && !checked_points.contains(&(current.0, current.1 + 1)) {
                    points_to_check.push((current.0, current.1 + 1));
                }
                if map[current.0][current.1 - 1] != 9 && !points_to_check.contains(&(current.0, current.1 - 1)) && !checked_points.contains(&(current.0, current.1 - 1)) {
                    points_to_check.push((current.0, current.1 - 1));
                }
            checked_points.push(current);
        }
        checked_points.len() as u32
    }).collect();
    basins.sort();
    basins.reverse();
    let part2= basins[0]*basins[1]*basins[2];

    println!("{}", part1);
    println!("{}", part2);
}
