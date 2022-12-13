use pathfinding;

use pathfinding::prelude::bfs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
                l.push(b'a'-1);
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
 
 // flood outwards from current value
 // if the next val is present mark that first discovery
 // if flood is finished with no additon then look for val -l...
 
fn main() {
    const INPUT: &str = include_str!("../input12.txt");
    let grid = parse_input(INPUT);
    //let x_len = grid[0].len();
    //let y_len = grid.len();

    let start = find_symbol(&grid, b'a'-1);
    let end = find_symbol(&grid, b'z'+1);

    let result = bfs(&start, |p| p.successors(&grid), |p| *p == end);

    match result {
        Some(v) => println!("found a vec with len {}", v.len()-1),
        None => println!("didn't find a solution"),
    }

}
