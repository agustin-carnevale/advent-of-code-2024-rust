#![allow(warnings)]
use std::os::unix::process::parent_id;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

fn main() {
    println!("Day 2 - Part 2");
    let input = include_str!("input.txt");
    let output = part2(input);

    dbg!(output);
}

fn part2(input: &str) -> u32 {
    let (_input, instructions) = parse(input).unwrap();
    dbg!(&instructions);

    let (_, sum) = instructions
        .iter()
        .fold((Instruction::Do, 0), |(state, acc), ins| match ins {
            Instruction::Mul(a, b) => {
                if (state == Instruction::Do) {
                    (state, acc + a * b)
                } else {
                    (state, acc)
                }
            }
            Instruction::Do => (Instruction::Do, acc),
            Instruction::Dont => (Instruction::Dont, acc),
        });

    println!("Sum: {sum}");

    sum
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

fn mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;

    Ok((input, Instruction::Mul(pair.0, pair.1)))
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Dont, tag("don't()")),
        value(Instruction::Do, tag("do()")),
        mul,
    ))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, instruction).map(|(_discard, ins)| ins))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = part2(input);
        assert_eq!(result, 48);
    }
}
