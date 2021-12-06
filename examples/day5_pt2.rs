// Advent of Code Day 5 Part 2 (12/05/21)
// Written in Rust!
// Author: Logan D.G. Smith
// NOTE: Pretty fun update -- I wish I had used Enums for the first part as well. Definitely could be cleaner/more functional.

// Advent of Code Library Functions
use advent_of_rust_2021::*;

// Packages
use itertools::Itertools;

// Standard Library
use std::collections::HashMap;

// Examining the slant moving from the left to the right
enum Slant {
    Vertical,   // Constant x
    Horizontal, // Constant y
    Upward,     // Moving from larger y to smaller y
    Downward    // Moving from smaller y to larger y
}

// Create range from small -> large (inclusive), unlike the `..` operator...
fn agnostic_range(start: i32, finish: i32) -> impl Iterator<Item = i32> {
    if start < finish {
        return start..finish + 1;
    }
    else {
        return finish..start + 1;
    }
}

// Determine the slant of the line
fn find_slant(start_point: (i32, i32), end_point: (i32, i32)) -> Slant {
    // Constant x
    if start_point.0 == end_point.0 {
        return Slant::Vertical;
    }
    // Constant y
    else if start_point.1 == end_point.1 {
        return Slant::Horizontal;
    }
    // Moving in the positive direction (left to right)
    else if end_point.0 - start_point.0 > 0 {
        if end_point.1 - start_point.1 > 0 {
            return Slant::Downward;
        }
        else {
            return Slant::Upward;
        }
    }
    // Moving in the negative direction (right to left)
    else {
        if end_point.1 - start_point.1 > 0 {
            return Slant::Upward;
        }
        else {
            return Slant::Downward;
        }
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

        match find_slant(start, end) {
            // Vertical lines
            Slant::Vertical => {
                for y in agnostic_range(start.1, end.1) {
                    let count = intersecting_points.entry((start.0, y)).or_insert(0);
                    *count += 1;
                }
                // println!("ver: {}\n", test_count);
            },
            // Horizontal lines
            Slant::Horizontal => {
                for x in agnostic_range(start.0, end.0) {
                    let count = intersecting_points.entry((x, start.1)).or_insert(0);
                    *count += 1;
                }
                // println!("hor: {}\n", test_count);
            },
            // Sloping from smaller y to larger y
            Slant::Downward => {
                for (index, x) in agnostic_range(start.0, end.0).enumerate() {
                    let min = std::cmp::min(start.1, end.1);
                    let count = intersecting_points.entry((x, min + index as i32)).or_insert(0);
                    *count += 1;
                }
            },
            // Sloping from larger y to smaller y
            Slant::Upward => {
                for (index, x) in agnostic_range(start.0, end.0).enumerate() {
                    let max = std::cmp::max(start.1, end.1);
                    let count = intersecting_points.entry((x, max - index as i32)).or_insert(0);
                    *count += 1;
                }
            }
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