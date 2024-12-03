use std::fs::File;
use std::io::{self, BufRead};

fn calculate_total_distance(file_path: &str) -> io::Result<i32> {
    // Open the file in read-only mode
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Collect the numbers from the file
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        if numbers.len() == 2 {
            left_list.push(numbers[0]);
            right_list.push(numbers[1]);
        }
    }

    // Sort both lists
    left_list.sort();
    right_list.sort();

    // Calculate the total distance between the pairs
    let total_distance: i32 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(left, right)| (left - right).abs()) // Calculate the absolute difference
        .sum(); // Sum the differences

    Ok(total_distance)
}

fn main() {
    let file_path = "../data/day1-1.txt"; // Replace with your actual file path
    match calculate_total_distance(file_path) {
        Ok(distance) => println!("Total distance: {}", distance),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
