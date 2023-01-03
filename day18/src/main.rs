use regex::Regex;
use std::collections::HashSet;

const IS_SAMPLE: bool = false;

struct Boundaries {
    min_x: i8,
    min_y: i8,
    min_z: i8,
    max_x: i8,
    max_y: i8,
    max_z: i8,
}

fn get_cube_grid( input: &str) -> (Boundaries, HashSet<(i8,i8,i8)>) {
    let mut cube_grid = HashSet::new();

    let mut boundaries = Boundaries{min_x:0,min_y:0,min_z:0,max_x:0,max_y:0,max_z:0};

    let re: Regex = Regex::new(r"([0-9]+),([0-9]+),([0-9]+)$").unwrap();
    for line in input.lines() {
        let cap = re.captures(line);
        match cap {
            Some(capture) => {
                let x:i8 = capture[1].parse().unwrap();
                let y:i8 = capture[2].parse().unwrap();
                let z:i8 = capture[3].parse().unwrap();
                if x < boundaries.min_x {
                    boundaries.min_x = x;
                }
                if x > boundaries.max_x {
                    boundaries.max_x = x;
                }
                if y < boundaries.min_y {
                    boundaries.min_y = y;
                }
                if y > boundaries.max_y {
                    boundaries.max_y = y;
                }
                if z < boundaries.min_z {
                    boundaries.min_z = z;
                }
                if z > boundaries.max_z {
                    boundaries.max_z = z;
                }
                cube_grid.insert((x,y,z));
            }
            _ => panic!("didn't match line"),
        }
    }
    println!("found {} cubes in range X [{}..{}] Y [{}..{}] Z [{}..{}]", 
        cube_grid.len(), 
        boundaries.min_x,boundaries.max_x,
        boundaries.min_y,boundaries.max_y, 
        boundaries.min_z, boundaries.max_z);
    (boundaries, cube_grid)
}

fn flood_outer_envelope( boundaries: &Boundaries, surface: HashSet<(i8,i8,i8)>)-> HashSet<(i8,i8,i8)> {
    let mut envelope = HashSet::new();
    envelope
}

fn count_all_surface(boundaries: &Boundaries, cube_grid: &HashSet<(i8,i8,i8)>) -> usize {
    let adjascent: Vec<(i8,i8,i8)> = vec![(-1,0,0), (1,0,0),(0,-1,0),(0,1,0), (0,0,-1),(0,0,1)];
    let mut surface_area = 0;
    for x in boundaries.min_x..=boundaries.max_x {
        for y in boundaries.min_y..=boundaries.max_y {
            for z in boundaries.min_z..=boundaries.max_z {
                if cube_grid.contains(&(x,y,z)) {
                    surface_area += adjascent.iter()
                        .filter(|&delta| {
                            //println!("checking for neighbor of ({x},{y},{z}) at {:?}", delta);
                            !cube_grid.contains(&(x+delta.0,y+delta.1,z+delta.2))
                        })
                        .count();
                }
            }
        }
    }
    println!("surface was {surface_area}");
    surface_area
}

fn main() {
    const INPUT: &str = if IS_SAMPLE {
        include_str!("../sample.txt")
    } else {
        include_str!("../input18.txt")
    };

    let result = get_cube_grid(INPUT);
    let boundaries = result.0;
    let cube_grid: HashSet<(i8,i8,i8)> = result.1;
    //let adjascent = (-1..=1).map(|_| -1..=1).multi_cartesian_product();
    count_all_surface(&boundaries, &cube_grid);

}