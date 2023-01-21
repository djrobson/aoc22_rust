const IS_SAMPLE: bool = true;
const PHASE: u8 = 1;

struct Input {
    grid: &'static str,
    dir: &'static str,
    order: &'static str,
    size: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum GridVal {
    Space,
    Rock,
    Void,
}
#[derive(Clone,Copy)]
struct GridLocation{
    val: GridVal,
    orig_row: u32,
    orig_cal: u32,
}

#[derive(Clone,Copy)]
enum GridOrientation {
    Top(usize),
    Bottom(usize),
    Right(usize),
    Left(usize),
}
use GridOrientation::*;

fn grid_order() -> Vec<Vec<GridOrientation>>
{
    // consider auto discoverying the order from the input
    let result = match (PHASE, IS_SAMPLE) {
        (1, true) =>
            vec![
                vec![Bottom(6), Right(1), Left(1), Top(2)],
                vec![Bottom(1), Right(4), Left(5), Top(6)],
                vec![Bottom(3), Right(6), Left(6), Top(3)],
                vec![Bottom(4), Right(5), Left(2), Top(4)],
                vec![Bottom(5), Right(2), Left(4), Top(5)],
                vec![Bottom(2), Right(3), Left(2), Top(1)],
            ],
        (1, false) => 
            vec![
                vec![Bottom(6), Right(3), Left(3), Top(2)],
                vec![Bottom(1), Right(2), Left(2), Top(6)],
                vec![Bottom(3), Right(1), Left(1), Top(3)],
                vec![Bottom(5), Right(6), Left(6), Top(5)],
                vec![Bottom(4), Right(5), Left(5), Top(4)],
                vec![Bottom(1), Right(4), Left(4), Top(1)],
            ],
        (2, _) => 
            vec![
                vec![Top(5), Left(4), Bottom(3), Right(2)],
                vec![Top(4), Left(6), Bottom(1), Right(3)],
                vec![Top(6), Left(5), Bottom(2), Right(1)],
                vec![Top(2), Left(1), Bottom(6), Right(5)],
                vec![Top(1), Left(3), Bottom(4), Right(6)],
                vec![Top(3), Left(2), Bottom(5), Right(4)],
            ],
        _ => panic!("unexpected grid order"),
    };
    result
}

fn parse_grid(input: &str, order: &Vec<Vec<Option<GridOrientation>>>, size: usize) -> Vec<Vec<Vec<GridLocation>>> {

    // parse the grid in it's original shape
    let mut big_grid: Vec<Vec<Vec<GridLocation>>> = vec![Vec::new();7];
    let mut row = 0;
    for line in input.split('\n') {
        let chars = line
        .as_bytes()
        .into_iter()
        .enumerate()
        .map(|(idx, byte)| {
                let val = match byte {
                    b' ' => GridVal::Void,
                    b'.' => GridVal::Space,
                    b'#' => GridVal::Rock,
                    _ => panic!("found bad grid val")
                };
                GridLocation{val, orig_row: row, orig_cal: idx as u32, }
            }
        ).collect();

        big_grid[0].push(chars);
        row += 1;
    }

    // break the grid into panels
    for row in 0..order.len() {
        for g in 0..order[row].len() {
            let row_start = row * size as usize;
            let col_start = g*size as usize;
            let mut face_row = 0;
            match order[row][g] {
                Some(Top(side)) => {
                    for input_row in row_start..(row_start+size as usize) {
                        big_grid[side].insert(face_row, Vec::new());
                        for input_col in col_start..(col_start+ size as usize) {
                            let grid_val = big_grid[0][input_row][input_col].clone();
                            big_grid[side][face_row].push(grid_val);
                        }
                        face_row +=1;
                    }
                },
                Some(Left(side)) => todo!(),
                Some(Right(side)) => todo!(),
                Some(Bottom(side)) => todo!(),
                None => (),
            };
        }
    }
    big_grid
}

fn parse_order(input: &str) -> Vec<Vec<Option<GridOrientation>>> {
    let mut output: Vec<Vec<Option<GridOrientation>>> = Vec::new();
    for line in input.lines() {
        let orientations = line
            .split_whitespace()
            .map(|pair| (pair.as_bytes()[0], pair.as_bytes()[1]))
            .map(|(side, orientation)| {
                match (side,orientation,PHASE) {
                    (b'B',_,_) => None,
                    (side, _, 1) => Some(GridOrientation::Top((side - b'0') as usize)),
                    (side, b'T', 2) => Some(GridOrientation::Top((side - b'0') as usize)),
                    (side, b'R', 2) => Some(GridOrientation::Right((side - b'0') as usize)),
                    (side, b'L', 2) => Some(GridOrientation::Left((side - b'0') as usize)),
                    (side, b'B', 2) => Some(GridOrientation::Bottom((side - b'0') as usize)),
                    _ => panic!("unexpected order"),
                }
            }).collect();
        output.push(orientations);
    }
    output
}

enum Direction {
    Forward(usize),
    TurnLeft,
    TurnRight,
}

use Direction::*;

fn parse_directions(input: &str) -> Vec<Direction> {
    let mut directions= Vec::new();
    let dir_bytes = input.as_bytes();
    let length = dir_bytes.len();
    let mut cursor = 0;
    while cursor < length {
        let char = dir_bytes[cursor];
        let dir = match char {
            b'0'..=b'9' => {
                let mut count = char as usize - b'0' as usize;
                loop {
                    if cursor == length -1 {
                        break;
                    }
                    let next = dir_bytes[cursor+1];
                    if next >= b'0' && next <= b'9' {
                        count = (count * 10) + (next as usize - b'0' as usize);
                        cursor = cursor +1;
                    } else {
                        break;
                    }
                }
                Forward(count)
            }
            b'L' => TurnLeft,
            b'R' => TurnRight,
            _ => panic!("unexpected direction")
        };
        directions.push(dir);
        cursor = cursor +1;
    }
    directions
}

#[derive(Copy,Clone)]
enum Facing {
    Top,
    Left,
    Right,
    Bottom,
}
#[derive(Copy, Clone)]
struct Location {
    x: usize,
    y: usize,
    grid: u8,
    facing: Facing,
}

impl Location {
    fn score(&self) -> u32 {
        match self.facing {
            Facing::Right => 0,
            Facing::Bottom => 1,
            Facing::Left => 2,
            Facing::Top => 3,
        }
    }
    fn turn(&mut self, dir: &Direction) {
        match (self.facing, dir) {
            (Facing::Top, TurnLeft) => self.facing = Facing::Left,
            (Facing::Top, TurnRight) => self.facing = Facing::Right,
            (Facing::Left, TurnLeft) => self.facing = Facing::Bottom,
            (Facing::Left, TurnRight) => self.facing = Facing::Top,
            (Facing::Right, TurnLeft) => self.facing = Facing::Top,
            (Facing::Right, TurnRight) => self.facing = Facing::Bottom,
            (Facing::Bottom, TurnLeft) => self.facing = Facing::Right,
            (Facing::Bottom, TurnRight) => self.facing = Facing::Left,
            _ => panic!("unexpected turn"),
        }
    }
    fn forward_around_edge(&mut self, face: Facing) {
        match (self.grid, self.facing) {
            (0,_) => {},
            (1,Facing::Top) => todo!(),
            (1,Facing::Left) => todo!(),
            (1,Facing::Right) => todo!(),
            (1,Facing::Bottom) => todo!(),
            (2,Facing::Top) => todo!(),
            (2,Facing::Left) => todo!(),
            (2,Facing::Right) => todo!(),
            (2,Facing::Bottom) => todo!(),
            (3,Facing::Top) => todo!(),
            (3,Facing::Left) => todo!(),
            (3,Facing::Right) => todo!(),
            (3,Facing::Bottom) => todo!(),
            (4,Facing::Top) => todo!(),
            (4,Facing::Left) => todo!(),
            (4,Facing::Right) => todo!(),
            (4,Facing::Bottom) => todo!(),
            (5,Facing::Top) => todo!(),
            (5,Facing::Left) => todo!(),
            (5,Facing::Right) => todo!(),
            (5,Facing::Bottom) => todo!(),
            (6,Facing::Top) => todo!(),
            (6,Facing::Left) => todo!(),
            (6,Facing::Right) => todo!(),
            (6,Facing::Bottom) => todo!(),
            _ => panic!("Forward across unexpected edge"),
        }

    }
}

fn try_move( cur_location: &Location, delta: &(i32,i32), face_size: usize, orientation: &Vec<Vec<GridOrientation>>) -> Location {
    
    // todo 
    let mut next_direction: GridOrientation;
    let mut next_side: u8;
    let mut next_x: usize;
    let mut next_y: usize;
    let mut next_dir: Facing;

    // off the left
    if delta.0 == -1 && cur_location.x == 0 {
        next_direction = orientation[cur_location.grid as usize][1];

        todo!();
    }
    // off the right
    else if delta.0 == 1 && cur_location.x == face_size{
        next_direction = orientation[cur_location.grid as usize][2];
        todo!();
    }
    // off the top
    else if delta.1 == -1 && cur_location.y == 0 {
        next_direction = orientation[cur_location.grid as usize][0];
        todo!();
    }
    // off the bottom
    else if delta.1 == 1 && cur_location.y == face_size {
        next_direction = orientation[cur_location.grid as usize][3];
        todo!();
    } else { // internal to a face
        next_x = (cur_location.x as i32 + delta.0) as usize;
        next_y = (cur_location.y as i32 + delta.1) as usize;
    }
    let new_location = Location{x: next_x, y: next_y, grid: next_side, facing: next_dir};
    new_location
}

fn perform_walk(orientation: &Vec<Vec<GridOrientation>>, grid: &Vec<Vec<Vec<GridLocation>>>, directions: &Vec<Direction>, location: &Location, face_size :usize) -> u32 {
    let mut my_location = location.clone();

    for d in directions {
        match d {
            Forward(count) => {
                let delta:(i32,i32) = match my_location.facing {
                    Facing::Top => (0,-1),
                    Facing::Bottom => (0,1),
                    Facing::Left => (-1,0),
                    Facing::Right => (1,0),
                };

                for _ in 0..*count {
                    let next_location:Location = try_move(&my_location, &delta, face_size, orientation);

                    if (grid[next_location.grid as usize][next_location.x][next_location.y].val == GridVal::Space) {
                        my_location = next_location;
                    } else {
                        // collision
                        break;
                    }
                }

            },
            _ => my_location.turn(d),
        }
    }

    let final_grid_location = grid[my_location.grid as usize][my_location.y][my_location.x].clone();
    let final_score = ((final_grid_location.orig_row+1) * 1000) 
                                + ((final_grid_location.orig_cal+1) * 8) 
                                + my_location.score();
    final_score
}
fn main() {
    let input : Input = if IS_SAMPLE {
        Input{  grid: include_str!("../sample-grid.txt"),
                dir: include_str!("../sample-directions.txt"),
                order:include_str!("../sample-order.txt"),
                size: 4}
    } else {
        Input{  grid: include_str!("../day22-grid.txt"),
                dir: include_str!("../day22-directions.txt"),
                order: include_str!("../day22-order.txt"),
                size: 50}
    };

    let input_sections = parse_order(input.order);
    let grid_orientation = grid_order();
    let grid = parse_grid(input.grid, &input_sections, input.size);
    let directions = parse_directions(input.dir);

    let start_location = if IS_SAMPLE {
        Location{x:8,y:0,grid:0,facing:Facing::Right}
    } else {
        Location{x:50,y:0,grid:1,facing:Facing::Right}
    };

    let end_location_score = perform_walk(&grid_orientation, &grid, &directions, &start_location, input.size);
    println!("score: {end_location_score}");

}
