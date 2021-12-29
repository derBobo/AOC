use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread::current;

fn main() {
    let path = "INPUT17.txt";
    let input = File::open(path).expect("file not found!");
    let buffered = BufReader::new(input);
    let lines = buffered
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();
    let x_range_regex = Regex::new(r"(x=)-?\d+..-?\d+").unwrap();
    let y_range_regex = Regex::new(r"(y=)-?\d+..-?\d+").unwrap();
    let x_range_string = x_range_regex.find(&lines[0]).unwrap().as_str();
    let y_range_string = y_range_regex.find(&lines[0]).unwrap().as_str();
    let x_range = x_range_string[2..]
        .split("..")
        .map(|x| i64::from_str_radix(&x.to_string(), 10).unwrap() as f64)
        .collect::<Vec<f64>>();
    let y_range = y_range_string[2..]
        .split("..")
        .map(|x| i64::from_str_radix(&x.to_string(), 10).unwrap() as f64)
        .collect::<Vec<f64>>();
    println!("{:?}", x_range);
    println!("{:?}", y_range);
    let mut current_step = 1;
    let mut max_height = 0;
    let mut avilable_velocities = HashSet::new();
    loop {
        if current_step == 4 || current_step == 5 {
            let c = 0;
        }
        //calculate intervall of start y velocities that would reach the area
        let lower_y_bound =
            (2.0 * y_range[0] / current_step as f64 + (current_step - 1) as f64) / 2.0;
        let upper_y_bound =
            (2.0 * y_range[1] / current_step as f64 + (current_step - 1) as f64) / 2.0;
        if upper_y_bound - lower_y_bound < 0.001 {
            break;
        }

        //calculate intervall of start x velocities that would reach the area
        // x bound only work if target area values>0
        let mut lower_x_bound =
            ((2.0 * x_range[0] / current_step as f64 + (current_step - 1) as f64) / 2.0).ceil()
                as i64;
        let upper_x_bound = ((2.0 * x_range[1] / current_step as f64 + (current_step - 1) as f64)
            / 2.0)
            .floor() as i64;

        if lower_x_bound < current_step {
            lower_x_bound = current_step;
        }

        for y in lower_y_bound.ceil() as i64..=upper_y_bound.floor() as i64 {
            for x in lower_x_bound..=upper_x_bound {
                avilable_velocities.insert((x, y));
            }
        }

        // calculate x velocities that would hit bc they already slowed down to 0
        if x_range[0] < ((current_step * current_step + current_step) / 2) as f64 {
            let lower_limit = (((8.0 * x_range[0] + 1.0).sqrt() - 1.0) / 2.0).ceil() as i64;
            let mut upper_limit = (((8.0 * x_range[1] + 1.0).sqrt() - 1.0) / 2.0).floor() as i64;
            if x_range[1] > ((current_step * current_step + current_step) / 2) as f64 {
                upper_limit = current_step;
            }
            for y in lower_y_bound.ceil() as i64..=upper_y_bound.floor() as i64 {
                for x in lower_limit..=upper_limit {
                    avilable_velocities.insert((x, y));
                }
            }
        }

        //calculate part1
        for i in lower_y_bound.ceil() as i64..=upper_y_bound.floor() as i64 {
            if i > 0 {
                let mut current_max = -i * (-i - 1) / 2;
                if current_step < i {
                    current_max = current_step * (current_step - 2 * i - 1) / 2;
                }
                if max_height < current_max {
                    max_height = current_max;
                }
            }
        }

        current_step += 1;
    }
    println!("{:?}", max_height);
    println!("{}", avilable_velocities.len());
}
