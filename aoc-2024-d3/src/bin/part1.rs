#![allow(warnings)]
use std::os::unix::process::parent_id;

use nom::{
    bytes::complete::tag,
    character::complete::{self, anychar},
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

fn main() {
    println!("Day 2 - Part 1");
    let input = include_str!("input.txt");
    let output = part1(input);

    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let (_input, instructions) = parse(input).unwrap();
    // dbg!(&instructions);

    // let sum = instructions
    //     .iter()
    //     .map(|ins| match ins {
    //         Instruction::Mul(a, b) => a * b,
    //     })
    //     .sum();

    let sum = instructions.iter().fold(0, |acc, ins| match ins {
        Instruction::Mul(a, b) => acc + a * b,
    });

    println!("Sum: {sum}");

    sum
}
#[derive(Debug)]
enum Instruction {
    Mul(u32, u32),
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;

    Ok((input, Instruction::Mul(pair.0, pair.1)))
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, instruction).map(|(_discard, ins)| ins))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = part1(input);
        assert_eq!(result, 161);
    }
}
