use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn parse(path: &str) -> Vec<((u32, u32), (u32, u32))> {
    let lines = lines_from_file(path);
    lines
        .iter()
        .map(|line| {
            let l = line.split(",").collect::<Vec<&str>>();
            let area1 = l[0].split("-").collect::<Vec<&str>>();
            let area2 = l[1].split("-").collect::<Vec<&str>>();
            (
                (
                    area1[0].parse::<u32>().unwrap(),
                    area1[1].parse::<u32>().unwrap(),
                ),
                (
                    area2[0].parse::<u32>().unwrap(),
                    area2[1].parse::<u32>().unwrap(),
                ),
            )
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
fn part1(areas: &Vec<((u32, u32), (u32, u32))>) {
    let mut counter = 0;
    for area in areas {
        if (area.0 .0 <= area.1 .0 && area.0 .1 >= area.1 .1)
            || (area.0 .0 >= area.1 .0 && area.0 .1 <= area.1 .1)
        {
            counter += 1;
        }
    }
    println!("part 1:{}", counter)
}
fn part2(areas: &Vec<((u32, u32), (u32, u32))>) {
    let mut counter = 0;
    for area in areas {
        if (area.0 .0 >= area.1 .0 && area.0 .0 <= area.1 .1)
            || (area.0 .1 >= area.1 .0 && area.0 .1 <= area.1 .1)
            || (area.1 .0 >= area.0 .0 && area.1 .0 <= area.0 .1)
            || (area.1 .1 >= area.0 .0 && area.1 .1 <= area.0 .1)
        {
            counter += 1;
        }
    }
    println!("part 2:{}", counter)
}
fn main() {
    let input = "..\\INPUT04.txt";
    let areas = parse(input);
    part1(&areas);
    part2(&areas);
}
