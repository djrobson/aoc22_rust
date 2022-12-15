
use std::fmt::Display;

#[derive(Clone,PartialEq,Debug)]
enum Voxel {
    Space,
    Rock,
    Sand,
    Void,
}
impl Display for Voxel {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Voxel::Space => ".",
                Voxel::Rock => "#",
                Voxel::Sand => "o",
                Voxel::Void => "X"
            }
        )
    }
}
/*impl Display for Vec<Vec<Voxel>> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Voxel::Space => ".",
                Voxel::Rock => "#",
                Voxel::Sand => "o",
                Voxel::Void => "X"
            }
        )
    }
}*/

fn print_cave(cave: &Vec<Vec<Voxel>>, x_range:(i32,i32), y_range: (i32,i32)) -> (){
    for y in y_range.0..=y_range.1 {
        for x in x_range.0..=x_range.1 {
            print!("{}",cave[y as usize][x as usize]);
        }
        print!("\n");
    }
    //print!("\n");
}

fn main() {
    const IS_SAMPLE: bool = false;
    const INPUT: &str = if IS_SAMPLE {
        include_str!("../sample.txt")
    } else {
        include_str!("../input14.txt")
    };

    let mut min_x = 999;
    let mut max_x = 0;
    let mut min_y = 999;
    let mut max_y = 0;

    // read the input
    let mut line_segments: Vec<Vec<(i32,i32)>> = Vec::new();
    for line in INPUT.lines() {
        let mut this_line: Vec<(i32,i32)> = Vec::new();
        for pair in line.split(" -> ") {
            let mut xy = pair.split(",");
            let x = xy.next().expect("failed to find x").parse::<i32>().expect("X wasn't a number");
            let y = xy.next().expect("failed to find y").parse::<i32>().expect("Y wasn't a number");
            if x < min_x {
                min_x = x;
            } else if x > max_x {
                max_x = x;
            }
            if y < min_y {
                min_y = y;
            } else if y > max_y {
                max_y = y;
            }
            this_line.push((x,y));
        }
        line_segments.push(this_line);
    }
    println!("cave dimensions {min_x} {max_x} {min_y} {max_y} {}", line_segments.len());

    // create an empty cave
    let mut cave: Vec<Vec<Voxel>> = Vec::new();
    for _y in 0..=max_y {
        cave.push(vec![Voxel::Space;max_x as usize+1]);
    }
    cave.push(vec![Voxel::Void;max_x as usize+1]);

    // fill in the rocks
    for line in line_segments {
        for seg in line.windows(2) {
            let from = seg[0];
            let to = seg[1];
            match (from,to) {
                ((fx,fy),(tx,ty)) if fx == tx  && fy != ty => {
                    // delta y
                    if fy < ty {
                        for sy in fy..=ty {
                            cave[sy as usize][fx as usize] = Voxel::Rock;
                        }
                    } else {
                        for sy in ty..=fy {
                            cave[sy as usize][fx as usize] = Voxel::Rock;
                        }
                    }
                }
                ((fx,fy),(tx,ty)) if fx != tx  && fy == ty => {
                    // delta x
                    if fx < tx {
                        for sx in fx..=tx {
                            cave[fy as usize][sx as usize] = Voxel::Rock;
                        }
                    } else {
                        for sx in tx..=fx {
                            cave[fy as usize][sx as usize] = Voxel::Rock;
                        }
                    }
                }
                _ => panic!("unexpected val"),
            }
        }
    }

    let origin = (500,0);
    let mut sand_grains = 0;

    'sand: loop { // for all failing sand
        let mut x = origin.0;
        let mut y = origin.1;

        sand_grains += 1;

        // fall until you hit bottom
        'falling: loop {

            //print_cave(&cave, (min_x,max_x),(min_y,max_y));
            // falling down
            if cave[y+1][x] == Voxel::Space {
                y += 1;
                continue 'falling;
            }
            if cave[y+1][x] == Voxel::Rock || cave[y+1][x] == Voxel::Sand {

                // falling down left
                if cave[y+1][x-1] == Voxel::Space {
                    y +=1;
                    x -=1;
                    continue 'falling;
                }
                // falling down right
                if cave[y+1][x+1] == Voxel::Space {
                    y +=1;
                    x +=1;
                    continue 'falling;
                }
                // fall center
                cave[y][x] = Voxel::Sand;
                continue 'sand;
            }
            if cave[y][x] == Voxel::Void 
                || cave[y][x-1] == Voxel::Void 
                || cave[y][x+1] == Voxel::Void {
                break 'sand;
            }
            
            println!("fell off at ({x},{y}) dropped {sand_grains}");
            panic!("didn't chose what to do");
        }
    }
    println!("dropped {sand_grains}");
}