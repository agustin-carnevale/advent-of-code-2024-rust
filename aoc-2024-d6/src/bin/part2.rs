#![allow(warnings)]
use std::collections::{HashMap, HashSet};

use glam::IVec2;
use nom::{
    bytes::complete::tag,
    character::complete::{self, anychar, line_ending, one_of},
    multi::{fold_many1, many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult, Parser,
};
use nom_locate::{position, LocatedSpan};

type Span<'a> = LocatedSpan<&'a str>;

fn token(input: Span) -> IResult<Span, (IVec2, char)> {
    let y = input.location_line() as i32 - 1;
    let x = input.get_column() as i32 - 1;
    let (input, token) = one_of(".#^")(input)?;

    Ok((input, (IVec2::new(x, y), token)))
}

fn parse(input: Span) -> IResult<Span, ((IVec2, char), HashMap<IVec2, char>, usize, usize)> {
    let (input, items) = separated_list1(line_ending, many1(token))(input)?;

    let width = items[0].len();
    let height = items.len();

    let player = items
        .iter()
        .flatten()
        .find(|(pos, token)| token == &'^')
        .cloned()
        .expect("player not found");

    let walls = items
        .into_iter()
        .flatten()
        .filter(|(_pos, token)| token == &'#')
        .collect::<HashMap<IVec2, char>>();

    // dbg!(player, walls);

    Ok((input, (player, walls, width, height)))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }

    fn to_ivec2(&self) -> IVec2 {
        match self {
            Self::Up => IVec2::NEG_Y,
            Self::Down => IVec2::Y,
            Self::Left => IVec2::NEG_X,
            Self::Right => IVec2::X,
        }
    }
}

fn has_loop(
    new_wall: IVec2,
    mut walls: HashMap<IVec2, char>,
    width: usize,
    height: usize,
    player_initial_position: IVec2,
) -> bool {
    let mut direction = Direction::Up;
    let mut positions_visited: HashSet<(IVec2, Direction)> = HashSet::new();

    let mut player_position = player_initial_position;
    positions_visited.insert((player_position, direction));

    walls.insert(new_wall, '#');

    while (0..width as i32).contains(&player_position.x)
        && (0..height as i32).contains(&player_position.y)
    {
        let next_position = player_position + direction.to_ivec2();
        if walls.contains_key(&next_position) {
            direction = direction.turn_right();
        } else if positions_visited.contains(&(next_position, direction)) {
            // This means we have a loop because we are visiting a position we already
            // visited and in the same direction
            // dbg!(new_wall);
            return true;
        } else {
            // dbg!(player_position);
            player_position = next_position;
            positions_visited.insert((player_position, direction));
            // dbg!(player_position);
        }
    }

    false
}

fn part2(input: &str) -> u32 {
    let (_input, ((player_initial_position, _), walls, width, height)) =
        parse(Span::new(input)).expect("Upps");

    let mut direction = Direction::Up;
    let mut positions_visited: HashSet<IVec2> = HashSet::new();

    let mut player_position = player_initial_position;
    positions_visited.insert(player_position);

    while (0..width as i32).contains(&player_position.x)
        && (0..height as i32).contains(&player_position.y)
    {
        let next_position = player_position + direction.to_ivec2();
        if walls.contains_key(&next_position) {
            direction = direction.turn_right();
        } else {
            // dbg!(player_position);
            player_position = next_position;
            positions_visited.insert(player_position);
            // dbg!(player_position);
        }
    }

    // After calculating the positions visited by the player in part1
    // These are the only positions where adding a single extra wall could create a loop
    // For each of this positions we pass the initials walls + the new_wall to check for loops

    positions_visited.remove(&player_initial_position);

    positions_visited
        .iter()
        .filter(|pos| {
            has_loop(
                *pos.clone(),
                walls.clone(),
                width,
                height,
                player_initial_position,
            )
        })
        .count() as u32
}

fn main() {
    println!("Day 6 - Part 2");
    let input = include_str!("input.txt");
    let output = part2(input);

    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let result = part2(input);
        assert_eq!(result, 6);
    }
}
