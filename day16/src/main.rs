use regex::Regex;
use std::cmp;
use std::collections::HashMap;
const IS_SAMPLE: bool = false;

//#[derive(Copy)]
struct Room {
    bit_index: u8,
    room_index: u8,
    //name: String,
    rate: u32,
    edges: Vec<u8>,
}
#[derive(Hash, PartialEq, Eq)]
struct State {
    my_location: u8,
    ele_location: u8,
    tick: u8,
    opened_bits: u16,
}

/*fn dfs1( pos: String, state: State, rooms: &HashMap<String,Room>, cache: &mut HashMap<State, u32>, bit_count: u8) -> u32 {
    // if out of time
    if state.tick == 0 {
        return 0;
    }

    // if we already checked this cache
    if let Some(s) = cache.get(&state) {
        return *s;
    }

    // if everything is opened
    let mut all_open = true;
    for x in 1..=bit_count {
        if state.opened_bits & (1 << x) == 0 {
            all_open = false;
            break;
        }
    }
    if all_open {
        cache.insert(state, 0);
        return 0;
    }

    let mut max_rate = u32::MIN;
    // if we open this one
    let (my_rate, my_room_index, my_bit_index) = (rooms[&pos].rate, rooms[&pos].room_index, rooms[&pos].bit_index);
    if (my_rate > 0) && (0 == state.opened_bits & 1 << my_bit_index) {
        let new_state = State{location: my_room_index, tick: state.tick -1, opened_bits: state.opened_bits | 1 << my_bit_index};
        max_rate = my_rate * (state.tick-1) as u32 + dfs(pos.clone(), new_state, rooms, cache, bit_count );
    }

    // if we move to an neighbor
    let edges = rooms[&pos].edges.clone();
    for n in edges {
        let new_state = State{location: rooms[&n].room_index, tick: state.tick -1, opened_bits: state.opened_bits};
        max_rate = cmp::max(max_rate, dfs(n.clone(), new_state, rooms, cache, bit_count));
    }
    cache.insert(state, max_rate);
    max_rate
}*/

fn dfs2( state: State, rooms: &HashMap<u8,Room>, cache: &mut HashMap<State, u32>, bit_count: u8) -> u32 {
    // if out of time
    if state.tick == 0 {
        return 0;
    }

    // if we already checked this cache
    if let Some(s) = cache.get(&state) {
        return *s;
    }

    // if everything is opened
    let mut all_open = true;
    for x in 1..=bit_count {
        if state.opened_bits & (1 << x) == 0 {
            all_open = false;
            break;
        }
    }
    if all_open {
        cache.insert(state, 0);
        return 0;
    }

    let mut max_rate = u32::MIN;

    // if I open this round
    let (my_rate, my_room_index, my_bit_index) = (rooms[&state.my_location].rate, rooms[&state.my_location].room_index, rooms[&state.my_location].bit_index);
    if (my_rate > 0) && (0 == state.opened_bits & 1 << my_bit_index) {
        // if elephant opens this round
        let (ele_rate, ele_room_index, ele_bit_index) = (rooms[&state.ele_location].rate, rooms[&state.ele_location].room_index, rooms[&state.ele_location].bit_index);
        if (state.ele_location != state.my_location) && (ele_rate > 0) && (0 == state.opened_bits & 1 << ele_bit_index) {
            let both_open_state = State{my_location: state.my_location, ele_location: state.ele_location, tick: state.tick-1, 
                                               opened_bits: state.opened_bits |  1 << ele_bit_index | 1<< my_bit_index};
            max_rate = cmp::max( max_rate, 
                                my_rate * (state.tick-1) as u32 + 
                                ele_rate * (state.tick-1) as u32  +
                                dfs2(both_open_state, rooms, cache, bit_count));
        } else {
            // I open and elephant moves
            let edges = rooms[&ele_room_index].edges.clone();
            for n in edges {
                let new_state = State{my_location: state.my_location, ele_location: n, tick: state.tick-1, 
                    opened_bits: state.opened_bits | 1<< my_bit_index};
                max_rate = cmp::max( max_rate, 
                    my_rate * (state.tick-1) as u32 + dfs2(new_state, rooms, cache, bit_count));
            }
        }
    } else {
        // I move this round and ele opens or doesn't
        let (ele_rate, ele_room_index, ele_bit_index) = (rooms[&state.ele_location].rate, rooms[&state.ele_location].room_index, rooms[&state.ele_location].bit_index);
        if (state.ele_location != state.my_location) && (ele_rate > 0) && (0 == state.opened_bits & 1 << ele_bit_index) {
            // elephant opens, I move
            let edges = rooms[&my_room_index].edges.clone();
            for n in edges {
                let new_state = State{my_location: n, ele_location: state.ele_location, tick: state.tick-1, 
                    opened_bits: state.opened_bits | 1<< ele_bit_index};
                max_rate = cmp::max( max_rate, 
                    ele_rate * (state.tick-1) as u32  + dfs2(new_state, rooms, cache, bit_count));
            }
        } else {
            // we both move
            let my_edges = rooms[&my_room_index].edges.clone();
            let ele_edges = rooms[&ele_room_index].edges.clone();
            for my_edge in &my_edges {
                for ele_edge in &ele_edges {
                    let new_state = State{my_location: *my_edge, ele_location: *ele_edge, tick: state.tick-1, 
                        opened_bits: state.opened_bits};
                    max_rate = cmp::max( max_rate, dfs2(new_state, rooms, cache, bit_count));
                }
            }
        }
    }

    cache.insert(state, max_rate);
    max_rate
}

/*
// calculate the optimal route
fn part1(rooms: HashMap<String,Room>, bit_count: u8) -> u32 {
    let state: State = State{location: rooms["AA"].room_index, tick: 30, opened_bits: 0};
    let mut cache: HashMap<State,u32> = HashMap::new();
    let max_flow = dfs("AA".to_string(), state, &rooms, &mut cache, bit_count);
    max_flow
}*/
// calculate the optimal route
fn part2(start_room: u8, rooms: HashMap<u8,Room>, bit_count: u8) -> u32 {
    let state: State = State{my_location: start_room, ele_location: start_room, tick: 26, opened_bits: 0};
    let mut cache: HashMap<State,u32> = HashMap::new();
    let max_flow = dfs2(state, &rooms, &mut cache, bit_count);
    max_flow
}


fn main() {
    const INPUT: &str = if IS_SAMPLE {
        include_str!("../sample.txt")
    } else {
        include_str!("../input16.txt")
    };

    let mut rooms: HashMap<u8,Room> = HashMap::new();

    // Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    let re: Regex = Regex::new(r".*([A-Z]{2})[^0-9]+([0-9]+).*valve[s]* ([A-Z ,]+)$").unwrap();
    let mut name_index: HashMap<String, u8> = HashMap::new();

    let mut room_index = 0;
    let mut bit_index: u8 = 0;
    for line in INPUT.lines() {
        let cap = re.captures(line);
        match cap {
            Some(capture) => {
                let room = capture[1].to_string();

                // have we seen this room name before?
                let this_room_index= if name_index.contains_key(&room) {
                    name_index[&room]
                } else {
                    let my_index = room_index;
                    name_index.insert(room.clone(), room_index);
                    room_index += 1;
                    my_index
                };

                let rate:u32 = capture[2].parse().unwrap();
                let edges_str = capture[3].to_string().replace(" ", "");
                //println!("es: {edges_str}");

                // translate all the edges to indices
                let edges: Vec<u8> = edges_str.split(",").map(|s| s.to_string()).map( |edge|
                    {
                        // if we've seen this name before return the string
                        if name_index.contains_key(&edge) {
                            name_index[&edge]
                        } else {
                            // make a new index and add it to the name_index map
                            let my_index = room_index;
                            name_index.insert(edge, room_index);
                            room_index += 1;
                            my_index
                        }
                    }).collect();
                //println!("{room} {:?}", edges);
                let this_bit_index = if rate == 0 {
                    0
                } else {
                    bit_index += 1;
                    if bit_index >= 16 {
                        panic!("too many rooms for bit field");
                    }
                    bit_index
                };

                rooms.insert(this_room_index, 
                    Room{bit_index: this_bit_index, 
                        room_index: this_room_index,
                        //name: room, 
                        rate: rate, 
                        edges: edges, 
                    });
            }
            _ => panic!("didn't match line"),
        }
    }
    //println!("read {} lines", rooms.len());

    // calculate the optimal route
    let p2_max = part2(name_index["AA"], rooms, bit_index);

    println!("total path {p2_max}");

}