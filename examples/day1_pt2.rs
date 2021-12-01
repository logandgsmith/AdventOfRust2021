// Advent of Code Day 1 Part 2 (12/01/21)
// Written in Rust!
// Author: Logan D.G. Smith

use advent_of_rust_2021::*;

// Valid windows will always have a positive depth
fn is_valid_window(window: [i32; 3]) -> bool {
    for measurement in window.iter() {
        if measurement < &0 { return false; }
        else { return true; }
    }
    false // something went wrong
}

// Sum all of the measurements in the window
fn sum_window(window: [i32; 3]) -> i32 {
    let mut sum = 0;
    for measurement in window.iter() {
        sum += measurement;
    }
    sum
}

// Slide the values back in the window
fn slide_window(window: &mut [i32; 3], new_measurement: i32) {
    window[0] = window[1];
    window[1] = window[2];
    window[2] = new_measurement;
}

fn main() {
    // Declarations
    let mut prev_window: [i32; 3] = [-1; 3];
    let mut curr_window: [i32; 3] = [-1; 3];
    let mut inc_count:i32 = 0;

    // Obtain an iterator for the input file
    if let Ok(lines) = read_lines("inputs/day1input.txt") {
        // Consume the iterator (read each line of the file)
        for line in lines {
            if let Ok(entry) = line {
                // Parse input line
                let curr = match entry.parse::<i32>() {
                    Ok(parsed_int) => parsed_int,
                    Err(_) => {
                        println!("Failed to parse line.");
                        return;
                    }
                };

                // Slide the window
                slide_window(&mut curr_window, curr);
                let prev_sum = sum_window(prev_window);
                let curr_sum = sum_window(curr_window);

                // No comparison needed if this is the first iteration
                if !is_valid_window(prev_window) || !is_valid_window(curr_window) {
                    println!("Still scanning...");
                    println!("{:?}", prev_window);
                    println!("{:?}", curr_window);
                }
                // Increased
                else if curr_sum > prev_sum {
                    print!("{}: ", curr_sum);
                    valid_line("(increased)");
                    inc_count += 1;
                }
                // Decreased
                else if curr_sum < prev_sum {
                    print!("{}: ", curr_sum);
                    invalid_line("(decreased)");
                }
                // No change from last window
                else {
                    print!("{}: ", curr_sum);
                    invalid_line("no change");
                }

                // Move to next line
                prev_window = curr_window;
            }
        }

        // Final Output
        valid_line(&format!("{} increased lines", inc_count));
    }
    else {
        invalid_line("Couldn't open file!")
    }
}