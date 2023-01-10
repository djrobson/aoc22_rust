// algorithm copied from https://nickymeuleman.netlify.app/garden/aoc2022-day19#final-code

use regex::Regex;
//use std::collections::HashMap;
use std::collections::VecDeque;
const IS_SAMPLE: bool = false;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct State {
    elapsed: u8,
    inventory: [u8;4],
    bots: [u8;4],
}

fn max_geodes(blueprint: &[[u8; 4]; 4], max_time: u8) -> u8 {
    // calculate the maximum amount for every type of bot so that the creation of a new bot of any type is never bottlenecked
    // it doesn't make sense to build more bots than that maximum if the resources a bot type generates are
    // enough to cover that type (ore, clay, obsidian) cost for any possible bot (per question, you can only build 1 bot per turn)
    // for geode bots, there is no logical maximum amount
    // [ore, clay, obsidian, geode]
    let mut max_robots = [u8::MAX; 4];
    for i in 0..3 {
        max_robots[i] = blueprint.iter().map(|cost| cost[i]).max().unwrap();
    }
    let mut max_geodes:u8 = 0;

    let mut q = VecDeque::new();
    q.push_back(State {
        inventory: [0, 0, 0, 0],
        bots: [1, 0, 0, 0],
        elapsed: 0,
    });

    while let Some(State {
        inventory,
        bots,
        elapsed,
    }) = q.pop_front()
    {
        // for every bot cost, run simulation
        for i in 0..blueprint.len() {
            // if we already have enough of this bot type, skip
            if bots[i] == max_robots[i] {
                continue;
            }

            let costs = &blueprint[i];

            // Find the limiting resource type for the costs.
            let wait_time = (0..3)
                .map(|idx| {
                    match costs[idx] {
                        // state has enough of current resource in inventory to cover that part of the target bot cost. 0 wait time
                        cost if cost <= inventory[idx] => 0,
                        // no target bot type made yet so we need to build something else first
                        _ if bots[idx] == 0 => max_time + 1,
                        _ => (costs[idx] - inventory[idx] + bots[idx] - 1) / bots[idx],
                    }
                })
                .max()
                .unwrap();

            // if that choice would cause the time limit be to exceeded, skip
            // the + 1 is so the built bot has the chance to do something, it merely being built is not enough
            let new_elapsed = elapsed + wait_time + 1;
            if new_elapsed >= max_time {
                continue;
            }

            // gather ores with previously available bots
            let mut new_inventory = [0; 4];
            for idx in 0..bots.len() {
                new_inventory[idx] = inventory[idx] + bots[idx] * (wait_time + 1) - costs[idx];
            }

            // increase bot type for the bot we just built
            let mut new_bots = bots.clone();
            new_bots[i] += 1;

            // extra optimization:
            // if we theoretically only built geode bots every turn, and we still don't beat the maximum, skip
            let remaining_time:u16 = max_time as u16 - new_elapsed as u16;
            if ((remaining_time - 1) * remaining_time) / 2
                + new_inventory[3] as u16
                + (remaining_time * new_bots[3] as u16)
                < (max_geodes as u16)
            {
                continue;
            }

            q.push_back(State {
                inventory: new_inventory,
                bots: new_bots,
                elapsed: new_elapsed,
            })
        }

        let geodes = inventory[3] + bots[3] * (max_time - elapsed);
        max_geodes = geodes.max(max_geodes);
    }

    max_geodes
}

fn main() {
    const INPUT: &str = if IS_SAMPLE {
        include_str!("../sample.txt")
    } else {
        include_str!("../input19.txt")
    };

    let mut recipes: Vec<[[u8;4];4]> = Vec::new();
    // Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
    let re: Regex = Regex::new(r".*([0-9])+ ore.*([0-9]+) ore.*([0-9]+) ore and ([0-9]+) clay.*([0-9]+) ore and ([0-9]+) obsidian.").unwrap();
    for line in INPUT.lines() {
        let cap = re.captures(line);
        match cap {
            Some(capture) => {
                recipes.push([[capture[1].parse::<u8>().unwrap(), 0,0,0],
                              [capture[2].parse::<u8>().unwrap(), 0,0,0],
                              [capture[3].parse::<u8>().unwrap(), capture[4].parse::<u8>().unwrap(), 0,0],
                              [capture[5].parse::<u8>().unwrap(), 0, capture[6].parse::<u8>().unwrap(),0]]);
            }
            _ => panic!("didn't match line"),
        }
    }

    // calculate the optimal route
    let p1_total: usize = recipes
        .iter()
        .enumerate()
        .map(|(index,rec)| (index + 1) * max_geodes(rec, 24) as usize)
        .sum();
    println!("part1 total: {p1_total}");
    let p2_total: usize = recipes
        .iter()
        .take(3)
        .map(|rec| max_geodes(rec, 32) as usize)
        .product();
    println!("part2 total: {p2_total}");
}