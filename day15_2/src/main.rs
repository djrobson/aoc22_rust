use regex::Regex;
use std::collections::BTreeMap;

fn manhattan_distance(s:(i32,i32),b:(i32,i32)) -> i32 {
    let dx = (s.0.abs() - b.0.abs()).abs();
    let dy = (s.1.abs() - b.1.abs()).abs();
    dx + dy
}
#[derive(PartialEq)]
enum Location {
    Sensor,
    Beacon,
    Empty,
    Unknown,
}

struct Pair {
    sensor: (i32,i32),
    beacon: (i32,i32),
    distance: i32,
}

fn manhattan_range_for_row(coord: (i32,i32), row: i32, distance: i32) -> Option<(i32,i32)> {
    let x = coord.0;
    let y = coord.1;

    if y < row && y+distance < row {
        return None;
    }
    if y > row && y-distance > row {
        return None;
    }
    //let mut coords: Vec<(i32,i32)> = Vec::new();
    //for dy in y-distance..=y+distance {
        let dy = row;
        let left_over = if dy < y {
            distance - (y-dy)
        } else {
            distance - (dy -y)
        };
        //for dx in x-left_over..=x+left_over {
        //    coords.push((dx,dy));
        //}
    //}
    Some((x-left_over, x+left_over))
}
const IS_SAMPLE: bool = false;
const EXPECTED_ROW: i32 = if IS_SAMPLE {
    10  
} else {
    2000000
};
const CAVE_RANGE: i32 = if IS_SAMPLE {
    20 
}else {
    4_000_000
};

fn step1(sensor_beacon_pairs: Vec<Pair>) -> () {
    //let mut empty_in_row = 0;
    let mut ranges: Vec<(i32,i32)> = Vec::new();
    for pair in &sensor_beacon_pairs {
        let sensor= &pair.sensor;
        let distance = &pair.distance;
        if let Some(range) = manhattan_range_for_row(*sensor, EXPECTED_ROW, *distance) {
            ranges.push(range);
        }
    }
    ranges.sort();
    let range = ranges.into_iter().reduce(|a,b| {
        println!("Comparing ({},{}) to ({},{})", a.0,a.1,b.0,b.1);
        // as .. bs .. ae .. be
        if a.0 < b.0 && a.1 >= b.0 && a.1 <= b.1 {
            (a.0,b.1)
        } // bs .. as .. be .. ae
        else if b.0 < a.0 && b.1 >= a.0 && b.1 <= a.1 {
            (b.0,a.1)
        } // as .. bs .. be .. ae
        else if a.0 <= b.0 && a.1 >= b.1 {
            (a.0,a.1)
        } // bs .. as .. ae .. be
        else if b.0 <= a.0 && b.1 >= a.1 {
            (b.0,b.1)
        } // as .. ae .. bs .. be || bs .. be .. as .. ae
        else {
            panic!("it didn't overlap and won't reduce easily")
        }
    }).unwrap();

    // plus 1 for the 0 index, -1 for the beacon
    println!("found range {:?} with {} empty spots in row {EXPECTED_ROW}",range, range.1-range.0);

}

fn step2(sensor_beacon_pairs: Vec<Pair>) -> () {
    //let mut empty_in_row = 0;
    for urow in 0..=CAVE_RANGE {
        let row = CAVE_RANGE - urow;
        let mut ranges: Vec<(i32,i32)> = Vec::new();
        for pair in &sensor_beacon_pairs {
            let sensor= &pair.sensor;
            let distance = &pair.distance;
            if let Some(range) = manhattan_range_for_row(*sensor, row, *distance) {
                ranges.push(range);
            }
        }
        if row == 957210 {
            println!("here");
        }
        ranges.sort();
        let _range = ranges.into_iter().reduce(|a,b| {
            //println!("Comparing ({},{}) to ({},{})", a.0,a.1,b.0,b.1);
            // as .. bs .. ae .. be
            if a.0 < b.0 && a.1 >= b.0 && a.1 <= b.1 {
                (a.0,b.1)
            } // bs .. as .. be .. ae
            else if b.0 < a.0 && b.1 >= a.0 && b.1 <= a.1 {
                (b.0,a.1)
            } // as .. bs .. be .. ae
            else if a.0 <= b.0 && a.1 >= b.1 {
                (a.0,a.1)
            } // bs .. as .. ae .. be
            else if b.0 <= a.0 && b.1 >= a.1 {
                (b.0,b.1)
            } // as .. ae .. bs .. be || bs .. be .. as .. ae
            else {
                println!("Comparing ({},{}) to ({},{})", a.0,a.1,b.0,b.1);
                println!("found gap at X {} Y {row} with tunning {}", a.1+1, ((a.1+1) as usize * 4_000_000) + row as usize);
                panic!("program done");
            }
        });
    }

    // plus 1 for the 0 index, -1 for the beacon
    //println!("found range {:?} with {} empty spots in row {EXPECTED_ROW}",range, range.1-range.0);

}

fn main() {
    const INPUT: &str = if IS_SAMPLE {
        include_str!("../sample.txt")
    } else {
        include_str!("../input15.txt")
    };
    
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    let mut max_dist = i32::MIN;

    let re: Regex = Regex::new(r"[^\-0-9]+([\-0-9]+)[^\-0-9]+([\-0-9]+)[^\-0-9]+([\-0-9]+)[^\-0-9]+([\-0-9]+)").unwrap();
    let mut sensor_beacon_pairs: Vec<Pair> = Vec::new();
    let mut tunnel: BTreeMap<(i32,i32), Location> = BTreeMap::new();

    for line in INPUT.lines() {
        let cap = re.captures(line);
        match cap {
            Some(capture) => {
                let sensor = (capture[1].parse().unwrap(),capture[2].parse().unwrap());
                let beacon = (capture[3].parse().unwrap(),capture[4].parse().unwrap());
                let distance = manhattan_distance(sensor, beacon);
                
                if sensor.0 < min_x {
                    min_x = sensor.0;
                }
                if sensor.0 > max_x {
                    max_x = sensor.0;
                }
                if beacon.0 < min_x {
                    min_x = beacon.0;
                }
                if beacon.0 > max_x {
                    max_x = beacon.0;
                }
                if sensor.1 < min_y {
                    min_y = sensor.1;
                }
                if sensor.1 > max_y {
                    max_y = sensor.1;
                }
                if beacon.1 < min_y {
                    min_y = beacon.1;
                }
                if beacon.1 > max_y {
                    max_y = beacon.1;
                }
                if distance > max_dist {
                    max_dist = distance;
                }
                sensor_beacon_pairs.push(
                    Pair{sensor: sensor, 
                        beacon: beacon, 
                        distance: distance} );
                tunnel.insert(sensor, Location::Sensor);
                tunnel.insert(beacon, Location::Beacon);
                
            }
            _ => panic!("didn't match line"),
        }
    }
    println!("Found {} sensor beacon pairs", sensor_beacon_pairs.len());
    println!("({min_x},{min_y}) to ({max_x},{max_y}) max dist {max_dist}");
    step2(sensor_beacon_pairs);
}
