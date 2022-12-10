use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let input = "INPUT08.txt";
    let trees = parse(input);
    part1(&trees);
    part2(&trees);
}

fn part2(trees: &Vec<Vec<u32>>) {
    let mut scores = Vec::new();
    for col in 0..trees.len() {
        for row in 0..trees[0].len() {
            scores.push(get_score(trees, col, row));
        }
    }
    println!("part 2: {}", scores.iter().max().unwrap());
}
fn get_score(trees: &Vec<Vec<u32>>, col: usize, row: usize) -> usize {
    let mut top = 0;
    let mut left = 0;
    let mut right = 0;
    let mut bottom = 0;
    while col - top > 0 && (trees[col - top][row] < trees[col][row] || top == 0) {
        top += 1;
    }
    while col + bottom < trees.len() - 1
        && (trees[col + bottom][row] < trees[col][row] || bottom == 0)
    {
        bottom += 1;
    }
    while row - left > 0 && (trees[col][row - left] < trees[col][row] || left == 0) {
        left += 1;
    }
    while row + right < trees[col].len() - 1
        && (trees[col][row + right] < trees[col][row] || right == 0)
    {
        right += 1;
    }

    top * left * right * bottom
}

fn get_column(trees: &Vec<Vec<u32>>, col: usize) -> Vec<u32> {
    trees.iter().map(|x| x[col]).collect()
}

fn tree_is_visible(trees: &Vec<Vec<u32>>, row: usize, col: usize) -> bool {
    let col_v = get_column(trees, col);
    let row_v = trees[row].clone();
    row == 0
        || col == 0
        || row == trees.len() - 1
        || col == trees[0].len() - 1
        || *(row_v[0..col].iter().max().unwrap()) < trees[row][col]
        || *(row_v[col + 1..row_v.len()].iter().max().unwrap()) < trees[row][col]
        || *(col_v[0..row].iter().max().unwrap()) < trees[row][col]
        || *(col_v[row + 1..col_v.len()].iter().max().unwrap()) < trees[row][col]
}

fn part1(trees: &Vec<Vec<u32>>) {
    let mut counter = 0;
    for x in 0..trees.len() {
        for y in 0..trees[0].len() {
            if tree_is_visible(&trees, x, y) {
                counter += 1;
            }
        }
    }
    println!("part 1: {}", counter);
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
fn parse(path: &str) -> Vec<Vec<u32>> {
    let lines = lines_from_file(path);
    let mut res = Vec::new();
    for line in lines {
        let row = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        res.push(row);
    }
    res
}
