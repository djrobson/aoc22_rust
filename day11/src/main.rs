
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

    let do_sample = true;

    let mut monkeys: Vec<Monkey> = Vec::new();
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

    let item_count: usize = monkeys.iter().map(|m| m.items.len()).sum();
    let mut m_count = vec![0;monkeys.len()];
    let mut target_list: HashMap<usize, Vec<usize>> = HashMap::new();
    for _ in 0..20 {
        //assert!(m_count.iter().sum::<usize>() == c*item_count);
        for m in &mut monkeys {
            // gather any previous tosses
            m.items.extend(target_list.get(&m.id).unwrap_or(&vec![]));
            target_list.insert(m.id, vec![]);

            while let Some(item) = m.items.pop_front() {
                let mut worry = (m.op)(item);
                worry = worry/3;
                m_count[(m.pred)(worry)] += 1;
                target_list.entry((m.pred)(worry)).or_insert(vec![]).push(worry);
            }
        }
    }

    m_count.sort();

    println!("monkey business {}", m_count[m_count.len()-1] * m_count[m_count.len()-2]);

    ()
}