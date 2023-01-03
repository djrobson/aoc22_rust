use regex::Regex;
const IS_SAMPLE: bool = true;
const TOTAL_TIME: u8 = 24;

struct Recipe {
    ore: u8,
    clay: u8,
    obsidian: u8,
    geode: u8,
}

struct State {
    time: u8,
    ore: u8,
    clay: u8,
    obsidian: u8,
    geode: u8,
    ore_robot: u8,
    clay_robot: u8,
    obsidian_robot: u8,
    geode_robot: u8,
}

fn p1_optimize(recipe: &Recipe) -> usize{
    0
}
fn main() {
    const INPUT: &str = if IS_SAMPLE {
        include_str!("../sample.txt")
    } else {
        include_str!("../input19.txt")
    };

    let mut recipes: Vec<Recipe> = Vec::new();
    // Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
    let re: Regex = Regex::new(r".*([0-9])+ ore.*([0-9]+) ore.*([0-9]+) ore and ([0-9]+) clay.*([0-9]+) ore and ([0-9]+) obsidian.").unwrap();
    for line in INPUT.lines() {
        let cap = re.captures(line);
        match cap {
            Some(capture) => {
                recipes.push(Recipe{ore: capture[1].parse::<u8>().unwrap(), 
                                   clay: capture[2].parse::<u8>().unwrap(), 
                               obsidian: capture[3].parse::<u8>().unwrap(), 
                                  geode: capture[4].parse::<u8>().unwrap()});
            }
            _ => panic!("didn't match line"),
        }
    }

    // calculate the optimal route
    let p1_total: usize = recipes.iter().enumerate().map(|(index,rec)| index * p1_optimize(rec)).sum();
    println!("part1 total: {p1_total}");
}