const IS_SAMPLE: bool = true;
const WIDTH: usize = 7;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Down,
}

fn print_chamber_range( chamber: & Vec<Vec<u8>>, top: usize, bottom: usize) {
    for r in (bottom..=top).rev() {
        print!("|");
        for c in &chamber[r] {
            if *c == 1 {
                print!("@");
            } else if *c == 2 {
                print!("#");
            } else {
                print!(".")
            }
        }
        print!("|\n");
    }
    if bottom == 0 {
        println!("+-------+");
    }
}

fn print_chamber( chamber: & Vec<Vec<u8>>) {
    print_chamber_range(chamber, chamber.len()-1, 0);
}

fn set_shape_at_coord( chamber: &mut Vec<Vec<u8>>, shape: &Vec<Vec<u8>>, coords: (i32,i32), new_val: u8) -> () {
    let shape_height = shape.len();
    let shape_width = shape[0].len();
    for sy in 0..shape_height { // for each verticle row in the shape
        for sx in 0..shape_width {
            if shape[sy][sx] == 1 { // check the shape bitmap
                let x = (coords.0) as usize + sx;
                let y = coords.1 as usize - sy;
                println!("chamber at x={x} y={y} was {} now {}", chamber[y][x], new_val );
                chamber[y][x] = new_val;
            }
        }
    }
}

fn attempt_shift( chamber: &mut Vec<Vec<u8>>, direction: Direction, shape: &Vec<Vec<u8>>, coords: &mut (i32,i32)) -> bool {
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
                    if shape[sy][sx] == 1 {
                        if chamber[coords.1 as usize -sy][(coords.0-1) as usize + sx] == 2 {
                            return false;
                        }
                    }
                }
            }
            set_shape_at_coord(chamber, shape, *coords, 0);
            *coords = (coords.0 -1, coords.1);
            set_shape_at_coord(chamber, shape, *coords, 1);
        },
        Direction::Right => {
            // if we're on the right wall then fail
            if coords.0 as usize + shape_width >= WIDTH {
                return false;
            }
            // check for collision
            for sy in 0..shape_height { // for each verticle row in the shape
                for sx in 0..shape_width {
                    if shape[sy][sx] == 1 {
                        if chamber[coords.1 as usize - sy][(coords.0+1) as usize + sx] == 2 {
                            return false;
                        }
                    }
                }
            }
            set_shape_at_coord(chamber, shape, *coords, 0);
            *coords = (coords.0 +1, coords.1);
            set_shape_at_coord(chamber, shape, *coords, 1);
        },
        Direction::Down => {
            // if we're on the bottom then fail
            if coords.1 - shape_height as i32 <= 0 {
                set_shape_at_coord(chamber, shape, *coords, 2);
                return false;
            }
            // check for collision
            for sy in 0..shape_height { // for each verticle row in the shape
                for sx in 0..shape_width {
                    if shape[sy][sx] == 1 {
                        if chamber[(coords.1 -1) as usize - sy][(coords.0) as usize + sx] == 2 {
                            return false;
                        }
                    }
                }
            }
            set_shape_at_coord(chamber, shape, *coords, 0);
            *coords = (coords.0, coords.1 -1);
            set_shape_at_coord(chamber, shape, *coords, 1);
        },
    }
    true
}
fn main() {
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
        }).collect();

    const TOTAL_ROCKS: usize = if IS_SAMPLE {
        9
    } else {
        2023
    };

    const VSPACE: usize = 3;
    let shapes: Vec<Vec<Vec<u8>>> = vec![
        vec![vec![1,1,1,1]], // hline
        vec![vec![0,1,0],vec![1,1,1],vec![0,1,0]], // cross
        vec![vec![0,0,1],vec![0,0,1],vec![1,1,1]], // elbow
        vec![vec![1],vec![1],vec![1],vec![1]], // vline
        vec![vec![1,1],vec![1,1]], // square
    ];

    let mut chamber: Vec<Vec<u8>> = Vec::new();
    let mut wind_index = 0;
    let mut rock_count = 0;

    loop {
        // if total rocks == 2023
        if rock_count == TOTAL_ROCKS {
            break;
        }

        // add 3 rows
        let mut empty_rows: usize = 0;
        for row in chamber.iter().rev() {
            if row.iter().all(|s| *s ==  0) {
                empty_rows += 1;
            } else {
                break;
            }
        }

        for _r in empty_rows..VSPACE {
            chamber.push(vec![0;WIDTH]);
        }

        // add next rock
        let rock_idx = rock_count % shapes.len();
        let mut shape_top_left = (2, (chamber.len() -1) as i32 + (shapes[rock_idx].len() -1) as i32);
        for line in shapes[rock_idx].iter().rev() { // consider pre-flipping?
            let mut padded_line: Vec<u8> = Vec::new();
            padded_line.push(0);
            padded_line.push(0);
            padded_line.append(&mut line.clone());
            while padded_line.len() < WIDTH {
                padded_line.push(0);
            }
            chamber.push(padded_line);
        }

        'rock_movement: loop {
    
            print_chamber(&chamber);
            // puff of air
            attempt_shift(&mut chamber, input[wind_index], &shapes[rock_idx], &mut shape_top_left);
            wind_index += 1;
            // decend one
            if !attempt_shift(&mut chamber, Direction::Down, &shapes[rock_idx], &mut shape_top_left) {// down
                // we hit bottom, leave it here
                break 'rock_movement;
            }
        }
        rock_count += 1;
    }

    let mut empty_rows: usize = 0;
    loop {
        if let Some(row) = chamber.get(chamber.len() - empty_rows) {
            if row.iter().all(|s| *s ==  0) {
                empty_rows += 1;
            } else {
                break;
            }
        }
    }
    println!("chamber is {} rows tall, {} have content", chamber.len(), chamber.len()-empty_rows);
}
