
use std::collections::VecDeque;
use std::collections::HashMap;

#[ derive(Clone)]
struct Monkey {
    id: usize,
    items: VecDeque<usize>,
    op: fn(usize) -> usize,
    pred: fn(usize) -> usize,
}

fn main() -> () {

    let do_sample = false;

    let mut monkeys: Vec<Monkey> = Vec::new();
    let step2 = true;
    let round_count = if step2 {10000} else {20};
    let divisor = if do_sample {96_577} else {9_699_690};
    if do_sample {
        monkeys.push( 
            Monkey{
                id:0,
                items: VecDeque::from(vec!(79,98)),
                op: |x| x*19,
                pred: |x| if 0==x%23 {2}else{3}
            }
        );
        monkeys.push( 
            Monkey{
                id:1,
                items: VecDeque::from(vec!(54,65,75,74)),
                op: |x| x+6,
                pred: |x| if 0==x%19 {2}else{0},
            }
        );
        monkeys.push( 
            Monkey{
                id:2,
                items: VecDeque::from(vec!(79,60,97)),
                op: |x| x*x,
                pred: |x| if 0==x%13 {1}else{3},
            }
        );
        monkeys.push( 
            Monkey{
                id:3,
                items: VecDeque::from(vec!(74)),
                op: |x| x+3,
                pred: |x| if 0==x%17 {0}else{1},
            }
        );
    } else {
        monkeys.push( 
            Monkey{
                id:0,
                items: VecDeque::from(vec!(57)),
                op: |x| x*13,
                pred: |x| if 0==x%11 {3}else{2}
            }
        );
        monkeys.push( 
            Monkey{
                id:1,
                items: VecDeque::from(vec!(58, 93, 88, 81, 72, 73, 65)),
                op: |x| x+2,
                pred: |x| if 0==x%7 {6}else{7}
            }
        );
        monkeys.push( 
            Monkey{
                id:2,
                items: VecDeque::from(vec!(65,95)),
                op: |x| x+6,
                pred: |x| if 0==x%13 {3}else{5}
            }
        );
        monkeys.push( 
            Monkey{
                id:3,
                items: VecDeque::from(vec!(58,80,81,83)),
                op: |x| x*x,
                pred: |x| if 0==x%5 {4}else{5}
            }
        );
        monkeys.push( 
            Monkey{
                id:4,
                items: VecDeque::from(vec!(58,89,90,96,55)),
                op: |x| x+3,
                pred: |x| if 0==x%3 {1}else{7}
            }
        );
        monkeys.push( 
            Monkey{
                id:5,
                items: VecDeque::from(vec!(66,73,87,58,62,67)),
                op: |x| x*7,
                pred: |x| if 0==x%17 {4}else{1}
            }
        );
        monkeys.push( 
            Monkey{
                id:6,
                items: VecDeque::from(vec!(85,55,89)),
                op: |x| x+4,
                pred: |x| if 0==x%2 {2}else{0}
            }
        );
        monkeys.push( 
            Monkey{
                id:7,
                items: VecDeque::from(vec!(73,80,54,94,90,52,69,58)),
                op: |x| x+7,
                pred: |x| if 0==x%19 {6}else{0}
            }
        )
    }

    //let item_count: usize = monkeys.iter().map(|m| m.items.len()).sum();
    let mut m_count = vec![0;monkeys.len()];
    //let mut target_list: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut target_list: Vec<Vec<usize>> = vec![vec![];monkeys.len()];

    for round in 1..=round_count {
        for m in &mut monkeys {
            // gather any previous tosses
            m.items.extend(target_list[m.id].clone());
            // clear the list for this monkey
            target_list[m.id] = vec![];

            // process my items
            while let Some(item) = m.items.pop_front() {
                m_count[m.id] += 1;
                let mut worry = (m.op)(item) % divisor;
                if !step2 {
                    worry = worry / 3;
                }
                target_list[(m.pred)(worry)].push(worry);
            }
        }
        /*
        println!("After round {round}:");
        for idx in 0..monkeys.len() {
            print!("Monkey {idx}: ");
            for item in &monkeys[idx].items {
                print!("{item}, ");
            }
            for item in &target_list[idx] {
                print!("{item}, ");
            }
            print!("\n");
        }*/
        if 0 == round % 1000 || round == 20 || round == 1{
            println!("after round {round} {:?}", m_count);
        }
    }

    println!("{:?}", m_count);
    m_count.sort();
    println!("monkey business {}", m_count[m_count.len()-1] as usize * m_count[m_count.len()-2] as usize);

    ()
}