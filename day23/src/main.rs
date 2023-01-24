use std::collections::{HashMap, HashSet};

const IS_SAMPLE: bool = false;
const ROUNDS: usize = 10;

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    for line in input.lines() {
        grid.push(line.as_bytes().iter().map(|b| *b).collect());
    }
    grid
}
fn find_elves_in_grid(grid: &Vec<Vec<u8>>, offset: i32) -> HashSet<(i32, i32)> {
    let elves: HashSet<(i32, i32)> = grid
        .iter()
        .enumerate()
        .flat_map(|row| {
            let my_row = row.0 as i32 + offset;
            row.1
                .iter()
                .enumerate()
                .filter(|col| *col.1 == b'#')
                .map(move |col| (col.0 as i32 + offset, my_row))
        })
        .collect::<HashSet<(i32, i32)>>();
    elves
}

fn has_neighbors(elf: &(i32, i32), elves: &HashSet<(i32, i32)>) -> bool {
    let neighbors = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    neighbors
        .iter()
        .map(|c| (elf.0 + c.0, elf.1 + c.1))
        .any(|c| elves.contains(&c))
}
fn print_grid(elves: &HashSet<(i32,i32)>) {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    for elf in elves {
        min_x = min_x.min(elf.0);
        max_x = max_x.max(elf.0);
        min_y = min_y.min(elf.1);
        max_y = max_y.max(elf.1);
    }
    
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if elves.contains(&(x,y)) {
                print!("#");
            }else{
                print!(".")
            }
        }
        println!("");
    }

}
fn main() {
    const INPUT: &str = if IS_SAMPLE {
        include_str!("../sample.txt")
    } else {
        include_str!("../input23.txt")
    };

    let moves = [
        [(-1, -1), ( 0, -1), ( 1, -1)], // N
        [(-1,  1), ( 0,  1), ( 1,  1)], // S
        [(-1, -1), (-1,  0), (-1,  1)], // W
        [( 1, -1), ( 1,  0), ( 1,  1)], // E
    ];

    let grid = parse_input(INPUT);
    let mut elves = find_elves_in_grid(&grid, 0);
    let mut tried_to_move: bool;

    for step in 0..ROUNDS {
        tried_to_move = false;
        println!("start of round {step}");
        print_grid(&elves);
        let mut planned_moves: HashMap<(i32, i32), Vec<&(i32, i32)>> = HashMap::new();
        for elf in &elves {
            // check adjascent
            if !has_neighbors(elf, &elves) {
                planned_moves.insert(elf.clone(), vec![elf]);
            } else {
                tried_to_move = true;
                // check dirs
                let mut did_move = false;
                for offset in 0..4 {
                    if !moves[(step + offset) % 4]
                        .iter()
                        .map(|c| (elf.0 + c.0, elf.1 + c.1))
                        .any(|c| elves.contains(&c))
                    {
                        let new_pos = (
                            elf.0 + moves[(step + offset) % 4][1].0,
                            elf.1 + moves[(step + offset) % 4][1].1,
                        );
                        match planned_moves.get_mut(&new_pos) {
                            None => {
                                planned_moves.insert(new_pos, vec![elf]);
                            }
                            Some(list) => {
                                list.push(elf);
                            }
                        };
                        did_move = true;
                        break;
                    }
                }
                if !did_move {
                    planned_moves.insert(*elf, vec![elf]);
                }
            }
        }
        if !tried_to_move {
            break;
        }
        // check for collisions
        let mut new_elves = HashSet::new();
        for pmove in planned_moves {
            if pmove.1.len() == 1 {
                new_elves.insert(pmove.0);
            } else {
                for elf in pmove.1 {
                    new_elves.insert(elf.clone());
                }
            }
        }
        elves = new_elves;
    }

    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    for elf in &elves {
        min_x = min_x.min(elf.0);
        max_x = max_x.max(elf.0);
        min_y = min_y.min(elf.1);
        max_y = max_y.max(elf.1);
    }
    println!("final grid state");
    print_grid(&elves);
    let total_squares = (max_x +1 - min_x) * (max_y+1 - min_y);
    println!("empty squares {}", total_squares as usize - elves.len());
}
