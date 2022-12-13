use pathfinding;

use pathfinding::prelude::bfs;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
  fn successors(&self, grid: &Vec<Vec<u8>>) -> Vec<Pos> {
    let &Pos(x, y) = self;
    let my_val = grid[y][x];
    let max_y = grid.len();
    let max_x = grid[y].len();

    let mut neighbors = Vec::new();
    // left x-1,y
    if x >0 && grid[y][x-1] <= my_val +1 {
        neighbors.push(Pos(x-1, y));
    }
    //right x+1,y
    if x < (max_x -1) && grid[y][x+1] <= my_val +1 {
        neighbors.push(Pos(x+1, y));
    }
    if y >0 && grid[y-1][x] <= my_val +1 {
        neighbors.push(Pos(x, y-1));
    }
    //down x, y+1
    if y < (max_y -1) && grid[y+1][x] <= my_val +1 {
        neighbors.push(Pos(x, y+1));
    }
    neighbors
  }
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let mut output = Vec::new();
    for line in input.lines() {
        let mut l = Vec::new();
        for c in line.as_bytes() {
            if c == &b'S' {
                l.push(b'a');
            } else if c == &b'E' {
                l.push(b'z' + 1);
            } else {
                l.push(*c);
            }
        }
        output.push(l);
    }
    return output;
}
fn find_symbol(grid: &Vec<Vec<u8>> , sym: u8) -> Pos {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == sym {
                return Pos(x,y);
            }
        }
    }
    panic!("didn't find sym {}", sym);
}
fn find_symbols(grid: &Vec<Vec<u8>> , sym: u8) -> Vec<Pos> {
    let mut found = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == sym {
                found.push(Pos(x,y));
            }
        }
    }
    found
}
 
 // flood outwards from current value
 // if the next val is present mark that first discovery
 // if flood is finished with no additon then look for val -l...
 
fn main() {
    const IS_SAMPLE: bool = false;
    const INPUT: &str = if IS_SAMPLE {
        include_str!("../sample.txt")
    } else {
        include_str!("../input12.txt")
    };
    let grid = parse_input(INPUT);
    //let x_len = grid[0].len();
    //let y_len = grid.len();
    let start = if IS_SAMPLE {
        Pos(0,0) 
    } else {
        Pos(0,20)
    };

    println!("starting at {} {}", start.0, start.1);
    let end = find_symbols(&grid, b'z'+1)[0];

    let starts = find_symbols(&grid, b'a');

    let mut min_len = usize::MAX;

    for start in starts {
        let result = bfs(&start, |p| p.successors(&grid), |p| *p == end);

        match result {
            Some(v) => {
                if v.len() < min_len {
                    min_len = v.len();
                }
            }
            None => (),
        }
    }
    println!("found a vec with len {}", min_len-1)

}
