use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "INPUT11.txt";
    let input = File::open(path).expect("file not found!");
    let buffered = BufReader::new(input);
    let mut part1 = 0;
    let mut part2 = 0;
    let mut octopuses: Vec<Vec<u32>> = buffered.lines().map(|y| y.unwrap().chars().map(|x| x.to_digit(10).unwrap()).collect()).collect();
    let mut num_of_steps = 100;

    for i in 1..=num_of_steps {
        let current_flashes=make_step(&mut octopuses);
        part1+=current_flashes;
        if current_flashes == (octopuses.len() * octopuses[0].len()) as i32 {
            part2 = i;
        }
    }
    while part2==0 {
        num_of_steps+=1;
        if make_step(&mut octopuses)== (octopuses.len() * octopuses[0].len()) as i32{
            part2=num_of_steps;
        }

    }

    println!("{}",part1);
    println!("{}",part2);
}
fn make_step(octopuses: &mut Vec<Vec<u32>>)->i32{
    let mut flashes=0;
    for y in 0..octopuses.len() {
        for x in 0..octopuses[y].len() {
            octopuses[y][x] += 1;
            if octopuses[y][x] == 10 {
                flash_neighbours(y as i32, x as i32, octopuses);
            }
        }
    }
    for y in 0..octopuses.len() {
        for x in 0..octopuses[y].len() {
            if octopuses[y][x] > 9 {
                octopuses[y][x]=0;
                flashes+=1;
            }
        }
    }
    return flashes;
}

fn flash_neighbours(y:i32, x: i32, octopuses: &mut Vec<Vec<u32>>) {
    for i in [-1, 0, 1] {
        for j in [-1, 0, 1] {
            if i == 0 && j == 0 {
                continue;
            }
            if y + i >= 0 && y + i < octopuses.len() as i32&& x + j >= 0 && x + j < octopuses[(y + i) as usize].len() as i32 {
                octopuses[(y + i) as usize][(x + j) as usize]+=1;
                if octopuses[(y + i) as usize][(x + j) as usize]==10{
                    flash_neighbours(y+i, x+j, octopuses);
                }
            }
        }
    }
}
