#![feature(iter_collect_into)]

use aoc2024::get_input_reader;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    let mut lines = get_input_reader().lines().map(|l| unsafe { l.unwrap_unchecked() });

    // Pre-allocate required space
    let mut rules: HashSet<(u32, u32)> = HashSet::with_capacity(1200);
    let mut updates: Vec<Vec<u32>> = Vec::with_capacity(200);

    // Throw safety out the window for speed
    unsafe {
        // Read the rules.
        lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                let (a, b) = line.split_once('|').unwrap_unchecked();
                (a.parse::<u32>().unwrap_unchecked(), b.parse::<u32>().unwrap_unchecked())
            })
            .collect_into(&mut rules);

        // Read the updates.
        lines
            .by_ref()
            .map(|line| {
                line.split(',')
                    .map(|s| s.parse::<u32>().unwrap_unchecked())
                    .collect_vec()
            })
            .collect_into(&mut updates);
    }
    
    let mut correct_middle_page_sum = 0;
    let mut incorrect_middle_page_sum = 0;
    
    for mut update in updates {
        if update.is_sorted_by(|a, b| rules.contains(&(*a, *b))) {
            correct_middle_page_sum += update[update.len() / 2]
        } else {
            let middle_index = update.len() / 2;
            update.select_nth_unstable_by(middle_index,|a, b| {
                if rules.contains(&(*a, *b)) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            incorrect_middle_page_sum += update[update.len() / 2];
        }
    }

    println!("{}\n{}", correct_middle_page_sum, incorrect_middle_page_sum);
}
