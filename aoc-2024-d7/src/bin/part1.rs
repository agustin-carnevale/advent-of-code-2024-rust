#![allow(warnings)]
use core::num;
use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

use itertools::Itertools;

fn parse(input: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    let (input, equations) = separated_list1(
        line_ending,
        separated_pair(
            complete::u64,
            tag(": "),
            separated_list1(space1, complete::u64),
        ),
    )(input)?;

    Ok((input, equations))
}

// Recursive version
// fn check_possible_operation(result: u64, nums: Vec<u64>) -> bool {
//     if nums.len() == 0 {
//         return result == 0;
//     }
//     return (result >= nums[0] && check_possible_operation(result - nums[0], nums[1..].to_vec()))
//         || (result % nums[0] == 0
//             && check_possible_operation(result / nums[0], nums[1..].to_vec()));
// }

// Recursive version
// fn part1(input: &str) -> u64 {
//     let (_input, equations) = parse(input).expect("Upps");

//     equations
//         .iter()
//         .filter(|(result, nums)| {
//             let mut nums_reversed = nums.clone();
//             nums_reversed.reverse();
//             check_possible_operation(*result, nums_reversed)
//         })
//         .map(|(result, _nums)| *result)
//         .sum()
// }

// Multi Cartesian Product + Reduce version
const OPERATORS: [char; 2] = ['+', '*'];

fn part1(input: &str) -> u64 {
    let (_input, equations) = parse(input).expect("Upps");

    equations
        .iter()
        .filter_map(|(result, numbers)| {
            let operators_count = numbers.len() - 1;
            (0..operators_count)
                .map(|_| OPERATORS)
                .multi_cartesian_product()
                .any(|seq| {
                    let mut s = seq.iter();
                    // dbg!(&seq);
                    let operation_result = numbers
                        .iter()
                        .copied()
                        .reduce(|acc, next_number| match s.next() {
                            Some(&'+') => acc + next_number,
                            Some(&'*') => acc * next_number,
                            _ => unreachable!(),
                        })
                        .unwrap();
                    *result == operation_result
                })
                .then_some(result)
        })
        .sum()
}

fn main() {
    println!("Day 7 - Part 1");
    let input = include_str!("input.txt");
    let output = part1(input);

    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let result = part1(input);
        assert_eq!(result, 3749);
    }
}
