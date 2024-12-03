use advent_of_code_day1::{solve_puzzle_1_1, solve_puzzle_1_2};
use std::env;
use std::process;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if we have enough arguments
    if args.len() < 3 {
        eprintln!("Usage: {} <puzzle_number> <file_path>", args[0]);
        process::exit(1);
    }

    // Parse puzzle number
    let puzzle_number = &args[1];

    // Get file path
    let file_path = &args[2];

    // Solve the specified puzzle
    let result = match puzzle_number.as_str() {
        "1-1" => solve_puzzle_1_1(file_path),
        "1-2" => solve_puzzle_1_2(file_path),
        _ => {
            eprintln!("Puzzle {} is not implemented", puzzle_number);
            process::exit(1);
        }
    };

    // Handle the result
    match result {
        Ok(distance) => println!("Puzzle {} result: {}", puzzle_number, distance),
        Err(e) => {
            eprintln!("Error solving puzzle {}: {}", puzzle_number, e);
            process::exit(1);
        }
    }
}
