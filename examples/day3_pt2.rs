// Advent of Code Day 3 Part 2 (12/06/21)
// Written in Rust!
// Author: Logan D.G. Smith

use advent_of_rust_2021::*;

// Winnow the group based on the position
fn calculate_bit_criteria(candidates: Vec<String>, position: usize, favored_bit: char) -> Vec<String> {
    // Break if length of 1
    if candidates.len() == 1 {
        return candidates;
    }

    // Declarations
    let mut favored: Vec<String> = Vec::new();
    let mut unfavored: Vec<String> = Vec::new();

    // Sort the candidates
    for candidate in candidates {
        if candidate.chars().nth(position).expect("Failed to parse char!") == favored_bit {
            favored.push(candidate);
        }
        else {
            unfavored.push(candidate);
        }
    }

    // Oxy should be most common
    if favored_bit == '1' && favored.len() >= unfavored.len() {
        calculate_bit_criteria(favored, position + 1, favored_bit)
    }
    // CO2 should be least common
    else if favored_bit == '0' && favored.len() <= unfavored.len() {
        calculate_bit_criteria(favored, position + 1, favored_bit)
    }
    else  {
        calculate_bit_criteria(unfavored, position + 1, favored_bit)
    }
    
}

fn main() {
    // Read file and unwrap lines
    let input_lines: Vec<String> = read_lines("inputs/day3input.txt")
        .expect("Failed to read file!")
        .map(|l| l.expect("Failed to read line!"))
        .collect();  

    // Clone the groups to winnow seperately 
    let oxy_group: Vec<String> = input_lines.clone();
    let co2_group: Vec<String> = input_lines.clone();

    // Winnow the groups
    let oxy_winnowed: Vec<String> = calculate_bit_criteria(oxy_group, 0, '1');
    let co2_winnowed: Vec<String> = calculate_bit_criteria(co2_group, 0, '0');

    // Parse Rates from binary strings
    let oxy_rate: isize = isize::from_str_radix(&oxy_winnowed[0], 2).expect("Failed to parse oxy binary!");
    let co2_rate: isize = isize::from_str_radix(&co2_winnowed[0], 2).expect("Failed to parse co2 binary!");

    // Print the output
    println!("Oxygen Generator Rating: {}", oxy_rate);
    println!("CO2 Scrubber Rating: {}", co2_rate);
    println!("Life Support Rating: {}", oxy_rate * co2_rate);
}