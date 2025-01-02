use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // Path to the input file
    let path = "input.txt";

    // Open the file for reading
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();

        let left_num = parts[0].parse::<i32>().unwrap();
        let right_num = parts[1].parse::<i32>().unwrap();

        left.push(left_num as u32);
        right.push(right_num as u32);
    }

    let mut right_map: HashMap<u32, usize> = HashMap::new();

    for x in right {
        *right_map.entry(x).or_default() += 1;
    }

    // println!("{:#?}", right_map);

    let mut score = 0;

    for y in left {
        if let Some(c) = right_map.get_mut(&y) {
            score += y * (*c as u32);
        }
    }

    println!("Total score: {}", score);
}
