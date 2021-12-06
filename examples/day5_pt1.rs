// Advent of Code Day 5 Part 1 (12/05/21)
// Written in Rust!
// Author: Logan D.G. Smith
// NOTE: Itertools is a really cool crate that I need to look into more. There's also probably a better way to implement agnostic_range()

// Advent of Code Library Functions
use advent_of_rust_2021::*;

// Packages
use regex::Regex;
use itertools::Itertools;

// Standard Library
use std::collections::HashMap;

// Create range from small -> large (inclusive), unlike the `..` operator...
fn agnostic_range(start: i32, finish: i32) -> impl Iterator<Item = i32> {
    if start < finish {
        return start..finish + 1;
    }
    else {
        return finish..start + 1;
    }
}

fn main() {
    // Regex
    let split_regex = regex::Regex::new(r"\s->\s|,").expect("Failed to create regex!");

    //  k: (x, y), v: lines touching the point
    let mut intersecting_points: HashMap<(i32,i32), i32> = HashMap::new();      

    // Read file
    let lines = read_lines("inputs/day5input.txt").expect("Failed to read lines!");

    // Process lines
    for line in lines {
        let reading = line.expect("Failed to read line!");
        let mut coords_iter = split_regex
            .split(&reading)
            .map(|s| s.parse::<i32>().expect("Failed to parse int!"))
            .tuples::<(i32,i32)>(); // itertools version of a "tuple collect"

        // Consume iterator
        let start = coords_iter.next().expect("Failed to read line start!");
        let end = coords_iter.next().expect("Failed to read line end!");

        // Debug points
        //println!("{:?} -> {:?}", start, end);

        // Vertical lines
        if start.0 == end.0 {
            for y in agnostic_range(start.1, end.1) {
                let count = intersecting_points.entry((start.0, y)).or_insert(0);
                *count += 1;
            }
            // println!("ver: {}\n", test_count);
        }
        // Horizontal lines
        else if start.1 == end.1 {
            for x in agnostic_range(start.0, end.0) {
                let count = intersecting_points.entry((x, start.1)).or_insert(0);
                *count += 1;
            }
            // println!("hor: {}\n", test_count);
        }
    }

    // Final count of vents
    let mut count = 0;
    for value in intersecting_points.values() {
        if *value > 1 {
            count += 1
        }
    }

    println!("Points with multiple vents: {}", count);
}