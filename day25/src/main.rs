const IS_SAMPLE: bool = false;

fn parse_input(input: &str) -> Vec<Vec<isize>> {
    let mut result: Vec<Vec<isize>> = Vec::new();
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

fn convert_to_snafu(input: isize) -> String {
    
}

fn main() {
    const INPUT: &str = if IS_SAMPLE {
        include_str!("../sample.txt")
    } else {
        include_str!("../input25.txt")
    };
    let inputs =  parse_input(INPUT);
    let fuel_requirements: Vec<isize> = inputs.iter().map(|line| {
        line.iter().rev().enumerate().map(|(position, quintits)| {
            let pos = 5_isize.pow(position as u32);
            pos as isize * *quintits as isize
        }).sum::<isize>()
    }).collect();
    let total:isize = fuel_requirements.iter().sum();
    println!("{total}");
}
