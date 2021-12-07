// Advent of Code Day 6 Part 1 (12/06/21)
// Written in Rust!
// Author: Logan D.G. Smith

use advent_of_rust_2021::*;

use std::collections::VecDeque;

// Simulate a spawn cycle
fn simulate_day(lifespans: &mut VecDeque<u32>) {
    // Spawners
    let zero_days = lifespans[0];

    // rotate all values than re-add spawners
    lifespans.rotate_left(1);
    lifespans[6] += zero_days;
}

fn main() {
    // Declarations
    let days: usize = 80;
    let mut lifespans: VecDeque<u32> = VecDeque::from(vec![0; 9]);

    // Process input
    let fish_ages: Vec<usize> = read_lines("inputs/day6input.txt")
        .expect("Failed to read file!") // Unwrap iter. 
        .next()                         // There should only be one line of input
        .expect("Nothing to read!")     // Unwrap Option. File might be empty
        .expect("Failed to read line!") // Unwrap Result. Catch iter errors here
        .split(',')
        .map(|age_string| age_string.parse::<usize>().expect("Failed to parse int!"))
        .collect();

    // Populate starting fish
    for age in fish_ages {
        lifespans[age] += 1;
    }

    // Run simpulation
    for _ in 0..days {
        simulate_day(&mut lifespans);
    }

    // Add all fish generated
    let mut total = 0;
    for fish in lifespans {
        total += fish;
    }

    // Print output
    println!("{}", total);
}