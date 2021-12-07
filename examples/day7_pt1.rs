// Advent of Code Day 7 Part 1 (12/07/21)
// Written in Rust!
// Author: Logan D.G. Smith

use advent_of_rust_2021::*;

fn main() {
    // Process input
    let mut positions: Vec<i32> = read_lines("inputs/day7input.txt")
        .expect("Failed to read file!") // Unwrap iter. 
        .next()                         // There should only be one line of input
        .expect("Nothing to read!")     // Unwrap Option. File might be empty
        .expect("Failed to read line!") // Unwrap Result. Catch iter errors here
        .split(',')
        .map(|pos_string| pos_string.parse::<i32>().expect("Failed to parse int!"))
        .collect();
    positions.sort();

    // Target the midpoint of all positions
    let target = positions[positions.len() / 2];

    // Determine and print the total fuel to reach target
    println!("{}", 
        positions
            .iter()
            .map(|pos| (pos - target).abs())
            .reduce(|a, b| a + b)
            .expect("Failed to reduce!")
    );
}