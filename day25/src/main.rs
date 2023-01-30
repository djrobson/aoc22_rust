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

fn decimal_to_snafu(decimal: isize) -> String {
    let mut nums: Vec<i8> = Vec::new();

    let mut num = decimal;
    let mut borrow = 0;
    while num != 0 {
        let mut remainder = num%5;
        remainder +=  borrow;
        num = num /5;
        if remainder >= 3 {
            remainder -=5;
            borrow = 1;
        } else {
            borrow = 0;
        }
        nums.push(remainder as i8);
    }

    let result: String = nums.iter().rev().map(|n| {
        match n {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => panic!("unexpected conversion"),
        }
    }).collect();
    result
}

fn main() {
    const INPUT: &str = if IS_SAMPLE {
        include_str!("../sample.txt")
    } else {
        include_str!("../input25.txt")
    };
    let inputs =  parse_input(INPUT);
    let fuel_requirements: Vec<isize> = inputs.iter().map(|line| {
        line.iter().rev().enumerate().map(|(position, quint)| {
            let pos = 5_isize.pow(position as u32);
            pos as isize * *quint as isize
        }).sum::<isize>()
    }).collect();
    let total:isize = fuel_requirements.iter().sum();
    println!("{total}");
    println!("{}", decimal_to_snafu(total));
}
