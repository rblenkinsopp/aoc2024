// Advent of Code 2024: Day 1
use itertools::Itertools;
use std::collections::HashMap;
use std::io::BufRead;
use aoc2024::get_input;

fn main() -> anyhow::Result<()> {
    // Parse the data into sorted lists of integers and count the right numbers.
    let mut left_numbers: Vec<i32> = Vec::new();
    let mut right_numbers: Vec<i32> = Vec::new();
    let mut right_numbers_counts: HashMap<i32, i32> = HashMap::new();
    for line in get_input().lines() {
        let (left, right) = line?
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();
        left_numbers.push(left);
        right_numbers.push(right);
        *right_numbers_counts.entry(right).or_insert(0) += 1;
    }
    left_numbers.sort();
    right_numbers.sort();

    // Part 1
    // Calculate the accumulation of the difference between the two numbers.
    let mut difference_sum = 0;
    for (left, right) in left_numbers.iter().zip(right_numbers.iter()) {
        difference_sum += (left - right).abs();
    }
    println!("{}", difference_sum);

    // Part 2
    // Calculate the similarity score.
    let mut similarity_score: i32 = 0;
    for number in left_numbers {
        similarity_score += number * *right_numbers_counts.entry(number).or_default();
    }
    println!("{}", similarity_score);

    Ok(())
}