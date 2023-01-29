#![feature(is_some_and)]
use std::collections::{VecDeque,HashMap};
//use rayon::prelude::*;

const IS_SAMPLE: bool = false;

struct Grid {
    grid_at_time: Vec<Vec<Vec<Vec<u8>>>>,
    move_at_time: HashMap<((usize,usize),usize),Vec::<(usize,usize)>>,
    max_x: usize,
    max_y: usize,
}

impl Grid {
    pub fn new(first: Vec<Vec<Vec<u8>>>) -> Self {
        let mut grid_at_time = Vec::new();
        let max_x = first[0].len();
        let max_y = first.len();
        grid_at_time.insert(0, first);
        let move_at_time = HashMap::new();
        Self {
            grid_at_time,
            move_at_time,
            max_x,
            max_y,
        }
    }
    pub fn possible_move(&mut self, pos: (usize, usize), time: usize) -> bool {
        let _gt = self.at_time(time);
        //return self.move_at_time.contains_key(&(pos,time));
        
        if let Some(_m) = self.move_at_time.get(&(pos,time)) {
            self.move_at_time.remove(&(pos,time));
            true
        } else {
            false
        }
        //gt[pos.1][pos.0].len() == 0
    }

    /// return the shape of the grid at specified time
    fn at_time(&mut self, time: usize) -> &Vec<Vec<Vec<u8>>> {
        let cycle_time = (self.max_x -2) * (self.max_y-2);
        //if time >= cycle_time {
        //    return &self.grid_at_time[(time - cycle_time)%time];
        //}
        while self.grid_at_time.len() <= time {
            //dbg!(self.grid_at_time.len());
            let grid = self.calc_next_tick(self.grid_at_time.len());
            for row in 0..grid.len() {
                for col in 0..grid[row].len() {
                    if grid[row][col].len() == 0 {
                        self.move_at_time.insert(((col,row),time), Vec::new());
                    }
                }
            }
            self.grid_at_time.insert(
                self.grid_at_time.len(),
                grid,
            );
        }
        &self.grid_at_time[time]
    }

    fn remove_grid_at_time(&mut self, time: usize) {
        self.grid_at_time.truncate(time-1);
    }

    fn get_dest_pos(&self) -> (usize, usize) {
        (self.max_x - 2, self.max_y -1)
    }

    fn print_grid(grid: &Vec<Vec<Vec<u8>>>) {
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                let v = &grid[r][c];
                if v.len() == 0 {
                    print!(".");
                } else if v.len() > 1 {
                    print!("{:1}", v.len());
                } else {
                    print!("{}", v[0] as char);
                }
            }
            println!("");
        }
    }

    fn calc_next_tick(&self, tick: usize) -> Vec<Vec<Vec<u8>>> {
        let mut next_grid: Vec<Vec<Vec<u8>>> = Vec::new();
        //let mut move_at_time = HashMap::new();

        let this_grid = &self.grid_at_time[tick - 1];

        for row in 0..self.max_y {
            next_grid.insert(row, Vec::new());
            for col in 0..self.max_x {
                next_grid[row].insert(col, Vec::new());
            }
        }

        for row in 0..this_grid.len() {
            for col in 0..this_grid[row].len() {
                for blizz in &this_grid[row][col] {
                    match blizz {
                        b'<' => {
                            if this_grid[row][col - 1].get(0).is_some_and(|v| *v == b'#') {
                                next_grid[row][self.max_x - 2].push(*blizz);
                            } else {
                                next_grid[row][col - 1].push(*blizz);
                            }
                        }
                        b'^' => {
                            if this_grid[row - 1][col].get(0).is_some_and(|v| *v == b'#') {
                                next_grid[self.max_y - 2][col].push(*blizz);
                            } else {
                                next_grid[row - 1][col].push(*blizz);
                            }
                        }
                        b'>' => {
                            if this_grid[row][col + 1].get(0).is_some_and(|v| *v == b'#') {
                                next_grid[row][1].push(*blizz);
                            } else {
                                next_grid[row][col + 1].push(*blizz);
                            }
                        }
                        b'v' => {
                            if this_grid[row + 1][col].get(0).is_some_and(|v| *v == b'#') {
                                next_grid[1][col].push(*blizz);
                            } else {
                                next_grid[row + 1][col].push(*blizz);
                            }
                        }
                        b'#' => next_grid[row][col].push(*blizz),
                        b'.' => (),
                        _ => panic!("unexpected grid val"),
                    }
                }
            }
        }
        next_grid
    }
}

fn parse_grid(input: &str) -> Vec<Vec<Vec<u8>>> {
    let mut result: Vec<Vec<Vec<u8>>> = Vec::new();
    for row in input.lines() {
        result.push(row.as_bytes().iter().map(|c| vec![*c]).collect());
    }
    result
}

fn traverse_grid(grid: &mut Grid, start: &(usize, usize), stop: &(usize, usize), at_time: usize) -> Option<usize> {
    let possible_moves: [(isize, isize); 5] = [(1, 0), (0, 1), (0, 0), (-1, 0), (0, -1)];

    let mut moves: VecDeque<((usize, usize), usize)> = VecDeque::new();
    let mut cur_tick = at_time;
    moves.push_back((*start, at_time));
    while moves.len() > 0 {
        let m = moves.pop_front().unwrap();
        //dbg!(m);
        if m.0 == *stop {
            println!("{}", m.1);
            return Some(m.1);
        }
        if m.1 != cur_tick {
        //     println!("Reached tick {} with {} options in the list", m.1, moves.len());
        //     Grid::print_grid(&grid.at_time(m.1));
             cur_tick = m.1;
        }
        for rm in possible_moves.iter() {
            let pmx = m.0.0 as isize + rm.0;
            let pmy = m.0.1 as isize + rm.1;
            //println!("trying ({},{}) to ({},{}) on tick {}", m.0.0, m.0.1, pmx, pmy, m.1);
            if pmy >= 0 && pmx >= 0 {
                let next_move = (pmx as usize, pmy as usize);
                if grid.possible_move(next_move, m.1 + 1) {
                    //println!("{:?} can reach {:?} on tick {}", m, next_move, m.1+1);
                    moves.push_back((next_move, m.1 +1));
                }
            }
        }
    }
    if moves.len() == 0 {
        println!("exhausted all moves at {cur_tick}");
    }
    return None;
}

fn main() {
    const INPUT: &str = if IS_SAMPLE {
        include_str!("../sample.txt")
    } else {
        include_str!("../input24.txt")
    };
    let mut grid = Grid::new(parse_grid(INPUT));
    //println!("initial grid");
    //Grid::print_grid(&grid_at_time.at_time(0));

    let start: (usize, usize) = (1, 0);
    let stop = grid.get_dest_pos();
    let first = traverse_grid(&mut grid, &start, &stop, 0);
    let mut total_time = 0;
    match first {
        None => panic!("couldn't solve first pass"),
        Some(t) => total_time = t,
    }
    let second = traverse_grid(&mut grid, &stop, &start, total_time);

    match second {
        None => panic!("couldn't solve second pass"),
        Some(t) => total_time = t,
    }
    grid.remove_grid_at_time(total_time-1);
    let third = traverse_grid(&mut grid, &start, &stop, total_time);

    match third {
        None => panic!("couldn't solve third pass"),
        Some(t) => total_time = t,
    }
    println!("solved with total {total_time}");
}