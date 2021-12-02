// Advent of Code Day 2 Part 1 (12/02/21)
// Written in Rust!
// Author: Logan D.G. Smith

use advent_of_rust_2021::*;

fn main() {
    if let Ok(lines) = read_lines("inputs/day2input.txt") {
        // Declarations
        let mut horizonal_pos: i32 = 0;
        let mut vertical_pos: i32 = 0;

        // Consume Iterator
        for line in lines {
            match line {
                Ok(entry) => {
                    // Process token
                    let tokens: Vec<&str> = entry.split(" ").collect();
                    let distance: i32 = match tokens[1].parse::<i32>() {
                        Ok(parsed_int) => parsed_int,
                        Err(_) => {
                            println!("Failed to parse int");
                            return;
                        }
                    };

                    // Match direction
                    match tokens[0] {
                        "forward" => {
                            horizonal_pos += distance;
                        },
                        "up" => {
                            vertical_pos -= distance;
                        },
                        "down" => {
                            vertical_pos += distance;
                        },
                        _ => {
                            println!("Failed to parse!");
                            return;
                        }
                    }
                },
                Err(_) => {
                    println!("Failed to read line!");
                    return;
                }
            }
        }

        // Output
        valid_line(&format!("Horizontal Pos: {}", horizonal_pos));
        valid_line(&format!("Vertical Pos: {}", vertical_pos));
        valid_line(&format!("Final Answer: {}", horizonal_pos * vertical_pos));
    }
    else {
        println!("Failed to read file!");
        return;
    }
}