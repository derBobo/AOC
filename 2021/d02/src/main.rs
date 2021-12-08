use std::fs::File;
use std::io::{BufRead,  BufReader};

fn main() {
    let file = File::open("INPUT02.txt").unwrap();
    let  reader = BufReader::new(file);
    let (mut depth,mut horizontal,mut aim)=(0,0,0);

    for (_index,line) in reader.lines().enumerate() {
        let line=line.unwrap();
        let instruction=line.split_whitespace().collect::<Vec<&str>>();
        match instruction[0]{
            "forward"=>{ horizontal =horizontal+ instruction[1].parse::<i32>().unwrap();
                        depth=depth+instruction[1].parse::<i32>().unwrap()*aim},
            "down"=> aim =aim+ instruction[1].parse::<i32>().unwrap(),
            "up"=> aim =aim- instruction[1].parse::<i32>().unwrap(),
            _ => {}
        }
    }
    println!("{}",depth*horizontal);
    let u=10;
}
