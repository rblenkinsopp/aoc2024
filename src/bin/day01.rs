// Advent of Code 2024: Day 1
use aoc2024::get_input;
use itertools::Itertools;
use std::collections::HashMap;
use std::io::BufRead;

fn day01(input: impl BufRead) -> anyhow::Result<(i32, i32)> {
    // Parse the data into sorted lists of integers and count the right numbers.
    let mut left_numbers: Vec<i32> = Vec::new();
    let mut right_numbers: Vec<i32> = Vec::new();
    let mut right_numbers_counts: HashMap<i32, i32> = HashMap::new();
    for line in input.lines() {
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

    // Part 2
    // Calculate the similarity score.
    let mut similarity_score: i32 = 0;
    for number in left_numbers {
        similarity_score += number * *right_numbers_counts.entry(number).or_default();
    }
    
    Ok((difference_sum, similarity_score))
}

fn main() -> anyhow::Result<()>{
    day01(get_input())?;
    Ok(())
}

#[cfg(test)]
mod test {
    use std::io::BufReader;
    use super::*;

    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";
    const TEST_ANSWER_PART_1: i32 = 11;
    const TEST_ANSWER_PART_2: i32 = 31;

    #[test]
    fn test() {
        match day01(BufReader::new(std::io::Cursor::new(TEST_INPUT))) {
            Ok(result) => assert_eq!(result, (TEST_ANSWER_PART_1, TEST_ANSWER_PART_2)),
            Err(err) => panic!("Test failed with error: {:?}", err),
        };
    }
}
