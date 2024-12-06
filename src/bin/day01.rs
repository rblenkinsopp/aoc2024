// Advent of Code 2024: Day 1
use itertools::Itertools;
use std::collections::HashMap;
use std::io::BufRead;
use aoc2024::get_input_reader;

// Pre-allocate 1000 entries as this is the expected input count.
const EXPECTED_INPUT_COUNT: usize = 1000;

fn main() {
    // Parse the data into sorted lists of integers and count the right numbers.
    let mut left_numbers: Vec<u32> = Vec::with_capacity(EXPECTED_INPUT_COUNT);
    let mut right_numbers: Vec<u32> = Vec::with_capacity(EXPECTED_INPUT_COUNT);
    let mut right_numbers_counts: HashMap<u32, u32> = HashMap::with_capacity(EXPECTED_INPUT_COUNT);

    // Input will always be valid so ignore additional safety checks
    unsafe {
        for line in get_input_reader().lines() {
            let (left, right) = line.unwrap_unchecked()
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap_unchecked())
                .collect_tuple()
                .unwrap_unchecked();
            left_numbers.push(left);
            right_numbers.push(right);
            *right_numbers_counts.entry(right).or_insert(0) += 1;
        }
    }
    left_numbers.sort_unstable();
    right_numbers.sort_unstable();

    // Part 1
    // Calculate the accumulation of the difference between the two numbers.
    let difference_sum: u32 = left_numbers
        .iter()
        .zip(right_numbers)
        .map(|(left, right)| left.abs_diff(right))
        .sum();


    // Part 2
    // Calculate the similarity score.
    let similarity_score: u32 = left_numbers.iter()
        .filter_map(|&number| right_numbers_counts.get(&number).map(|&count| number * count))
        .sum();

    println!("{}\n{}", difference_sum, similarity_score);
}