const IS_SAMPLE: bool = true;

fn parse_input(input: &str) -> Vec<Vec<i8>> {
    let mut result: Vec<Vec<i8>> = Vec::new();
    for num in input.lines() {
        result.push(num.chars().map(|c| {
            match c {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => panic!("unexpected input"),
            }
        }).collect());
    }
    result
}

fn main() {
    const INPUT: &str = if IS_SAMPLE {
        include_str!("../sample.txt")
    } else {
        include_str!("../input25.txt")
    };
    let inputs =  parse_input(INPUT);
    let fuel_requirements: Vec<usize> = inputs.iter().map(|line| {
        line.iter().rev().enumerate().map(|(position, quintits)| {
            let pos = 5^position;
            pos * *quintits as usize
        }).sum::<usize>()
    }).collect();
    let total:usize = fuel_requirements.iter().sum();
    println!("{total}");
}
