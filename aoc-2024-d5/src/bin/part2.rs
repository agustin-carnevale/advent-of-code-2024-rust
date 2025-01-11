#![allow(warnings)]
use nom::{
    bytes::complete::tag,
    character::complete::{self, anychar, line_ending},
    multi::{fold_many1, many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult, Parser,
};
use std::{cmp::Ordering, collections::HashMap, hash::Hash};

// fn is_correctly_ordered(rules: &HashMap<u32, Vec<u32>>, update: &Vec<u32>) -> bool {
//     for (i, current) in update.iter().rev().enumerate() {
//         if let Some(restricted) = rules.get(current) {
//             for &next in update.iter().rev().skip(i + 1) {
//                 if restricted.contains(&next) {
//                     return false; // Found a disallowed element appearing after `current`
//                 }
//             }
//         }
//     }
//     true // All rules are respected
// }

fn is_correctly_ordered(rules: &HashMap<u32, Vec<u32>>, update: &Vec<u32>) -> bool {
    update.is_sorted_by(|a, b| {
        if rules.get(a).is_some_and(|pages| pages.contains(b)) || rules.get(a).is_none() {
            true
        } else {
            false
        }
    })
}

fn get_middle_value(update: Vec<u32>) -> u32 {
    let mid_index = update.len() / 2;
    update[mid_index]
}

fn rules(input: &str) -> IResult<&str, HashMap<u32, Vec<u32>>> {
    fold_many1(
        terminated(
            separated_pair(complete::u32, tag("|"), complete::u32),
            line_ending,
        ),
        HashMap::default,
        |mut acc: HashMap<u32, Vec<u32>>, (page, after)| {
            acc.entry(page)
                .and_modify(|afters| {
                    afters.push(after);
                })
                .or_insert(vec![after]);
            acc
        },
    )(input)
}

fn updates(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(line_ending, separated_list1(tag(","), complete::u32))(input)
}

fn parse(input: &str) -> IResult<&str, (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>)> {
    let (input, parsed_rules) = terminated(rules, line_ending)(input)?;
    // dbg!(parsed_rules);
    let (input, parsed_updates) = updates(input)?;
    // dbg!(parsed_updates);

    Ok((input, (parsed_rules, parsed_updates)))
}

fn part2(input: &str) -> u32 {
    let (_input, (rules, updates)) = parse(input).expect("Upps");

    updates
        .iter()
        .filter(|update| !is_correctly_ordered(&rules, *update))
        .map(|update| {
            let mut update_clone = update.clone();
            update_clone.sort_by(|a, b| {
                if rules.get(a).is_some_and(|pages| pages.contains(b)) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            update_clone
        })
        .map(|update_sorted| get_middle_value(update_sorted))
        .sum()
}

fn main() {
    println!("Day 5 - Part 2");
    let input = include_str!("input.txt");
    let output = part2(input);

    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let result = part2(input);
        assert_eq!(result, 123);
    }
}
