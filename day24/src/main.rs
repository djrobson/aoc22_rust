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
        if let Some(_m) = self.move_at_time.get(&(pos,time)) {
            self.move_at_time.remove(&(pos,time));
            true
        } else {
            false
        }

        //gt[pos.1][pos.0].len() == 0
    }

    fn at_time(&mut self, time: usize) -> &Vec<Vec<Vec<u8>>> {
        let cycle_time = (self.max_x -2) * (self.max_y-2);
        if time >= cycle_time {
            return &self.grid_at_time[(time - cycle_time)%time];
        }
        while self.grid_at_time.len() <= time {
            dbg!(self.grid_at_time.len());
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
        //Grid::print_grid(&next_grid);
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

fn main() {
    const INPUT: &str = if IS_SAMPLE {
        include_str!("../sample.txt")
    } else {
        include_str!("../input24.txt")
    };
    let mut grid_at_time = Grid::new(parse_grid(INPUT));
    //println!("initial grid");
    //rid::print_grid(&grid_at_time.at_time(0));

    let my_pos: (usize, usize) = (1, 0);
    let end_pos = grid_at_time.get_dest_pos();
    let possible_moves: [(isize, isize); 5] = [(1, 0), (0, 1), (0, 0), (-1, 0), (0, -1)];

    let mut moves: VecDeque<((usize, usize), usize)> = VecDeque::new();

    moves.push_back((my_pos, 0));
    while moves.len() > 0 {
        let m = moves.pop_front().unwrap();
        //dbg!(m);
        if m.0 == end_pos {
            println!("{}", m.1);
            break;
        }
        for rm in possible_moves.iter() {
            let pmx = m.0.0 as isize + rm.0;
            let pmy = m.0.1 as isize + rm.1;
            //println!("trying ({},{}) to ({},{}) on tick {}", m.0.0, m.0.1, pmx, pmy, m.1);
            if pmy >= 0 && pmx >= 0 {
                let next_move = (pmx as usize, pmy as usize);
                if grid_at_time.possible_move(next_move, m.1 + 1) {
                    moves.push_back((next_move, m.1 +1));
                }
            }
        }
    }
    if moves.len() == 0 {
        println!("exhausted all moves");
    }
}
