use regex::Regex;

fn main() {
    const IS_SAMPLE: bool = true;
    const INPUT: &str = if IS_SAMPLE {
        include_str!("../sample.txt")
    } else {
        include_str!("../input15.txt")
    };
    let re: Regex = Regex::new(r"[^\-0-9]+([\-0-9]+)[^\-0-9]+([\-0-9]+)[^\-0-9]+([\-0-9]+)[^\-0-9]+([\-0-9]+)").unwrap();
    let mut sensor_beacon_pairs: Vec<((i32,i32),(i32,i32))> = Vec::new();

    for line in INPUT.lines() {
        let cap = re.captures(line);
        match cap {
            Some(capture) => {
                sensor_beacon_pairs.push(
                    (
                        (capture[1].parse().unwrap(),capture[2].parse().unwrap()),
                        (capture[3].parse().unwrap(),capture[4].parse().unwrap())
                    )
                )
            }
            _ => panic!("didn't match line"),
        }
    }
    
    println!("Found {} sensor beacon pairs", sensor_beacon_pairs.len());
}