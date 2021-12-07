// Advent of Code Day 4 Part 2 (12/06/21)
// Written in Rust!
// Author: Logan D.G. Smith
// NOTE: Did not become more optimized. Only continued to process boards after the first.

use advent_of_rust_2021::*;

// Wrapper for values
#[derive(Debug)]
struct Space {
    row: i32,
    col: i32,
    value: i32,
    is_called: bool
}

impl Space {
    // Constructor
    pub fn new(row: i32, col: i32, value: i32) -> Space {
        Space {
            row: row,
            col: col,
            value: value,
            is_called: false
        }
    }

    // Return a called version of the space
    pub fn call(&mut self) {
        self.is_called = true;
    }
}

// Board Object
#[derive(Debug)]
struct BingoBoard {
    rows: i32,
    row_length: i32,
    board: Vec<Space>,
    is_complete: bool
}

impl BingoBoard {
    // Constructor
    pub fn new() -> BingoBoard {
        BingoBoard {
            rows: 0,
            row_length: 5, // NOTE: Hardcoded, but can adjust later if desired
            board: Vec::new(),
            is_complete: false
        }
    }

    // Push a new row to the board
    pub fn push(&mut self, new_row: Vec<i32>) {
        let mut row_spaces: Vec<Space> = new_row
            .iter()
            .enumerate()
            .map(|(i, r)| Space::new(self.rows, i.try_into().expect("Failed to convert usize"), *r))
            .collect();

        self.board.append(&mut row_spaces);
        self.rows += 1;
    }

    // Check if board has a specified number
    pub fn check(&mut self, checked_num: i32) -> i32 {
        // Don't evaluate complete boards
        if self.is_complete {
            return 0;
        }

        // Check for bingos
        match self.board.iter().position(|x| x.value == checked_num) {
            Some(position) => {
                let element = &mut self.board[position];
                element.call();
                return self.bingo(self.board[position].row, self.board[position].col, checked_num);
            },
            None => { return 0; }
        };
    }

    // Check for vertical or horizonal bingos
    pub fn bingo(&mut self, row: i32, col: i32, checked_num: i32) -> i32 {
        // Declarations
        let mut is_horizontal_bingo: bool = true;
        let mut is_vertical_bingo: bool = true;

        // Horizonal bingo?
        for space in self.get_row(row) {
            if space.is_called == false { is_horizontal_bingo = false; }
        }

        // Vertical bingo?
        for space in self.get_col(col) {
            if space.is_called == false { is_vertical_bingo = false; }
        }

        // Return Bingo
        if is_horizontal_bingo || is_vertical_bingo {
            //println!("{:?}", self.board);
            self.is_complete = true;
            let total_uncalled = self.find_uncalled();
            return checked_num * total_uncalled;
        }

        return 0;
    }

    // Helper function to get row
    fn get_row(&self, row: i32) -> Vec<&Space> {
        assert!(row >= 0);
        assert!(row < self.rows);
        //println!("{}", row);
        self.board.iter().filter(|s| s.row == row).collect()
    }

    // Helper function to get col
    fn get_col(&self, col: i32) -> Vec<&Space> {
        assert!(col >= 0);
        assert!(col < self.row_length);
        self.board.iter().filter(|s| s.col == col).collect()
    }

    // Calculates the sum of all uncalled spaces on a board
    fn find_uncalled(&self) -> i32 {
        let mut total: i32 = 0;
        //println!("Board: {:?}", self.board);
        for space in &self.board {
            if !space.is_called {
                //println!("Adding: {}", space.value);
                total += space.value;
            }
        }
        total
    }
}

fn main() {
    // Declarations
    let mut called_numbers: Vec<i32> = Vec::new();
    let mut bingo_boards: Vec<BingoBoard> = Vec::new();

    // Obtain iterator for file
    let lines = read_lines("inputs/day4input.txt").expect("Failed to read lines!");

    for (index, line) in lines.enumerate() {
        // Unwrap line
        let reading: String = line.expect("Failed to read called numbers!");

        // Obtain called numbers before building boards
        if index == 0 {
            called_numbers = reading
                .split(",")
                .map(|s| s.parse::<i32>().expect("Failed to parse!"))
                .collect();

            continue;
        }
        
        // Newline -- New Board Needed
        if reading.is_empty() {
            let board: BingoBoard = BingoBoard::new();
            bingo_boards.push(board);

            continue;
        } 

        // Add row to latest BingoBoard
        match bingo_boards.last_mut() {
            Some(board) => {
                board.push(
                    reading
                        .split_whitespace()
                        .map(|s| s.parse::<i32>().expect("Failed to parse bingo row!"))
                        .collect());
            },
            None => {
                panic!("No board found!")
            }
        }
    }

    // Simulate calling numbers
    for called in called_numbers {
        for board in &mut bingo_boards {
            let score = board.check(called);
            if score > 0 {
                println!("Winning Number: {}", called);
                println!("Winning Board Score: {}", score);
            }
        }
    }
}