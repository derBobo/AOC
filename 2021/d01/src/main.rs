use std::fs::File;
use std::io::{BufRead,  BufReader};


fn main() {
    let mut increases=0;
    let (mut last1,mut last2,mut last3)=(i32::MAX,i32::MAX,i32::MAX);
    let file = File::open("INPUT01.txt").unwrap();
    let  reader = BufReader::new(file);


        for (_index,line) in reader.lines().enumerate() {
            let line=line.unwrap();
            if line.trim().parse::<i32>().is_ok(){
               match line.trim().parse::<i32>(){
                   Ok(n)=>{
                       let current=n;
                       if last3!= i32::MAX && current+last1+last2>last1+last2+last3{
                           increases = increases+1;
                       }
                       last3=last2;
                       last2=last1;
                       last1=current;
                   }
                   Err(e) => {println!("error")}
               }

            }
        }

    println!("{}",increases);
}
