use regex::Regex;
use std::collections::HashSet;

const IS_SAMPLE: bool = false;
fn main() {
    const INPUT: &str = if IS_SAMPLE {
        include_str!("../sample.txt")
    } else {
        include_str!("../input18.txt")
    };

    let re: Regex = Regex::new(r"([0-9]+),([0-9]+),([0-9]+)$").unwrap();
    let mut cube_grid: HashSet<(i8,i8,i8)> = HashSet::new();

    let mut min_x = i8::MAX;
    let mut min_y = i8::MAX;
    let mut min_z = i8::MAX;
    let mut max_x = i8::MIN;
    let mut max_y = i8::MIN;
    let mut max_z = i8::MIN;

    for line in INPUT.lines() {
        let cap = re.captures(line);
        match cap {
            Some(capture) => {
                let x:i8 = capture[1].parse().unwrap();
                let y:i8 = capture[2].parse().unwrap();
                let z:i8 = capture[3].parse().unwrap();
                if x < min_x {
                    min_x = x;
                }
                if x > max_x {
                    max_x = x;
                }
                if y < min_y {
                    min_y = y;
                }
                if y > max_y {
                    max_y = y;
                }
                if z < min_z {
                    min_z = z;
                }
                if z > max_z {
                    max_z = z;
                }
                cube_grid.insert((x,y,z));
            }
            _ => panic!("didn't match line"),
        }
    }
    println!("found {} cubes in range X [{min_x}..{max_x}] Y [{min_y}..{max_y}] Z [{min_z}..{max_z}]", cube_grid.len());
    //let adjascent = (-1..=1).map(|_| -1..=1).multi_cartesian_product();
    let adjascent: Vec<(i8,i8,i8)> = vec![(-1,0,0), (1,0,0),(0,-1,0),(0,1,0), (0,0,-1),(0,0,1)];
    let mut surface_area = 0;
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            for z in min_z..=max_z {
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
}