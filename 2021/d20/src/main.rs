use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "INPUT20.txt";
    let input = File::open(path).expect("file not found!");
    let buffered = BufReader::new(input);
    let lines: Vec<String> = buffered
        .lines()
        .map(|x| x.unwrap().to_string())
        .filter(|x| x != "")
        .collect();
    let mut algorithm: Vec<u16> = Vec::new();
    let mut image: Vec<Vec<u16>> = Vec::new();
    for line in lines {
        if algorithm.is_empty() {
            algorithm = line
                .chars()
                .map(|x| match x {
                    '#' => 1,
                    _ => 0,
                })
                .collect();
        } else {
            image.push(
                line.chars()
                    .map(|x| match x {
                        '#' => 1,
                        _ => 0,
                    })
                    .collect(),
            )
        };
    }
    print(&image);
    let mut default = 0;
    for i in 0..50 {
        image = enhance(&algorithm, image, default);
        default = (if default == 1 {
            algorithm[0b111111111]
        } else {
            algorithm[0]
        });
    }
    print(&image);
    let part1: u32 = image
        .iter()
        .fold(0, |sum, x| sum + x.iter().fold(0, |sum, &x| sum + x as u32));
    println!("{}", part1);
}
fn enhance(algorithm: &Vec<u16>, image: Vec<Vec<u16>>, default: u16) -> Vec<Vec<u16>> {
    let mut new_image = Vec::new();
    for i in 0..image.len() + 2 {
        let y = i as i32 - 1;
        let mut row = Vec::new();
        for j in 0..image.len() + 2 {
            let x = j as i32 - 1;
            let mut index = 0;
            for offset_y in [-1, 0, 1] {
                for offset_x in [-1, 0, 1] {
                    let window_x = x + offset_x;
                    let window_y = y + offset_y;
                    index = index << 1;
                    if window_y < 0
                        || window_y >= image.len() as i32
                        || window_x < 0
                        || window_x >= image[window_y as usize].len() as i32
                    {
                        index = index | default;
                    } else {
                        index = index | image[window_y as usize][window_x as usize];
                    }
                }
            }
            row.push(algorithm[index as usize]);
        }
        new_image.push(row);
    }
    return new_image;
}
fn print(image: &Vec<Vec<u16>>) {
    println!("╔{}╗", "═".repeat(image.len()));
    for l in image.iter() {
        print!("║");
        for c in l {
            print!(
                "{}",
                match *c {
                    1 => "█",
                    0 => " ",
                    _ => "E",
                }
            );
        }
        println!("║");
    }
    println!("╚{}╝", "═".repeat(image.len()));
}
