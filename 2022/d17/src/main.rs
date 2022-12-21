use crate::Blocktype::{Line, Minus, Plus, Reversedl, Square};
use crate::Direction::{Left, Right};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
}
#[derive(Copy, Clone)]
enum Blocktype {
    Minus,
    Plus,
    Reversedl,
    Line,
    Square,
}
impl Blocktype {
    fn height(&self) -> usize {
        match self {
            Minus => 1,
            Plus | Reversedl => 3,
            Line => 4,
            Square => 2,
        }
    }
}
fn main() {
    let input = "INPUT17.txt";
    let directions = parse(input);
    let blocks = vec![Minus, Plus, Reversedl, Line, Square];
    part1(&directions, &blocks);
    part2(&directions, &blocks);
}

fn part1(directions: &Vec<Direction>, blocks: &Vec<Blocktype>) {
    println!("part 1: {}", height_after_rounds(directions, blocks, 2022))
}
fn part2(directions: &Vec<Direction>, blocks: &Vec<Blocktype>) {
    println!(
        "part 2: {}",
        height_after_rounds(directions, blocks, 1000000000000)
    )
}

fn height_after_rounds(
    directions: &Vec<Direction>,
    blocks: &Vec<Blocktype>,
    rounds: usize,
) -> usize {
    let filter_length = 35;
    let mut playfield = Vec::new();
    let mut dir_index = 0;
    let mut block_index = 0;
    let mut round = 0;
    while playfield.len() < filter_length {
        let res = do_x_rounds(&playfield, directions, blocks, block_index, dir_index, 1);
        playfield = res.0;
        dir_index = res.1;
        block_index = res.2;
        round += 1;
    }
    let mut seen = HashMap::new();
    for cur in round..rounds {
        if let Some((old, old_height)) = seen.insert(
            playfield[playfield.len() - filter_length..].to_vec(),
            (cur, playfield.len()),
        ) {
            //found repetition
            let delta_round = cur - old;
            let delta_height = playfield.len() - old_height;
            let n = (rounds - cur) / delta_round;

            let remainder = (rounds - cur) % delta_round;
            playfield = do_x_rounds(
                &playfield,
                directions,
                blocks,
                block_index,
                dir_index,
                remainder,
            )
            .0;
            return playfield.len() + n * delta_height;
        }
        let res = do_x_rounds(&playfield, directions, blocks, block_index, dir_index, 1);
        playfield = res.0;
        dir_index = res.1;
        block_index = res.2;
        round += 1;
    }
    playfield.len()
}
fn do_x_rounds(
    field: &Vec<Vec<bool>>,
    directions: &Vec<Direction>,
    blocks: &Vec<Blocktype>,
    block_index: usize,
    dir_index: usize,
    rounds: usize,
) -> (Vec<Vec<bool>>, usize, usize) {
    let mut playfield = field.clone();
    let mut cur_dir_ind = dir_index;

    for i in 0..rounds {
        let cur_block = blocks[(block_index + i) % blocks.len()];
        let mut cur_position = (2, playfield.len() + 3);
        loop {
            let cur_dir = directions[cur_dir_ind];
            cur_dir_ind = (cur_dir_ind + 1) % directions.len();
            if can_move(&cur_dir, &cur_block, &playfield, &cur_position) {
                match cur_dir {
                    Left => cur_position.0 -= 1,
                    Right => cur_position.0 += 1,
                }
            }
            if can_drop(&cur_block, &playfield, &cur_position) {
                cur_position.1 -= 1
            } else {
                while cur_position.1 + cur_block.height() > playfield.len() {
                    playfield.push(vec![false, false, false, false, false, false, false]);
                }
                match cur_block {
                    Minus => {
                        playfield[cur_position.1][cur_position.0] = true;
                        playfield[cur_position.1][cur_position.0 + 1] = true;
                        playfield[cur_position.1][cur_position.0 + 2] = true;
                        playfield[cur_position.1][cur_position.0 + 3] = true;
                    }
                    Plus => {
                        playfield[cur_position.1][cur_position.0 + 1] = true;
                        playfield[cur_position.1 + 1][cur_position.0] = true;
                        playfield[cur_position.1 + 1][cur_position.0 + 1] = true;
                        playfield[cur_position.1 + 1][cur_position.0 + 2] = true;
                        playfield[cur_position.1 + 2][cur_position.0 + 1] = true;
                    }
                    Reversedl => {
                        playfield[cur_position.1][cur_position.0] = true;
                        playfield[cur_position.1][cur_position.0 + 1] = true;
                        playfield[cur_position.1][cur_position.0 + 2] = true;
                        playfield[cur_position.1 + 1][cur_position.0 + 2] = true;
                        playfield[cur_position.1 + 2][cur_position.0 + 2] = true;
                    }
                    Line => {
                        playfield[cur_position.1][cur_position.0] = true;
                        playfield[cur_position.1 + 1][cur_position.0] = true;
                        playfield[cur_position.1 + 2][cur_position.0] = true;
                        playfield[cur_position.1 + 3][cur_position.0] = true;
                    }
                    Square => {
                        playfield[cur_position.1][cur_position.0] = true;
                        playfield[cur_position.1 + 1][cur_position.0] = true;
                        playfield[cur_position.1][cur_position.0 + 1] = true;
                        playfield[cur_position.1 + 1][cur_position.0 + 1] = true;
                    }
                }
                //print_area(&playfield);
                break;
            }
        }
    }
    (
        playfield,
        cur_dir_ind,
        (block_index + rounds) % blocks.len(),
    )
}

fn can_drop(block: &Blocktype, playfield: &Vec<Vec<bool>>, position: &(usize, usize)) -> bool {
    if position.1 > playfield.len() {
        true
    } else if position.1 == 0 {
        false
    } else {
        match block {
            Minus => {
                !playfield[position.1 - 1][position.0]
                    && !playfield[position.1 - 1][position.0 + 1]
                    && !playfield[position.1 - 1][position.0 + 2]
                    && !playfield[position.1 - 1][position.0 + 3]
            }
            Plus => {
                !playfield[position.1 - 1][position.0 + 1]
                    && (position.1 + 1 > playfield.len()
                        || (!playfield[position.1][position.0]
                            && !playfield[position.1][position.0 + 2]))
            }
            Reversedl => {
                !playfield[position.1 - 1][position.0]
                    && !playfield[position.1 - 1][position.0 + 1]
                    && !playfield[position.1 - 1][position.0 + 2]
            }
            Line => !playfield[position.1 - 1][position.0],
            Square => {
                !playfield[position.1 - 1][position.0] && !playfield[position.1 - 1][position.0 + 1]
            }
        }
    }
}

fn can_move(
    direction: &Direction,
    block: &Blocktype,
    playfield: &Vec<Vec<bool>>,
    position: &(usize, usize),
) -> bool {
    if position.1 >= playfield.len() {
        match direction {
            Left => position.0 > 0,
            Right => match block {
                Minus => position.0 < 3,
                Plus | Reversedl => position.0 < 4,
                Line => position.0 < 6,
                Square => position.0 < 5,
            },
        }
    } else {
        match direction {
            Left => {
                position.0 > 0
                    && match block {
                        Minus => !playfield[position.1][position.0 - 1],
                        Plus => {
                            !playfield[position.1][position.0]
                                && (position.1 + 1 >= playfield.len()
                                    || !playfield[position.1 + 1][position.0 - 1])
                                && (position.1 + 2 >= playfield.len()
                                    || !playfield[position.1 + 2][position.0])
                        }
                        Reversedl => {
                            !playfield[position.1][position.0 - 1]
                                && (position.1 + 1 >= playfield.len()
                                    || !playfield[position.1 + 1][position.0 + 1])
                                && (position.1 + 2 >= playfield.len()
                                    || !playfield[position.1 + 2][position.0 + 1])
                        }
                        Line => {
                            !playfield[position.1][position.0 - 1]
                                && (position.1 + 1 >= playfield.len()
                                    || !playfield[position.1 + 1][position.0 - 1])
                                && (position.1 + 2 >= playfield.len()
                                    || !playfield[position.1 + 2][position.0 - 1])
                                && (position.1 + 3 >= playfield.len()
                                    || !playfield[position.1 + 3][position.0 - 1])
                        }
                        Square => {
                            !playfield[position.1][position.0 - 1]
                                && (position.1 + 1 >= playfield.len()
                                    || !playfield[position.1 + 1][position.0 - 1])
                        }
                    }
            }
            Right => match block {
                Minus => position.0 < 3 && !playfield[position.1][position.0 + 4],
                Plus => {
                    position.0 < 4
                        && !playfield[position.1][position.0 + 2]
                        && (position.1 + 1 >= playfield.len()
                            || !playfield[position.1 + 1][position.0 + 3])
                        && (position.1 + 2 >= playfield.len()
                            || !playfield[position.1 + 2][position.0 + 2])
                }
                Reversedl => {
                    position.0 < 4
                        && !playfield[position.1][position.0 + 3]
                        && (position.1 + 1 >= playfield.len()
                            || !playfield[position.1 + 1][position.0 + 3])
                        && (position.1 + 2 >= playfield.len()
                            || !playfield[position.1 + 2][position.0 + 3])
                }
                Line => {
                    position.0 < 6
                        && !playfield[position.1][position.0 + 1]
                        && (position.1 + 1 >= playfield.len()
                            || !playfield[position.1 + 1][position.0 + 1])
                        && (position.1 + 2 >= playfield.len()
                            || !playfield[position.1 + 2][position.0 + 1])
                        && (position.1 + 3 >= playfield.len()
                            || !playfield[position.1 + 3][position.0 + 1])
                }
                Square => {
                    position.0 < 5
                        && !playfield[position.1][position.0 + 2]
                        && (position.1 + 1 >= playfield.len()
                            || !playfield[position.1 + 1][position.0 + 2])
                }
            },
        }
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse Line"))
        .collect()
}

fn parse(path: &str) -> Vec<Direction> {
    let lines = lines_from_file(path);
    lines
        .first()
        .unwrap()
        .chars()
        .map(|x| match x {
            '>' => Right,
            _ => Left,
        })
        .collect()
}
