// Advent of Code Day 7 Part 2 (12/07/21)
// Written in Rust!
// Author: Logan D.G. Smith
// NOTE: Part 1 was a lot more efficient than this one. I couldn't think of a formula to apply to I fell back on brute forcing it.

use advent_of_rust_2021::*;

fn calculate_fuel(positions: &Vec<i32>, target: i32) -> i32 {
    positions
        .iter()
        .map(|pos| Iterator::sum::<i32>(0..=(pos - target).abs()))
        .reduce(|a, b| a + b)
        .expect("Failed to calculate lowest fuel")
}

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

    let mut fuel_consumption: Vec<i32> = Vec::new();

    for target in *positions.first().expect("Couldn't read first!")..=*positions.last().expect("Couldn't read last!") {
        fuel_consumption.push(calculate_fuel(&positions, target))
    }

    fuel_consumption.sort();

    println!("{}", fuel_consumption[0]);
    
}