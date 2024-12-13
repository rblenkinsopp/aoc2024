// Advent of Code 2024: Day 1
use itertools::Itertools;
use std::io::BufRead;
use aoc2024::{get_input_reader, print_answer};
use arrayvec::ArrayVec;
use rdst;
use rdst::RadixSort;

// Pre-allocate 1000 entries as this is the expected input count.
const EXPECTED_INPUT_COUNT: usize = 1000;
const EXPECTED_MAX_VALUE:usize = 99999;

fn main() {
    // Parse the data into sorted lists of integers and count the right numbers.
    let mut left_numbers: ArrayVec<u32, EXPECTED_INPUT_COUNT> = ArrayVec::new();
    let mut right_numbers: ArrayVec<u32, EXPECTED_INPUT_COUNT> = ArrayVec::new();
    let mut right_numbers_counts: Vec<u32> = vec![0; EXPECTED_MAX_VALUE];

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
            right_numbers_counts[right as usize] += 1;
        }
    }
    left_numbers.radix_sort_unstable();
    right_numbers.radix_sort_unstable();

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
        .filter_map(|&number| right_numbers_counts.get(number as usize).map(|&count| number * count))
        .sum();

    print_answer(difference_sum, similarity_score);
}