use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let input = "..\\INPUT07.txt";
    let lines = lines_from_file(input);
    let mut dirs = Vec::new();
    let total_size = parse(&mut lines.into_iter(), &mut dirs);
    println!("part 1:{}", dirs.iter().filter(|x| **x<=100000).sum::<u32>());
    let fs_size=70000000;
    let space_needed=30000000;
    let space_to_free=total_size+space_needed-fs_size;
    let mut sorted_dirs =dirs.into_iter().filter(|x|*x>=space_to_free).collect::<Vec<u32>>();
    sorted_dirs.sort();
    println!("part 2 : {}",sorted_dirs.first().unwrap() );
}
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
fn parse(lines: &mut dyn Iterator<Item = String>, dirs: &mut Vec<u32>) -> u32 {
    let mut size = 0;

    loop {
        let cur_line_o = lines.next();
        match cur_line_o {
            None => {
                break;
            }
            Some(line) => {
                if line.starts_with("$") {
                    if line.starts_with("$ cd ..") {
                        break;
                    } else if line.starts_with("$ cd") {
                        size += parse(lines, dirs);
                    }
                } else if !line.starts_with("dir") {
                    size += line.split(" ").collect::<Vec<&str>>()[0]
                        .parse::<u32>()
                        .unwrap();
                }
            }
        }
    }
    dirs.push(size);
    size
}
