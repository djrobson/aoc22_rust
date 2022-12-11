use std::fs;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    // Open the file in read-only mode
    let file = fs::File::open("input10.txt")?;
    //let file = fs::File::open("sample.txt")?;
    let reader = BufReader::new(file);
    let mut commands: Vec<i32> = Vec::new();
    commands.push(0);// clk 0 doesn't count
    commands.push(1);// start with val 1 at clk 1
    
    for line in reader.lines() {
        let l = line.unwrap();
        let mut words = l.split_whitespace();
        let cmd = words.next().unwrap();
        match cmd {
            "addx" => {commands.push(0);
                        commands.push(words.next().unwrap().parse().unwrap())},
            "noop" => commands.push(0),
            _   => panic!(),
        };
    }
    /*
    let mut x:i32 = 0;
    let mut total: i32 = 0;
    let checkpoints: Vec<usize> = vec![20,60,100,140,180,220];
    for clk in 1..=220 {
        x += commands[clk];
        if checkpoints.contains(&clk) {
            total += clk as i32 * x;
        }
    }

    println!("signal {}", total);
    */

    let mut sprite:i32 = 0;
    let mut clk: usize = 1;
    for y in 0..6 {
        for x in 0..40 {
            sprite += commands[clk];
            if sprite -1 == x || sprite == x || sprite +1 == x {
                print!("#");
            } else {
                print!(" ");
            }
            clk = clk + 1;
        }
        println!("");
    }
    Ok(())
}