use std::fmt;
use std::collections::HashMap;

const IS_SAMPLE: bool = false;
const WIDTH: usize = 7;

const TOTAL_ROCKS: usize = 1_000_000_000_000;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Down,
}

#[derive(PartialEq,Eq,Clone,Copy)]
enum Space {
    Empty,
    Ground,
    Rock,
}

impl fmt::Display for Space {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rock => write!(f,"@"),
            Ground => write!(f,"#"),
            Empty => write!(f,"."),
        }
    }
}

use Space::*;

fn print_chamber_range( chamber: & Vec<Vec<Space>>, top: usize, bottom: usize) {
    for r in (bottom..=top).rev() {
        print!("|");
        for c in &chamber[r] {
            print!("{}", c);
        }
        print!("| {}\n", r);
    }
    if bottom == 0 {
        println!("+-------+");
    }
}

fn print_chamber( chamber: & Vec<Vec<Space>>) {
    print_chamber_range(chamber, chamber.len()-1, 0);
}

fn set_shape_at_coord( chamber: &mut Vec<Vec<Space>>, shape: &Vec<Vec<Space>>, coords: (i32,i32), new_val: Space) -> () {
    let shape_height = shape.len();
    let shape_width = shape[0].len();
    for sy in 0..shape_height { // for each verticle row in the shape
        for sx in 0..shape_width {
            if shape[sy][sx] == Rock { // check the shape bitmap
                let x = (coords.0) as usize + sx;
                let y = coords.1 as usize - sy;
                //println!("chamber at x={x} y={y} was {} now {}", chamber[y][x], new_val );
                chamber[y][x] = new_val;
            }
        }
    }
}

fn attempt_shift( chamber: &mut Vec<Vec<Space>>, direction: Direction, shape: &Vec<Vec<Space>>, coords: &mut (i32,i32)) -> bool {
    let shape_height = shape.len();
    let shape_width = shape[0].len();
    match direction {
        Direction::Left => {
            // if we're on the left wall then fail
            if coords.0 == 0 {
                return false;
            }
            // check for collision
            for sy in 0..shape_height { // for each verticle row in the shape
                for sx in 0..shape_width {
                    if shape[sy][sx] == Rock {
                        if chamber[coords.1 as usize -sy][(coords.0-1) as usize + sx] == Ground {
                            return false;
                        }
                    }
                }
            }
            set_shape_at_coord(chamber, shape, *coords, Empty);
            *coords = (coords.0 -1, coords.1);
            set_shape_at_coord(chamber, shape, *coords, Rock);
        },
        Direction::Right => {
            // if we're on the right wall then fail
            if coords.0 as usize + shape_width >= WIDTH {
                return false;
            }
            // check for collision
            for sy in 0..shape_height { // for each verticle row in the shape
                for sx in 0..shape_width {
                    if shape[sy][sx] == Rock {
                        if chamber[coords.1 as usize - sy][(coords.0+1) as usize + sx] == Ground {
                            return false;
                        }
                    }
                }
            }
            set_shape_at_coord(chamber, shape, *coords, Empty);
            *coords = (coords.0 +1, coords.1);
            set_shape_at_coord(chamber, shape, *coords, Rock);
        },
        Direction::Down => {
            // if we're on the bottom then fail
            if coords.1 - shape_height as i32 + 1 <= 0 {
                set_shape_at_coord(chamber, shape, *coords, Ground);
                return false;
            }
            // check for collision
            let mut collided = false;
            'down_collide: for sy in 0..shape_height { // for each verticle row in the shape
                for sx in 0..shape_width {
                    if shape[sy][sx] == Rock {
                        if chamber[(coords.1 -1) as usize - sy][(coords.0) as usize + sx] == Ground {
                            collided = true;
                            break 'down_collide;
                        }
                    }
                }
            }
            if collided {
                set_shape_at_coord(chamber, shape, *coords, Ground);
            } else {
                set_shape_at_coord(chamber, shape, *coords, Empty);
                *coords = (coords.0, coords.1 -1);
                set_shape_at_coord(chamber, shape, *coords, Rock);
            }
            return !collided;
        },
    }
    true
}

fn find_tallest_rock(chamber: &Vec<Vec<Space>>) -> usize {
    let mut tallest_rock = 0;
    for row in chamber {
        if row.iter().all(|s| *s ==  Empty) {
            break;
        } else {
            tallest_rock += 1;
        }
    }
    tallest_rock
}
fn run_cycle(input: &Vec<Direction>, total_rocks_to_drop: usize) -> usize{
    
    // create the shapes
    const VSPACE: usize = 3;
    let shapes: Vec<Vec<Vec<Space>>> = vec![
        vec![vec![Rock,Rock,Rock,Rock]], // hline
        vec![vec![Empty,Rock,Empty],vec![Rock,Rock,Rock],vec![Empty,Rock,Empty]], // cross
        vec![vec![Empty,Empty,Rock],vec![Empty,Empty,Rock],vec![Rock,Rock,Rock]], // elbow
        vec![vec![Rock],vec![Rock],vec![Rock],vec![Rock]], // vline
        vec![vec![Rock,Rock],vec![Rock,Rock]], // square
    ];

    // initialize the world
    let mut chamber: Vec<Vec<Space>> = Vec::new();
    let mut wind_index = 0;
    let mut rock_count = 0;
    let mut cycle_search: HashMap<(usize,usize,u32), (usize,usize)> = HashMap::new();

    //  start -> initial rock count -> height
    let mut initial_rocks = 0;
    let mut initial_height = 0;
    //  cycle -> rocks per cycle -> height per cycle
    let mut cycles_to_skip = 0;
    let mut rocks_per_cycle = 0;
    let mut height_per_cycle = 0;
    //  remaining rocks -> total rocks - (rocks per cyle * cycles) - initial rock count
    let mut remaining_rocks = 0;
    //  remaining height -> remaining rocks past start of cycle -> height
    let mut remaining_height = 0;
    // total height = initial height + (height per cycle * cycle)  + remaining height
    let mut height_skipped = 0;
    let mut cycle_found = false;

    // for every rock we drop...
    loop {

        if rock_count == total_rocks_to_drop {
            println!("stopped at rock {} with idx {}", rock_count, rock_count % shapes.len());
            break;
        }

        let rock_idx = rock_count % shapes.len();
        let tallest_rock = find_tallest_rock(&chamber);
        let mut shape_top_left = (2, tallest_rock as i32 + (shapes[rock_idx].len()) as i32 - 1 + VSPACE as i32);

        // pad the chamber with empty lines so we have a place to put the rock
        while chamber.len() <= shape_top_left.1 as usize{
            chamber.push(vec![Empty;WIDTH]);
        }

        // add next rock
        let mut row_count = 0;
        for line in shapes[rock_idx].iter().rev() { // consider pre-flipping?
            let mut padded_line: Vec<Space> = Vec::new();
            padded_line.push(Empty);
            padded_line.push(Empty);
            padded_line.append(&mut line.clone());
            while padded_line.len() < WIDTH {
                padded_line.push(Empty);
            }
            chamber[tallest_rock + VSPACE + row_count] = padded_line;
            row_count += 1;
        }

        'rock_movement: loop {
    
            // puff of air
            attempt_shift(&mut chamber, input[wind_index%input.len()], &shapes[rock_idx], &mut shape_top_left);
            wind_index += 1;
            // decend one
            if !attempt_shift(&mut chamber, Direction::Down, &shapes[rock_idx], &mut shape_top_left) {
                // we hit bottom, leave it here
                break 'rock_movement;
            }
        }
        rock_count += 1;

        let current_height = find_tallest_rock(&chamber);
        if current_height > 4 {

            if !cycle_found {
                
                let row1:u8 = chamber[current_height].iter().enumerate()
                .map(|(a,b)| {
                    if b != &Empty {
                        1 << a
                    } else {
                        0
                    }
                    })
                .fold(0, |acc, a| acc | a);
            let row2:u8 = chamber[current_height-1].iter().enumerate()
                .map(|(a,b)| {
                    if b != &Empty {
                        1 << a
                    } else {
                        0
                    }
                    })
                .fold(0, |acc, a| acc | a);
            let row3:u8 = chamber[current_height-2].iter().enumerate()
                .map(|(a,b)| {
                    if b != &Empty {
                        1 << a
                    } else {
                        0
                    }
                    })
                .fold(0, |acc, a| acc | a);
            let row4:u8 = chamber[current_height-3].iter().enumerate()
                .map(|(a,b)| {
                    if b != &Empty {
                        1 << a
                    } else {
                        0
                    }
                    })
                .fold(0, |acc, a| acc | a);
                let key = (wind_index % input.len(), rock_count % shapes.len(), 
                    (row1 as u32) << 24 | 
                    (row2 as u32) << 16| 
                    (row3 as u32) << 8 |
                    (row4 as u32));
                if cycle_search.contains_key(&key) {
                    cycle_found = true;
                    print_chamber_range(&chamber, chamber.len()-1, chamber.len() - 10 );
                    println!("found cycle at height: {}, wind: {wind_index} rock: {rock_count} previously seen at rock {} with height {}", 
                        current_height, cycle_search.get(&key).unwrap().0,cycle_search.get(&key).unwrap().1 );
                    initial_rocks = cycle_search.get(&key).unwrap().0;
                    initial_height = cycle_search.get(&key).unwrap().1;
                    rocks_per_cycle = rock_count - cycle_search.get(&key).unwrap().0;
                    height_per_cycle = current_height- cycle_search.get(&key).unwrap().1;
                    cycles_to_skip = ((total_rocks_to_drop - current_height) / rocks_per_cycle); // we already found one cycle
                    height_skipped = cycles_to_skip * height_per_cycle;
                    println!("cycle rocks: {rocks_per_cycle}, cycle height: {height_per_cycle}, skipping {cycles_to_skip} cycles with {} rocks and height {height_skipped}", (rocks_per_cycle * cycles_to_skip) );
                    println!("cycle started at {initial_height} repeated at {current_height} then repeated until height {height_skipped}");

                    remaining_rocks = total_rocks_to_drop - (rocks_per_cycle * cycles_to_skip) - rock_count;
                    println!("continuing for {remaining_rocks} more rocks");
                    rock_count = total_rocks_to_drop - remaining_rocks;

                } else {
                    cycle_search.insert(key, (rock_count, find_tallest_rock(&chamber)));
                }
            }
        }
        //print_chamber(&chamber);
    }

    //if chamber.len() > 320 {
    //    print_chamber_range(&chamber, chamber.len()-1, chamber.len() - 320);
    //} else {
    //    print_chamber(&chamber);
    //}
    let tallest_rock = find_tallest_rock(&chamber);
    println!("dropped {} rocks, chamber is {} rows tall, {} have content, skipped {height_skipped}", total_rocks_to_drop, chamber.len(), tallest_rock);
    tallest_rock + height_skipped
}
fn main() {

    // initialize wind list
    let input: Vec<Direction> = if IS_SAMPLE {
            b">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"
        } else {
            include_str!("../input17.txt").as_bytes()
        }.iter()
        .map(|c| if *c == b'>' {
                Direction::Right
            } else if *c == b'<' {
                Direction::Left
            } else {
                panic!("unexpected input direction")
            })
        .collect();

    // how many rocks should we drop?
    //let total_rocks: usize = 10;
    //let total_rocks: usize = 200;
    //let total_rocks: usize = 2_022;
    //let cycle_size = input.len() * 5 * 2;

    let cycle_size = TOTAL_ROCKS;

    //let height = if total_rocks <= cycle_size {
    //    println!("running one test with {} total rocks", TOTAL_ROCKS);
    //    run_cycle(&input, total_rocks)
    //} else {
        println!("running one test with {} total rocks", cycle_size);
        let height = run_cycle(&input, cycle_size);
        //let cycle_count = total_rocks/cycle_size;
        //let remainder = total_rocks - (cycle_count*cycle_size);

        //println!("running a second test with {} total rocks", remainder);
        //let total = cycle_count * cycle_height + run_cycle(&input, remainder);
        //total
    //};
    println!("{}", height);
      
}
