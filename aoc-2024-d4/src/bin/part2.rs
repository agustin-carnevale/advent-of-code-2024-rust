#![allow(warnings)]

fn main() {
    println!("Day 2 - Part 2");
    let input = include_str!("input.txt");
    let output = part2(input);

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

fn check_diagonal(grid: &Vec<Vec<char>>, i: usize, j: usize, dir: (isize, isize)) -> bool {
    let mut y = i as isize + dir.0;
    let mut x = j as isize + dir.1;

    if (x < 0 || y < 0 || y >= grid.len() as isize || x >= grid[0].len() as isize) {
        return false;
    }

    if (grid[y as usize][x as usize] == 'M') {
        y = i as isize - dir.0;
        x = j as isize - dir.1;
        if (x < 0 || y < 0 || y >= grid.len() as isize || x >= grid[0].len() as isize) {
            return false;
        }
        if (grid[y as usize][x as usize] != 'S') {
            return false;
        }
    } else if (grid[y as usize][x as usize] == 'S') {
        y = i as isize - dir.0;
        x = j as isize - dir.1;
        if (x < 0 || y < 0 || y >= grid.len() as isize || x >= grid[0].len() as isize) {
            return false;
        }
        if (grid[y as usize][x as usize] != 'M') {
            return false;
        }
    } else {
        return false;
    }

    true
}

fn check_x_mas(grid: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
    let mut count = 0;
    let directions: Vec<(isize, isize)> = vec![(-1, -1), (1, -1)];

    for dir in directions {
        if !check_diagonal(grid, i, j, dir) {
            return 0;
        }
    }
    1
}

fn part2(input: &str) -> usize {
    let grid = parse(input);
    let n = grid.len();
    let m = grid[0].len();
    let mut count = 0;

    for i in 0..n {
        for j in 0..m {
            if (grid[i][j] == 'A') {
                count += check_x_mas(&grid, i, j);
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
    fn test_part2() {
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
        let result = part2(input);
        assert_eq!(result, 9);
    }
}
