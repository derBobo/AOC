use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

fn main() {
    let path= "INPUT13.txt";
    let input=File::open(path).expect("file not found!");
    let buffered =BufReader::new(input);
    let mut points:HashSet<(u16,u16)>= HashSet::new();
    let mut folds:Vec<(String,u16)>= Vec::new();
    let mut reading_folds =false;
    for line in buffered.lines(){
        let line_unwrapped =line.unwrap();
        if line_unwrapped.trim()==""{
            reading_folds =true;
            continue;
        }
        else if reading_folds {
            let fold:Vec<&str> = line_unwrapped.split(" ").collect::<Vec<&str>>()[2].split("=").collect();
            folds.push((fold[0].to_string(),fold[1].parse().unwrap()));
        }
        else{
            let point:Vec<u16>= line_unwrapped.split(",").map(|x| x.parse().unwrap()).collect();
            points.insert((point[0],point[1]));
        }
    }
    let mut i=0;
    for fold in folds{
        if fold.0.eq("y"){
           points =foldy(fold.1,points);
        }else {
            points=foldx(fold.1,points);
        }
        i+=1;
        println!("After Step {} there are {} points left.",i,points.len())
    }
    let output_size=(50,10);
    for y in 0..=output_size.1{
        for x in 0..=output_size.0{
            if points.contains(&(x,y)){
                print!("█");
            }
            else{
                print!(" ");
            }
        }
        println!();
    }
}
fn foldx(x:u16,points:HashSet<(u16,u16)>)->HashSet<(u16,u16)>{
    let mut newPoints=HashSet::new();
    for point in points{
        if point.0<x {
            newPoints.insert(point);
        }else {
            newPoints.insert((2*x-point.0,point.1));
        }
    }

    return newPoints;
}
fn foldy(y:u16,points:HashSet<(u16,u16)>)->HashSet<(u16,u16)>{
    let mut newPoints=HashSet::new();
    for point in points{
        if point.1<y {
            newPoints.insert(point);
        }else {
            newPoints.insert((point.0,2*y-point.1));
        }
    }

    return newPoints;
}