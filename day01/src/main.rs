use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::env;

fn main() -> io::Result<()> {
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    single_total().expect("single failed");
    triple_total().expect("triple failed");
    Ok(())
}

fn single_total() -> io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);

    let mut max: i32 = i32::MIN;
    let mut this_total: i32 = 0;

    for line in reader.lines() {
        if let Ok(new_num) = line?.parse::<i32>() {
            this_total = this_total + new_num;
         } else {
            if this_total > max {
                max = this_total;
            }
            this_total = 0;
            
        }
    }
    println!("single {}", max);

    Ok(())
}

fn triple_total() -> io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);

    let mut max = (i32::MIN, i32::MIN, i32::MIN);
    let mut this_total: i32 = 0;

    for line in reader.lines() {
        if let Ok(new_num) = line?.parse::<i32>() {
            this_total = this_total + new_num;
         } else {
            if this_total > max.0 {
                max = (this_total, max.0, max.1);
            } else if this_total > max.1 {
                max = (max.0, this_total, max.1);
            } else if this_total > max.2 {
                max = (max.0, max.1, this_total);
            }
            this_total = 0;
            
        }
    }
    println!("triple {}", [max.0, max.1,max.2].iter().sum::<i32>());

    Ok(())
}