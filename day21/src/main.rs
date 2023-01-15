use regex::Regex;
use std::collections::HashMap;
const IS_SAMPLE: bool = false;

#[derive(Clone)]
enum Val {
    Literal(isize),
    Operation(String,Op,String),
}

#[derive(Clone,Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}

static mut humn_max:isize  = 245167969204648;
static mut humn:isize  = 0;
static mut humn_min:isize  = -245167969204648;

fn calc_value(monkeys: &mut HashMap<String,Val>, name: String) -> isize {
    let my_val;
    if name.eq("humn") {
        unsafe {
            return humn;
        }
    }
    if let Some(val) = monkeys.get(&name) {
        my_val = val.clone();
    } else {
        panic!("bad monkey name {name}");
    } 

    match my_val {
        Val::Literal(num) => num,
        Val::Operation(first, Op::Add, second) => calc_value(monkeys, first.clone()) + calc_value(monkeys, second.clone()),
        Val::Operation(first, Op::Sub, second) => calc_value(monkeys, first.clone()) - calc_value(monkeys, second.clone()),
        Val::Operation(first, Op::Mul, second) => calc_value(monkeys, first.clone()) * calc_value(monkeys, second.clone()),
        Val::Operation(first, Op::Div, second) => calc_value(monkeys, first.clone()) / calc_value(monkeys, second.clone()),
        Val::Operation(first, Op::Eq, second) => {
            let mut left = calc_value(monkeys, first.clone());
            let right = calc_value(monkeys, second.clone());
            unsafe {
                while (left != right) {
                    
                    if left > right {
                        humn_min = humn;
                        humn = humn + ((humn_max - humn) /2);
                    } else {
                        humn_max = humn;
                        humn = (humn_min + humn) /2;
                    }
                    
                    left = calc_value(monkeys, first.clone());
                    println!("humn {humn_min}<={humn}<={humn_max}, {left} {right}");
                }
            }
            0
        },
    }
}
fn main() {
    const INPUT: &str = if IS_SAMPLE {
        include_str!("../sample.txt")
    } else {
        include_str!("../input21.txt")
    };

    let mut monkeys: HashMap<String, Val> = HashMap::new();

    let num: Regex = Regex::new(r"([a-z]{4}): (\d+)").unwrap();
    let math: Regex = Regex::new(r"([a-z]{4}): ([a-z]{4}) ([\+\-/\*]{1}) ([a-z]{4})").unwrap();
    for line in INPUT.lines() {
        if let Some(capture) = num.captures(line) {
            monkeys.insert(capture[1].to_string(), 
                Val::Literal(capture[2].parse::<isize>().unwrap()));
            
        } else if let Some(capture) = math.captures(line) {
            let op = match &capture[3] {
                "+" => Op::Add,
                "-" => Op::Sub,
                "/" => Op::Div,
                "*" => Op::Mul,
                _ => panic!("unexpected operation"),
            };
            monkeys.insert(capture[1].to_string(), 
                Val::Operation(capture[2].to_string(),op,capture[4].to_string()));
        } else {
            panic!("unexpected line");
        }
    }

    if let Val::Operation(first, _op, second) = monkeys.get("root").unwrap() {
        let new_op = Val::Operation( first.clone(), Op::Eq, second.clone());
        monkeys.insert("root".to_string(), new_op);
    } else {
        panic!("didnt find root monkey");
    }

    // recurse all the calculations
    let top_node: String = "root".to_string();
    let p2_total: isize = calc_value(&mut monkeys, top_node);

    println!("part2: {p2_total}");
}