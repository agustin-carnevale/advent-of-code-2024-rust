#![allow(warnings)]

fn main() {
    println!("Day 2 - Part 1");
    let input = include_str!("input.txt");
    let output = part1(input);

    dbg!(output);
}

/// Directions for searching
// #[derive(Debug)]
// enum Direction {
//     HorizontalRight,
//     HorizontalLeft,
//     VerticalDown,
//     VerticalUp,
//     DiagonalUpRight,
//     DiagonalUpLeft,
//     DiagonalDownRight,
//     DiagonalDownLeft,
// }

fn check_all_directions(grid: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
    let mut count = 0;
    let chars: Vec<char> = "MAS".chars().collect();
    let directions: Vec<(isize, isize)> = vec![
        (0, 1),
        (0, -1),
        (-1, 0),
        (-1, 1),
        (-1, -1),
        (1, 0),
        (1, 1),
        (1, -1),
    ];

    for dir in directions {
        let mut y = i as isize;
        let mut x = j as isize;
        for c in &chars {
            y += dir.0;
            x += dir.1;

            if (x < 0 || y < 0 || y >= grid.len() as isize || x >= grid[0].len() as isize) {
                break;
            }

            if (grid[y as usize][x as usize] != *c) {
                break;
            }

            if *c == 'S' {
                count += 1;
                println!("****count:{count}");
            }
        }
    }

    count
}

fn part1(input: &str) -> usize {
    let grid = parse(input);
    let n = grid.len();
    let m = grid[0].len();
    let mut count = 0;

    println!("n:{n} - m:{m}");
    println!("{:#?}", grid);

    for i in 0..n {
        println!("Iter i {i}");
        for j in 0..m {
            println!("\t\tIter j {j}");
            if (grid[i][j] == 'X') {
                count += check_all_directions(&grid, i, j);
            }
        }
    }

    count
}

fn parse(input: &str) -> Vec<Vec<char>> {
    // Parse the input into a 2D matrix
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let result = part1(input);
        assert_eq!(result, 18);
    }
}
