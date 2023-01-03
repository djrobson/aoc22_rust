use regex::Regex;
use std::collections::HashSet;

const IS_SAMPLE: bool = false;

#[derive(Debug)]
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
fn is_outside_boundary(point: (i8,i8,i8), boundary: &Boundaries) -> bool {
    if point.0 >= boundary.min_x && point.0 <= boundary.max_x &&
        point.1 >= boundary.min_y && point.1 <= boundary.max_y &&
        point.2 >= boundary.min_z && point.2 <= boundary.max_z {
        //println!("{:?} is inside {:?}", point, boundary);
        false
    } else {
        //println!("{:?} is outside {:?}", point, boundary);
        true
    }
}
fn flood_outer_envelope( boundaries: &Boundaries, surface: &HashSet<(i8,i8,i8)>)-> HashSet<(i8,i8,i8)> {
    let mut envelope = HashSet::new();
    let mut check_queue: Vec<(i8,i8,i8)> = Vec::new();

    // left and right X planes
    for y in boundaries.min_y..boundaries.max_y {
        for z in boundaries.min_z..boundaries.max_z {
            check_queue.push((boundaries.min_x,y,z));
            check_queue.push((boundaries.max_x,y,z));
        }
    }
    // bottom and top Y planes 
    for x in boundaries.min_x..boundaries.max_x {
        for z in boundaries.min_z..boundaries.max_z {
            check_queue.push((x,boundaries.min_y,z));
            check_queue.push((x,boundaries.max_y,z));
        }
    }
    // front and back Z planes
    for y in boundaries.min_y..boundaries.max_y {
        for x in boundaries.min_x..boundaries.max_x {
            check_queue.push((x,y,boundaries.min_z));
            check_queue.push((x,y,boundaries.max_z));
        }
    }
    
    let adjascent: Vec<(i8,i8,i8)> = vec![(-1,0,0), (1,0,0),(0,-1,0),(0,1,0), (0,0,-1),(0,0,1)];
    while check_queue.len() != 0 {
        let next = check_queue.pop().unwrap();
        if surface.contains(&next) {
            // this is inside the rock
            continue;
        }
        if envelope.contains(&next) {
            // we already counted this one
            continue;
        }
        // point is reachable from the outside
        envelope.insert(next);
        
        'neighbor_checks: for delta in adjascent.iter() {
            let point = (next.0+delta.0,next.1+delta.1,next.2+delta.2);
            if envelope.contains(&point) {
                // we already counted this one
                continue 'neighbor_checks;
            }
            if surface.contains(&point) {
                continue 'neighbor_checks;
            }
            if is_outside_boundary(point, boundaries) {
                continue 'neighbor_checks;
            }
            //println!("queuing ({:?}) at {:?}", next, delta);
            check_queue.push(point);
        }
    }
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
    println!("outer and inner surface was {surface_area}");
    surface_area
}
fn count_outer_surface(boundaries: &Boundaries, cube_grid: &HashSet<(i8,i8,i8)>) -> usize {
    let adjascent: Vec<(i8,i8,i8)> = vec![(-1,0,0), (1,0,0),(0,-1,0),(0,1,0), (0,0,-1),(0,0,1)];
    let mut surface_area = 0;
    let envelope = flood_outer_envelope(boundaries, cube_grid);

    for x in boundaries.min_x..=boundaries.max_x {
        for y in boundaries.min_y..=boundaries.max_y {
            for z in boundaries.min_z..=boundaries.max_z {
                if cube_grid.contains(&(x,y,z)) {
                    surface_area += adjascent.iter()
                        .filter(|&delta| {
                            //println!("checking for neighbor of ({x},{y},{z}) at {:?}", delta);
                            let point = (x+delta.0,y+delta.1,z+delta.2);
                            envelope.contains(&point) || is_outside_boundary(point, boundaries)
                        })
                        .count();
                }
            }
        }
    }
    println!("outer surface was {surface_area}");
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
    count_all_surface(&boundaries, &cube_grid);
    count_outer_surface(&boundaries, &cube_grid);
}