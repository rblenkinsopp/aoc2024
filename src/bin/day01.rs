// Advent of Code 2024: Day 1
use std::io::BufRead;
use aoc2024::get_input_reader;
use arrayvec::ArrayVec;
use fxhash::{FxBuildHasher, FxHashMap};
use rdst::RadixSort;

// Pre-allocate 1000 entries as this is the expected input count.
const EXPECTED_INPUT_COUNT: usize = 1000;

fn main() {
    // Parse the data into sorted lists of integers and count the right numbers.
    let mut left_numbers: ArrayVec<u32, EXPECTED_INPUT_COUNT> = ArrayVec::new();
    let mut right_numbers: ArrayVec<u32, EXPECTED_INPUT_COUNT> = ArrayVec::new();
    let mut right_numbers_counts: FxHashMap<u32, u32> = FxHashMap::with_capacity_and_hasher(EXPECTED_INPUT_COUNT, FxBuildHasher::default());

    for line in get_input_reader().lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();

        let left = parts.next().unwrap().parse::<u32>().unwrap();
        left_numbers.push(left);

        let right = parts.next().unwrap().parse::<u32>().unwrap();
        right_numbers.push(right);
        *right_numbers_counts.entry(right).or_insert(0) += 1;
    }
    left_numbers.radix_sort_unstable();
    right_numbers.radix_sort_unstable();

    // Part 1
    // Calculate the accumulation of the difference between the two numbers.
    let mut difference_sum: u32 = 0;
    for (left, right) in left_numbers.iter().zip(&right_numbers) {
        difference_sum += left.abs_diff(*right);
    }

    // Part 2
    // Calculate the similarity score.
    let mut similarity_score: u32 = 0;
    for &number in &left_numbers {
        if let Some(&count) = right_numbers_counts.get(&number) {
            similarity_score += number * count;
        }
    }

    println!("{}\n{}", difference_sum, similarity_score);
}