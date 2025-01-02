fn main() {
    println!("Day 2 - Part 1");
    let input = include_str!("input.txt");
    let output = part1(input);

    dbg!(output);
}

fn is_safe(levels: &Vec<u32>) -> bool {
    let first_level = levels[0];
    let second_level = levels[1];
    let mut ascending = true;

    if first_level == second_level || ((first_level as i32) - (second_level as i32)).abs() > 3 {
        return false;
    }

    if first_level > second_level {
        ascending = false;
    }

    if ascending {
        // Ascending
        for i in 1..levels.len() - 1 {
            if (levels[i] >= levels[i + 1]) {
                return false;
            }
            if (levels[i + 1] - levels[i] > 3) {
                return false;
            }
        }
    } else {
        // Descending
        for i in 1..levels.len() - 1 {
            if (levels[i] <= levels[i + 1]) {
                return false;
            }
            if (levels[i] - levels[i + 1] > 3) {
                return false;
            }
        }
    }

    true
}

// fn part1(input: &str) -> u32 {
//     let mut safe_levels = 0;

//     for line in input.lines() {
//         let levels: Vec<u32> = line
//             .split_whitespace()
//             .map(|n| n.parse::<u32>().unwrap())
//             .collect();

//         if (is_safe(&levels)) {
//             safe_levels += 1;
//             // println!("{:?}", levels);
//         }
//     }

//     safe_levels
// }

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect()
        })
        .filter(|report| is_safe(&report))
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9";
        let result = part1(input);
        assert_eq!(result, 2);
    }
}
