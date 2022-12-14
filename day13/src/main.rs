use nom::{
    branch::alt,
    bytes::complete::tag,
    multi::{separated_list0,separated_list1},
    sequence::{delimited,separated_pair},
    IResult,
    Parser,
};

use std::{
    cmp::Ordering::{self, *},
};

#[derive(Debug)]
enum Value {
    Val(u8),
    List(Vec<Value>),
}

impl Eq for Value {

}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Val(l0), Self::Val(r0))  => l0 == r0,
            (Self::List(l0), Self::Val(r0)) => {
                l0 == &vec![Value::Val(*r0)]
            }
            (Self::Val(l0), Self::List(r0)) => {
                &vec![Value::Val(*l0)] == r0
            }
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::List(a), Value::List(b)) => a.cmp(b),
            (Value::List(a), Value::Val(b)) => a.cmp(&vec![Value::Val(*b)]),
            (Value::Val(a),Value::List(b)) => vec![Value::Val(*a)].cmp(&b),
            (Value::Val(a), Value::Val(b)) => a.cmp(b)
        }
    }
}

fn parse_packet(input: &str) -> IResult<&str, Value> {
    alt((
        delimited(
            tag("["),
            separated_list0(tag(","), parse_packet),
            tag("]")
        )
        .map(|vec| Value::List(vec) ),
        nom::character::complete::u8
            .map(|num| Value::Val(num))    
        ))(input)
}

fn parse_input(input: &str) ->IResult<&str, Vec<(Value,Value)>>
{    
    separated_list1(
    tag("\r\n\r\n"),
      separated_pair(parse_packet, tag("\r\n"), parse_packet).map(
            |(p1,p2)| (p1,p2)
    ))(input)
}

fn main() {
    const IS_SAMPLE: bool = false;
    const INPUT: &str = if IS_SAMPLE {
        include_str!("../sample.txt")
    } else {
        include_str!("../input13.txt")
    };

    let input = parse_input(INPUT).unwrap().1;
    let mut count = 1;
    let mut total = 0;
    for pair in input {
        let (left, right) = pair;
        if left < right {
            total += count;
        }
        count += 1;
    }
    println!("step 1 {total}");}
