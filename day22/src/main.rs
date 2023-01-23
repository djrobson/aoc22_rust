use std::fmt;

const IS_SAMPLE: bool = true;
const PHASE: u8 = 2;

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
#[derive(Clone, Copy)]
struct GridLocation {
    val: GridVal,
    orig_row: u32,
    orig_col: u32,
}

#[derive(Copy, Clone)]
enum Facing {
    Top,
    Left,
    Right,
    Bottom,
}
impl fmt::Debug for Facing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Top => write!(f, "^"),
            Left => write!(f, "<"),
            Right => write!(f, ">"),
            Bottom => write!(f, "v"),
        }
    }
}
use Facing::*;

fn grid_order() -> Vec<Vec<(Facing, usize)>> {
    // consider auto discoverying the order from the input
    let result = match (PHASE, IS_SAMPLE) {
        (1, true) => vec![
            vec![(Bottom,0);4],
            vec![(Bottom, 6), (Right, 1), (Left, 1), (Top, 2)],
            vec![(Bottom, 1), (Right, 4), (Left, 5), (Top, 6)],
            vec![(Bottom, 3), (Right, 6), (Left, 6), (Top, 3)],
            vec![(Bottom, 4), (Right, 5), (Left, 2), (Top, 4)],
            vec![(Bottom, 5), (Right, 2), (Left, 4), (Top, 5)],
            vec![(Bottom, 2), (Right, 3), (Left, 2), (Top, 1)],
        ],
        (1, false) => vec![
            vec![(Bottom,0);4],
            vec![(Bottom, 6), (Right, 3), (Left, 3), (Top, 2)],
            vec![(Bottom, 1), (Right, 2), (Left, 2), (Top, 6)],
            vec![(Bottom, 3), (Right, 1), (Left, 1), (Top, 3)],
            vec![(Bottom, 5), (Right, 6), (Left, 6), (Top, 5)],
            vec![(Bottom, 4), (Right, 5), (Left, 5), (Top, 4)],
            vec![(Bottom, 1), (Right, 4), (Left, 4), (Top, 1)],
        ],
        (2, _) => vec![
            vec![(Bottom,0);4],
            vec![(Top, 5), (Left, 4), (Bottom, 3), (Right, 2)],
            vec![(Top, 4), (Left, 6), (Bottom, 1), (Right, 3)],
            vec![(Top, 6), (Left, 5), (Bottom, 2), (Right, 1)],
            vec![(Top, 2), (Left, 1), (Bottom, 6), (Right, 5)],
            vec![(Top, 1), (Left, 3), (Bottom, 4), (Right, 6)],
            vec![(Top, 3), (Left, 2), (Bottom, 5), (Right, 4)],
        ],
        _ => panic!("unexpected grid order"),
    };
    result
}

fn parse_grid(
    input: &str,
    order: &Vec<Vec<Option<(Facing, usize)>>>,
    size: usize,
) -> Vec<Vec<Vec<GridLocation>>> {
    // parse the grid in it's original shape
    let mut big_grid: Vec<Vec<Vec<GridLocation>>> = vec![Vec::new(); 7];
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
                    _ => panic!("found bad grid val"),
                };
                GridLocation {
                    val,
                    orig_row: row,
                    orig_col: idx as u32,
                }
            })
            .collect();

        big_grid[0].push(chars);
        row += 1;
    }

    // break the grid into panels
    for row in 0..order.len() {
        for g in 0..order[row].len() {
            let row_start = row * size as usize;
            let col_start = g * size as usize;
            let mut face_row = 0;
            match order[row][g] {
                Some((Top, side)) => {
                    // 00 10 20 30
                    // 01 11 21 31
                    // 02 12 22 32
                    // 03 13 23 33
                    for input_row in row_start..(row_start + size as usize) {
                        big_grid[side].insert(face_row, Vec::new());
                        for input_col in col_start..(col_start + size as usize) {
                            let grid_val = big_grid[0][input_row][input_col].clone();
                            big_grid[side][face_row].push(grid_val);
                        }
                        face_row += 1;
                    }
                }
                Some((Left, side)) => {
                    // 03 02 01 00
                    // 13 12 11 10
                    // 23 22 21 20
                    // 33 32 31 30
                    for input_col in (col_start..col_start + size as usize).rev() {
                        big_grid[side].insert(face_row, Vec::new());
                        for input_row in row_start..(row_start + size as usize) {
                            let grid_val = big_grid[0][input_row][input_col].clone();
                            big_grid[side][face_row].push(grid_val);
                        }
                        face_row +=1;
                    }
                }
                Some((Right, side)) => {
                    // 30 31 32 33
                    // 20 21 22 23
                    // 10 11 12 13
                    // 00 01 02 03 
                    for input_row in (row_start..row_start + size as usize).rev() {
                        big_grid[side].insert(face_row, Vec::new());
                        for input_col in col_start..(col_start + size as usize) {
                            let grid_val = big_grid[0][input_row][input_col].clone();
                            big_grid[side][face_row].push(grid_val);
                        }
                        face_row +=1;
                    }
                },
                Some((Bottom, side)) => {
                    // 33 23 13 03
                    // 32 22 12 02
                    // 31 21 11 01
                    // 30 30 10 00
                    for input_row in (row_start..(row_start + size as usize)).rev() {
                        big_grid[side].insert(face_row, Vec::new());
                        for input_col in (col_start..(col_start + size as usize)).rev() {
                            let grid_val = big_grid[0][input_row][input_col].clone();
                            big_grid[side][face_row].push(grid_val);
                        }
                        face_row += 1;
                    }
                },
                None => (),
            };
        }
    }
    big_grid
}

fn parse_order(input: &str) -> Vec<Vec<Option<(Facing, usize)>>> {
    let mut output: Vec<Vec<Option<(Facing, usize)>>> = Vec::new();
    for line in input.lines() {
        let orientations = line
            .split_whitespace()
            .map(|pair| (pair.as_bytes()[0], pair.as_bytes()[1]))
            .map(|(side, orientation)| match (side, orientation, PHASE) {
                (b'B', _, _) => None,
                (side, _, 1) => Some((Top, (side - b'0') as usize)),
                (side, b'T', 2) => Some((Top, (side - b'0') as usize)),
                (side, b'R', 2) => Some((Right, (side - b'0') as usize)),
                (side, b'L', 2) => Some((Left, (side - b'0') as usize)),
                (side, b'B', 2) => Some((Bottom, (side - b'0') as usize)),
                _ => panic!("unexpected order"),
            })
            .collect();
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
    let mut directions = Vec::new();
    let dir_bytes = input.as_bytes();
    let length = dir_bytes.len();
    let mut cursor = 0;
    while cursor < length {
        let char = dir_bytes[cursor];
        let dir = match char {
            b'0'..=b'9' => {
                let mut count = char as usize - b'0' as usize;
                loop {
                    if cursor == length - 1 {
                        break;
                    }
                    let next = dir_bytes[cursor + 1];
                    if next >= b'0' && next <= b'9' {
                        count = (count * 10) + (next as usize - b'0' as usize);
                        cursor = cursor + 1;
                    } else {
                        break;
                    }
                }
                Forward(count)
            }
            b'L' => TurnLeft,
            b'R' => TurnRight,
            _ => panic!("unexpected direction"),
        };
        directions.push(dir);
        cursor = cursor + 1;
    }
    directions
}

#[derive(Copy, Clone)]
struct Location {
    x: usize,
    y: usize,
    grid: usize,
    facing: Facing,
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"({}, {}): {} {:?}", self.x, self.y, self.grid, self.facing)
    }
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
}

fn rotate_transform(
    leaving_side: Facing,
    arriving_side: (Facing, usize),
    old_x: usize,
    old_y: usize,
    face_size: usize,
) -> Location {
    match (leaving_side,arriving_side.0) {
        (Bottom, Top) => Location{x:old_x, y: 0, grid: arriving_side.1, facing: Bottom},
        (Bottom, Left) => Location{x:0, y:(face_size -1 - old_x), grid: arriving_side.1, facing: Right},
        (Bottom, Right) => Location{x:face_size -1, y:old_x, grid: arriving_side.1, facing: Left},
        (Bottom, Bottom) => Location{x:(face_size -1 - old_x), y: face_size-1, grid: arriving_side.1, facing: Top},
        (Top, Top) => Location{x:(face_size -1 - old_x), y:0, grid: arriving_side.1, facing: Bottom},
        (Top, Left) => Location{x:0, y:old_x, grid: arriving_side.1, facing: Right},
        (Top, Right) => Location{x:face_size -1, y:(face_size -1 - old_x), grid: arriving_side.1, facing: Left},
        (Top, Bottom) => Location{x:old_x, y:face_size-1, grid: arriving_side.1, facing: Top},
        (Left, Top) => Location{x:old_y, y:0, grid: arriving_side.1, facing: Bottom},
        (Left, Left) => Location{x:0, y:(face_size -1 - old_y), grid: arriving_side.1, facing: Right},
        (Left, Right) => Location{x:face_size-1, y:old_y, grid: arriving_side.1, facing: Left},
        (Left, Bottom) => Location{x:old_y, y:face_size -1, grid: arriving_side.1, facing: Top},
        (Right, Top) => Location{x:(face_size -1 - old_y), y:0, grid: arriving_side.1, facing: Bottom},
        (Right, Left) => Location{x:0, y:old_y, grid: arriving_side.1, facing: Right},
        (Right, Right) => Location{x:face_size -1, y:(face_size -1 - old_y), grid: arriving_side.1, facing: Left},
        (Right, Bottom) => Location{x:old_y, y:face_size -1, grid: arriving_side.1, facing: Top},
    }
}

fn try_move(
    cur_location: &Location,
    delta: &(i32, i32),
    face_size: usize,
    orientation: &Vec<Vec<(Facing, usize)>>,
    grid: &Vec<Vec<Vec<GridLocation>>>,
) -> Location {
    let orig_loc = grid[cur_location.grid][cur_location.y][cur_location.x];
    let new_location: Location = if delta.0 == -1 && cur_location.x == 0 {
        // off the left
        println!("Leaving Left side of {} to {:?} of {} ({},{})", cur_location.grid,
            orientation[cur_location.grid as usize][1].0,
            orientation[cur_location.grid as usize][1].1,
            orig_loc.orig_col, 
            orig_loc.orig_row,
        );
        rotate_transform(
            Left,
            orientation[cur_location.grid as usize][1],
            cur_location.x,
            cur_location.y,
            face_size,
        )
    } else if delta.0 == 1 && cur_location.x == face_size-1 {
        // off the right
        println!("Leaving Right side of {} to {:?} of {} ({},{})", cur_location.grid,
            orientation[cur_location.grid as usize][2].0,
            orientation[cur_location.grid as usize][2].1,
            orig_loc.orig_col, 
            orig_loc.orig_row,);
        rotate_transform(
            Right,
            orientation[cur_location.grid as usize][2],
            cur_location.x,
            cur_location.y,
            face_size,
        )
    } else if delta.1 == -1 && cur_location.y == 0 {
        // off the top
        println!("Leaving Top side of {} to {:?} of {} ({},{})", cur_location.grid,
            orientation[cur_location.grid as usize][0].0,
            orientation[cur_location.grid as usize][0].1,
            orig_loc.orig_col, 
            orig_loc.orig_row,);
        rotate_transform(
            Top,
            orientation[cur_location.grid as usize][0],
            cur_location.x,
            cur_location.y,
            face_size,
        )
    } else if delta.1 == 1 && cur_location.y == face_size-1 {
        // off the bottom
        println!("Leaving Bottom side of {} to {:?} of {} ({},{})", cur_location.grid,
            orientation[cur_location.grid as usize][3].0,
            orientation[cur_location.grid as usize][3].1,
            orig_loc.orig_col, 
            orig_loc.orig_row,);
        rotate_transform(
            Bottom,
            orientation[cur_location.grid as usize][3],
            cur_location.x,
            cur_location.y,
            face_size,
        )
    } else {
        // internal to a face
        Location {
            x: (cur_location.x as i32 + delta.0) as usize,
            y: (cur_location.y as i32 + delta.1) as usize,
            grid: cur_location.grid,
            facing: cur_location.facing,
        }
    };
    new_location
}

fn perform_walk(
    orientation: &Vec<Vec<(Facing, usize)>>,
    grid: &Vec<Vec<Vec<GridLocation>>>,
    directions: &Vec<Direction>,
    location: &Location,
    face_size: usize,
) -> u32 {
    let mut my_location = location.clone();

    for d in directions {
        match d {
            Forward(count) => {

                for _walk_cnt in 0..*count {
                    let delta: (i32, i32) = match my_location.facing {
                        Facing::Top => (0, -1),
                        Facing::Bottom => (0, 1),
                        Facing::Left => (-1, 0),
                        Facing::Right => (1, 0),
                    };
                    
                    let next_location: Location =
                        try_move(&my_location, &delta, face_size, orientation, grid);

                    if grid[next_location.grid as usize][next_location.y][next_location.x].val
                        == GridVal::Space
                    {
                        println!("{:?} ({},{})", my_location, 
                            grid[my_location.grid][my_location.y][my_location.x].orig_col,
                            grid[my_location.grid][my_location.y][my_location.x].orig_row,);
                        my_location = next_location;
                    } else {
                        println!("{:?} bump", my_location);
                        // collision
                        break;
                    }
                }
            }
            _ => my_location.turn(d),
        }
    }

    let final_grid_location = grid[my_location.grid as usize][my_location.y][my_location.x].clone();
    let final_score = ((final_grid_location.orig_row + 1) * 1000)
        + ((final_grid_location.orig_col + 1) * 4)
        + my_location.score();
    final_score
}
fn main() {
    let input: Input = if IS_SAMPLE {
        Input {
            grid: include_str!("../sample-grid.txt"),
            dir: include_str!("../sample-directions.txt"),
            order: include_str!("../sample-order.txt"),
            size: 4,
        }
    } else {
        Input {
            grid: include_str!("../day22-grid.txt"),
            dir: include_str!("../day22-directions.txt"),
            order: include_str!("../day22-order.txt"),
            size: 50,
        }
    };

    let input_sections = parse_order(input.order);
    let grid_orientation = grid_order();
    let grid = parse_grid(input.grid, &input_sections, input.size);
    let directions = parse_directions(input.dir);

    let start_location =
        Location {
            x: 0,
            y: 0,
            grid: 1,
            facing: Facing::Right,
        };

    let end_location_score = perform_walk(
        &grid_orientation,
        &grid,
        &directions,
        &start_location,
        input.size,
    );
    println!("score: {end_location_score}");
}
