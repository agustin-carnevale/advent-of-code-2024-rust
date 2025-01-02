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

    left.sort();
    right.sort();

    assert!(left.len() == right.len());
    let lens = left.len();

    let mut diff = 0;

    for i in 0..lens {
        diff += (left[i] as i32 - right[i] as i32).abs() as u64;
    }

    println!("Total distance: {}", diff);
}
