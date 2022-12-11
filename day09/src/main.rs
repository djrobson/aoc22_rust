use std::fs;
use std::ops::Add;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coord(i32, i32);
/*/
impl PartialEq for Coord {
    fn eq(&self, other: &Coord) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}*/
impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self (
            self.0 + other.0,
            self.1 + other.1,
        )
    }
}
fn pull_direction(head: Coord, tail: Coord) -> Coord {
    //return Coord((head.0 - tail.0)/2, (head.1 - tail.1)/2);
    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;

    match (dx, dy) {
        (0,0) | (0,1) | (0,-1) |
        (1,0) | (1,1) | (1,-1) |
        (-1,0)|(-1,1) |(-1,-1)   => Coord(0,0),
        (2,0)                    => Coord(1,0),
        (-2,0)                   => Coord(-1,0),
        (0,2)                    => Coord(0,1),
        (0,-2)                   => Coord(0,-1),
        (2,1)  | (1,2)  | (2,2 ) => Coord(1,1),
        (2,-1) | (1,-2) | (2,-2) => Coord(1,-1),
        (-2,1) | (-1,2) | (-2,2) => Coord(-1,1),
        (-2,-1)|(-1,-2) | (-2,-2)=> Coord(-1,-1),
        _     => panic!(),
    }
}

fn main() -> io::Result<()> {
    // Open the file in read-only mode
    let file = fs::File::open("input09.txt")?;
    //let file = fs::File::open("sample.txt")?;
    let reader = BufReader::new(file);


    // Create a new vec to store the contents of the file
    let mut visited: HashSet<Coord> = HashSet::new();

    let mut snake:Vec<Coord> = Vec::new();
    let snake_len = 10;
    for _ in 0..snake_len {
        snake.push(Coord(0,0));
    }
    
    for line in reader.lines() {
        let l = line.unwrap();
        let (dir,count_str) = l.split_at(2);
        let delta = match dir {
            "R " => Coord(1,0),
            "L " => Coord(-1,0),
            "U " => Coord(0,1),
            "D " => Coord(0,-1),
            _   => panic!(),
        };
        let count:i32 = count_str.parse().unwrap();
        for _ in 0..count {
            snake[0] = snake[0] + delta;
            for n in 1..snake_len {
                snake[n] = snake[n] + pull_direction(snake[n-1], snake[n]);
            }
            visited.insert(snake[snake_len-1]);
        }
    }

    println!("visited {} spots", visited.len());
    Ok(())
}