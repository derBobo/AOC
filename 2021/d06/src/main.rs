use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "SMPLEINPUT06.txt";
    let simulationtime = 256;
    let input = File::open(path).expect("file not found!");
    let buffered = BufReader::new(input);
    let mut population: Vec<u64> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];

    for line in buffered.lines() {
        population = line
            .unwrap()
            .split(",")
            .map(|x| x.parse::<i32>().unwrap_or(0))
            .fold(vec![0, 0, 0, 0, 0, 0, 0, 0, 0], |mut result, x| {
                result[x as usize] += 1;
                result
            });
    }
    for i in 0..simulationtime {
        let new_fishes = population[0];
        population.remove(0);
        population.push(new_fishes);
        population[6] += new_fishes;
    }
    /*for i in 0..256{
    let mut fishestoadd=0;
    for a  in 0..fishage.len(){
        if fishage[a]<=0 && (fishage[a] % 7)==0 {
            fishestoadd=fishestoadd+1;
        }
        fishage[a]= fishage[a]-1;

    }
    for a in 0..fishestoadd{
        fishage.push(8);
    }*/

    println!("{}", population.iter().sum::<u64>());
}
