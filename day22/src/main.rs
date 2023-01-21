const IS_SAMPLE: bool = true;
const PHASE: u8 = 1;

struct Input {
    Grid: &'static str,
    Dir: &'static str,
    Order: &'static str,
    Size: u32
}

#[derive(Clone,Copy)]
enum Phase {
    One,
    Two,
}

#[derive(Clone)]
enum GridVal {
    Space,
    Rock,
    Void,
}
#[derive(Clone)]
struct GridLocation{
    val: GridVal,
    orig_row: u32,
    orig_cal: u32,
}
enum GridOrientation {
    Top(usize),
    Bottom(usize),
    Right(usize),
    Left(usize),
}
use GridOrientation::*;

fn GridOrder(_order: &str, phase: Phase) -> Vec<Vec<GridOrientation>>
{
    // consider auto discoverying the order from the input
    let result = match (phase, IS_SAMPLE) {
        (Phase::One, True) =>
            vec![
                vec![Bottom(6), Right(1), Left(1), Top(2)],
                vec![Bottom(1), Right(4), Left(5), Top(6)],
                vec![Bottom(3), Right(6), Left(6), Top(3)],
                vec![Bottom(4), Right(5), Left(2), Top(4)],
                vec![Bottom(5), Right(2), Left(4), Top(5)],
                vec![Bottom(2), Right(3), Left(2), Top(1)],
            ],
        (Phase::One, False) => 
            vec![
                vec![Bottom(6), Right(3), Left(3), Top(2)],
                vec![Bottom(1), Right(2), Left(2), Top(6)],
                vec![Bottom(3), Right(1), Left(1), Top(3)],
                vec![Bottom(5), Right(6), Left(6), Top(5)],
                vec![Bottom(4), Right(5), Left(5), Top(4)],
                vec![Bottom(1), Right(4), Left(4), Top(1)],
            ],
        (Phase::Two, _) => 
            vec![
                vec![Top(5), Left(4), Bottom(3), Right(2)],
                vec![Top(4), Left(6), Bottom(1), Right(3)],
                vec![Top(6), Left(5), Bottom(2), Right(1)],
                vec![Top(2), Left(1), Bottom(6), Right(5)],
                vec![Top(1), Left(3), Bottom(4), Right(6)],
                vec![Top(3), Left(2), Bottom(5), Right(4)],
            ],
    };
    result
}

fn parse_grid(input: &str, order: &Vec<Vec<Option<GridOrientation>>>, size: u32) -> Vec<Vec<Vec<GridLocation>>> {

    // parse the grid in it's original shape
    let mut big_grid: Vec<Vec<Vec<GridLocation>>> = Vec::new();
    big_grid.insert(0, Vec::new());
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
            match order[row][g] {
                Some(Top(side)) => todo!(),
                Some(Left(side)) => todo!(),
                Some(Right(side)) => todo!(),
                Some(Bottom(side)) => todo!(),
                None => (),
            };
        }
    }
    big_grid
}

fn parse_order(input: &str, phase: Phase) -> Vec<Vec<Option<GridOrientation>>> {
    let mut output: Vec<Vec<Option<GridOrientation>>> = Vec::new();
    for line in input.lines() {
        let orientations = line
            .split_whitespace()
            .map(|pair| (pair.as_bytes()[0], pair.as_bytes()[1]))
            .map(|(side, orientation)| {
                match (side,orientation,phase) {
                    (b'B',_,_) => None,
                    (side, _, Phase::One) => Some(GridOrientation::Top(side as usize)),
                    (side, b'T', Phase::Two) => Some(GridOrientation::Top(side as usize)),
                    (side, b'R', Phase::Two) => Some(GridOrientation::Right(side as usize)),
                    (side, b'L', Phase::Two) => Some(GridOrientation::Left(side as usize)),
                    (side, b'B', Phase::Two) => Some(GridOrientation::Bottom(side as usize)),
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
    fn turn(&mut self, dir: Direction) {
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

fn perform_walk(order: Vec<Vec<Option<GridOrientation>>>, grid: Vec<Vec<Vec<GridLocation>>>, directions: Vec<Direction>, location: Location, face_size :u32) -> u32 {
    let mut my_location = location;


    for d in directions {
        match d {
            Forward(_) => {
                let delta:(i32,i32) = match my_location.facing {
                    Facing::Top => (0,-1),
                    Facing::Bottom => (0,1),
                    Facing::Left => (-1,0),
                    Facing::Right => (1,0),
                };

                // off the left
                if delta.0 == -1 && my_location.x == 0 {
                    todo!();
                }
                // off the right
                else if delta.0 == 1 && my_location.x == face_size as usize {
                    todo!();
                }
                // off the top
                else if delta.1 == -1 && my_location.y == 0 {
                    todo!();
                }
                // off the bottom
                else if delta.1 == 1 && my_location.y == face_size as usize {
                    todo!();
                } else { // internal to a face
                    my_location.x = (my_location.x as i32 + delta.0) as usize;
                    my_location.y = (my_location.y as i32 + delta.1) as usize;
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
        Input{ Grid: include_str!("../sample-grid.txt"),
        Dir: include_str!("../sample-directions.txt"),
        Order:include_str!("../sample-order.txt"),
        Size: 4}
    } else {
        Input{ Grid: include_str!("../day22-grid.txt"),
        Dir: include_str!("../day22-directions.txt"),
        Order: include_str!("../day22-order.txt"),
        Size: 50}
    };

    let order = parse_order(input.Order, Phase::One);
    let grid = parse_grid(input.Grid, &order, input.Size);
    let directions = parse_directions(input.Dir);

    let start_location = if IS_SAMPLE {
        Location{x:8,y:0,grid:0,facing:Facing::Right}
    } else {
        Location{x:50,y:0,grid:1,facing:Facing::Right}
    };

    let end_location_score = perform_walk(order, grid, directions, start_location, input.Size);
    println!("score: {end_location_score}");

}
