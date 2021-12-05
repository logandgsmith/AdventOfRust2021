// Advent of Code Day 3 Part 1 (12/03/21)
// Written in Rust!
// Author: Logan D.G. Smith

use advent_of_rust_2021::*;

fn main() {

    // Collecting all the strings to make calculations easier
    let lines: Vec<Result<String, std::io::Error>> = read_lines("inputs/day3input.txt")
        .expect("Failed to read file!")
        .collect();

    // Setup counts based on input from file
    let line_count = lines.len(); 
    let digit_count = lines[0].as_ref().expect("Failed to read line!").len();
    let mut ones_counts: Vec<usize> = vec![0; digit_count];

    // Analyze the strings
    for line in lines {
        // Convert string to char vec
        let reading: Vec<char> = line
            .expect("Failed to read line!") // Attempt to unwrap the result types
            .chars()
            .collect();
       
        // Count 1s
        for (index, digit) in reading.iter().enumerate() {
            if *digit == '1' {
                ones_counts[index] += 1;
            }
        }
    }
    
    // Declare Binary Strings
    let mut gamma_binary = String::new();
    let mut epsilon_binary = String::new();

    // Construct gamma_binary
    for count in ones_counts {
        if count > line_count / 2 {
            gamma_binary += "1";
            epsilon_binary += "0";
        }
        else {
            gamma_binary += "0";
            epsilon_binary += "1";
        }
    }

    // Convert binary string to decimal
    let gamma_rate = isize::from_str_radix(&gamma_binary, 2).expect("Failed to parse gamma binary!");
    let epsilon_rate = isize::from_str_radix(&epsilon_binary, 2).expect("Failed to parse epsilon binary!");

    // Print Calculation
    println!("Gamma Rate: {}", gamma_rate);
    println!("Epsilon Rate: {}", epsilon_rate);
    println!("Power Consumption: {}", gamma_rate * epsilon_rate);
}