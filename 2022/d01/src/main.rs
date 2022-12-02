use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
fn parse(path: &str) -> Vec<i32> {
    let lines = lines_from_file(path);
    let mut sum = 0;
    let mut result = Vec::new();
    for line in lines {
        match line.as_str() {
            "" => {
                result.push(sum);
                sum = 0;
            },
            _ => {
                sum += line.parse::<i32>().unwrap();
            },
        }
    }
    result.push(sum);
    result.sort();
    result
}
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
fn part1(calories:&Vec<i32>){
    println!("result part 1 {}",calories.last().unwrap());
}
fn part2(calories:&Vec<i32>){

    println!("result part 2 {}",calories.iter().rev().take(3).sum::<i32>());
}
fn main() {
    let input = "..\\INPUT01.txt";
    let calories = parse(input);
    part1(&calories);
    part2(&calories);

}
