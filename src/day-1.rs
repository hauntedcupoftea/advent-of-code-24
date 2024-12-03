use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub fn calculate_total_distance(file_path: &str) -> io::Result<i32> {
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

pub fn calculate_similarity_score(file_path: &str) -> io::Result<i32> {
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

    // Count occurrences of numbers in the right list
    let right_counts: HashMap<i32, usize> =
        right_list.iter().fold(HashMap::new(), |mut acc, &num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        });

    // Calculate similarity score
    let similarity_score: i32 = left_list
        .iter()
        .map(|&num| num * right_counts.get(&num).cloned().unwrap_or(0) as i32)
        .sum();

    Ok(similarity_score)
}

pub fn solve_puzzle_1(file_path: &str) -> io::Result<i32> {
    calculate_total_distance(file_path)
}

pub fn solve_puzzle_2(file_path: &str) -> io::Result<i32> {
    calculate_similarity_score(file_path)
}
