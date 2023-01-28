#![feature(is_some_and)]
use std::collections::VecDeque;

const IS_SAMPLE: bool = true;

struct Grid {
    grid_at_time: Vec<Vec<Vec<Vec<u8>>>>,
    max_x: usize,
    max_y: usize,
}

impl Grid {
    pub fn new(first: Vec<Vec<Vec<u8>>>) -> Self {
        let mut grid_at_time = Vec::new();
        let max_x = first[0].len();
        let max_y = first.len();
        grid_at_time.insert(0, first);
        Self {grid_at_time, max_x, max_y }
    }
    pub fn possible_move(&mut self, pos: (usize, usize), time: usize) -> bool {
        let gt = self.at_time(time);
        gt[pos.1][pos.0].len() == 0
    }

    fn at_time(&mut self, time: usize) -> &Vec<Vec<Vec<u8>>> {
        while self.grid_at_time.len() < time {
            dbg!(self.grid_at_time.len());
            self.grid_at_time.push(self.calc_next_tick(self.grid_at_time.len()+1));
        }
        &self.grid_at_time[time]
    }

    fn get_dest_pos(&self) -> (usize,usize) {
        (self.max_x -1, self.max_y)
    }

    fn calc_next_tick(&self, tick: usize) -> Vec<Vec<Vec<u8>>> {
        let mut next_grid: Vec<Vec<Vec<u8>>> = Vec::new();
        
        let this_grid = &self.grid_at_time[tick-1];

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
                            if this_grid[row][col-1].get(0).is_some_and(|v| *v == b'#') {
                                next_grid[row][self.max_x -1].push(*blizz);
                            } else {
                                next_grid[row][col-1].push(*blizz);
                            }
                        },
                        b'^' => {
                            if this_grid[row-1][col].get(0).is_some_and(|v| *v == b'#') {
                                next_grid[self.max_y-1][col].push(*blizz);
                            } else {
                                next_grid[row-1][col].push(*blizz);
                            }
                        },
                        b'>' => {
                            if this_grid[row][col+1].get(0).is_some_and(|v| *v == b'#') {
                                next_grid[row][1].push(*blizz);
                            } else {
                                next_grid[row][col+1].push(*blizz);
                            }
                        },
                        b'v' => {
                            if this_grid[row+1][col].get(0).is_some_and(|v| *v == b'#') {
                                next_grid[1][col].push(*blizz);
                            } else {
                                next_grid[row+1][col].push(*blizz);
                            }
                        },
                        b'#' => {next_grid[row][col].push(*blizz)},
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

fn main() {
    const INPUT: &str = if IS_SAMPLE {
        include_str!("../sample.txt")
    } else {
        include_str!("../input24.txt")
    };
    let mut grid_at_time =  Grid::new(parse_grid(INPUT));

    let my_pos: (usize, usize) = (0,1);
    let end_pos = grid_at_time.get_dest_pos();
    let possible_moves:[(isize,isize);5] = [(1,0),(0,1),(0,0),(-1,0),(0,-1)];

    let mut moves: VecDeque<((usize, usize), usize)> = VecDeque::new();

    moves.push_back((my_pos, 0));
    while moves.len() > 0  {
        let m = moves.pop_front().unwrap();
        if m.0 == end_pos {
            println!("{}", m.1);
            break;
        }
        for rm in possible_moves.iter().filter(|pm| {
            let pmx = m.0.0 as isize + pm.0;
            let pmy = m.0.1 as isize + pm.1;
            if pmy >= 0 && pmx >=0 {
                grid_at_time.possible_move((pmx as usize,pmy as usize), m.1)
            } else {
                false
            }
        }){
            moves.push_back(((rm.0 as usize, rm.1 as usize),m.1));
        }
    }
    if moves.len() == 0 {
        println!("exhausted all moves");
    }
}
