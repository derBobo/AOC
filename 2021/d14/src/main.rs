use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};



fn main() {
    let steps = 40;
    let path = "INPUT14.txt";
    let input = File::open(path).expect("file not found!");
    let buffered = BufReader::new(input);
    let mut input: String = String::new();
    let mut found_start = false;
    let mut conversion_map: HashMap<String, [String; 2]> = HashMap::new();
    for line in buffered.lines() {
        let line_unwrapped = line.unwrap();
        if line_unwrapped.trim() == "" {
            found_start = true;
            continue;
        }
        if found_start {
            let a = line_unwrapped.split(" -> ").collect::<Vec<&str>>();
            let mut first_pair = String::new();
            first_pair.push(a[0].chars().collect::<Vec<char>>()[0]);
            first_pair += a[1];
            let mut second_pair = a[1].to_string();
            second_pair.push(a[0].chars().collect::<Vec<char>>()[1]);
            conversion_map.insert(a[0].to_string(), [first_pair, second_pair]);
        } else {
            input = line_unwrapped;
        }
    }
    let mut pairs:Vec<(String,i64)> = Vec::new();
    let mut char_count:Vec<(char,i64)>=Vec::new();
    let char_input: Vec<char> = input.chars().collect();
    char_count.push((char_input[0],1));
    char_count.push((*char_input.last().unwrap(), 1));
    for i in 1..char_input.len() {
        let  current_pair=format!("{}{}",char_input[i-1],char_input[i]);
        let mut contained=false;
        for j in 0..pairs.len(){
            if pairs[j].0==current_pair{
                contained=true;
                pairs[j].1+=1;
            }
        }
        if !contained{
            pairs.push((current_pair,1));
        }
    }
   for i in 0..steps {
       pairs = make_step(pairs, &conversion_map);
   }
    for pair in pairs{
        for char in pair.0.chars(){
            let mut contained=false;
            for  i in 0..char_count.len(){
                if char_count[i].0==char{
                    contained=true;
                    char_count[i].1+=pair.1;
                }
            }
            if !contained{
                char_count.push((char,pair.1));
            }
        }
    }
    char_count=char_count.iter().map(|x|(x.0,x.1/2)).collect();
    char_count.sort_by(|a,b| b.1.cmp(&a.1));
    println!("{}",char_count[0].1-char_count[char_count.len()-1].1);
}
fn make_step(pairs:Vec<(String,i64)>,conversion_map: &HashMap<String, [String; 2]>)->Vec<(String,i64)>{
    let mut new_pairs:Vec<(String,i64)>= Vec::new();
    for pair in pairs{
        for new_pair in conversion_map.get(&*pair.0).unwrap(){
            let mut contained=false;
            for i in 0..new_pairs.len() {
                if new_pairs[i].0==*new_pair{
                    new_pairs[i].1+=pair.1;
                    contained=true;
                }
            }
            if !contained{
                new_pairs.push((new_pair.to_string(), pair.1));
            }
        }
    }
    return new_pairs;
}


