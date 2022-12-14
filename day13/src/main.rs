use nom::{
    branch::alt,
    bytes::complete::tag,
    multi::{separated_list0,separated_list1},
    character::complete::newline,
    sequence::{delimited,separated_pair},
    IResult,
    Parser,
};

use std::{
    //cmp::Ordering::{self, *},
    fmt::Display,
};

#[derive(Debug)]
enum Value {
    Val(u8),
    List(Vec<Value>),
}

/*
impl Display for Value {
    fn fmt( &self, f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::List(list) => format!(
                    "[{}]",
                    list.iter()
                        .map(|v| v.to_string())
                        .intersperse(",".to_string())
                        .collect::<String>()
                ),
                Value::Val(num) => num.to_string(),
            },
        )
    }
}*/

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
    const IS_SAMPLE: bool = true;
    const INPUT: &str = if IS_SAMPLE {
        include_str!("../sample.txt")
    } else {
        include_str!("../input13.txt")
    };

    let input = parse_input(INPUT).unwrap().1;
    println!("stuff here {:?}", input);
}
