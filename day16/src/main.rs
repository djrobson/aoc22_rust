use regex::Regex;
use std::cmp;
use std::collections::HashMap;
const IS_SAMPLE: bool = false;

//#[derive(Copy)]
struct Room {
    bit_index: u8,
    room_index: u8,
    name: String,
    rate: u32,
    edges: Vec<String>,
}
#[derive(Hash, PartialEq, Eq)]
struct State {
    location: u8,
    tick: u8,
    opened_bits: u16,
}

#[inline(always)]
fn is_room_open(room: &Room, state: &State) -> bool{
    (state.opened_bits & 1<<room.bit_index) != 0
}

fn dfs( pos: String, state: State, rooms: &HashMap<String,Room>, cache: &mut HashMap<State, u32>, bit_count: u8) -> u32 {
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
}

// calculate the optimal route
fn part1(rooms: HashMap<String,Room>, bit_count: u8) -> u32 {
    let state: State = State{location: rooms["AA"].room_index, tick: 30, opened_bits: 0};
    let mut cache: HashMap<State,u32> = HashMap::new();
    let max_flow = dfs("AA".to_string(), state, &rooms, &mut cache, bit_count);
    max_flow
}


fn main() {
    const INPUT: &str = if IS_SAMPLE {
        include_str!("../sample.txt")
    } else {
        include_str!("../input16.txt")
    };

    let mut rooms: HashMap<String,Room> = HashMap::new();

    // Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    let re: Regex = Regex::new(r".*([A-Z]{2})[^0-9]+([0-9]+).*valve[s]* ([A-Z ,]+)$").unwrap();

    let mut room_index = 0;
    let mut bit_index: u8 = 0;
    for line in INPUT.lines() {
        let cap = re.captures(line);
        match cap {
            Some(capture) => {
                let room = capture[1].to_string();
                let rate:u32 = capture[2].parse().unwrap();
                let edges_str = capture[3].to_string().replace(" ", "");
                //println!("es: {edges_str}");
                let edges: Vec<String> = edges_str.split(",").map(|s| s.to_string()).collect();
                //println!("{room} {:?}", edges);
                let this_index = if rate == 0 {
                    0
                } else {
                    bit_index += 1;
                    if bit_index >= 16 {
                        panic!("too many rooms for bit field");
                    }
                    bit_index
                };

                rooms.insert(room.clone(), 
                    Room{bit_index: this_index, 
                        room_index: room_index,
                        name: room, 
                        rate: rate, 
                        edges: edges, 
                        //opened:false, 
                        //tick:0
                    });
                room_index += 1;
            }
            _ => panic!("didn't match line"),
        }
    }
    println!("read {} lines", rooms.len());

    // calculate the optimal route
    let p1_max = part1(rooms, bit_index);

    /*
    let total:u32 = rooms.values().map(|r| {
        if r.opened {
            r.rate * (30-r.tick) as u32
        } else {
            0
        }
    }).sum();*/

    println!("total path {p1_max}");

}