// Advent of Code Day 1 Part 1 (12/01/21)
// Written in Rust!
// Author: Logan D.G. Smith

use advent_of_rust_2021::*;

fn main() {
    // Declarations
    let mut prev:i32 = -1;
    let mut curr:i32 = -1;
    let mut inc_count:i32 = 0;

    // Obtain an iterator for the input file
    if let Ok(lines) = read_lines("inputs/day1input.txt") {
        // Consume the iterator (read each line of the file)
        for line in lines {
            if let Ok(entry) = line {
                // Parse input line
                curr = match entry.parse::<i32>() {
                    Ok(parsed_int) => parsed_int,
                    Err(_) => {
                        println!("Failed to parse line.");
                        return;
                    }
                };

                // No comparison needed if this is the first iteration
                print!("{} ", curr);
                if prev == -1 {
                    println!("(N/A - no previous measurement)");
                }
                // Increased
                else if curr > prev {
                    valid_line("(increased)");
                    inc_count += 1;
                }
                // Decreased
                else {
                    invalid_line("(decreased)");
                }

                // Move to next line
                prev = curr;
            }
        }

        // Final Output
        valid_line(&format!("{} increased lines", inc_count));
    }
    else {
        invalid_line("Couldn't open file!")
    }
}