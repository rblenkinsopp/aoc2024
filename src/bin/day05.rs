#![feature(iter_collect_into)]

use aoc2024::get_input_reader;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    let mut lines = get_input_reader().lines().map(|l| l.unwrap());

    // Pre-allocate required space
    let mut rules: HashSet<(u32, u32)> = HashSet::with_capacity(1200);
    let mut updates: Vec<Vec<u32>> = Vec::with_capacity(250);

    // Read the rules.
    lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.split('|')
                .map(|s| s.parse::<u32>().unwrap())
                .collect_tuple::<(u32, u32)>()
                .unwrap()
        })
        .collect_into(&mut rules);

    // Read the updates.
    lines
        .by_ref()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect_vec()
        })
        .collect_into(&mut updates);

    let mut correct_middle_page_sum = 0;
    let mut incorrect_middle_page_sum = 0;
    for mut update in updates {
        if update.is_sorted_by(|a, b| rules.contains(&(*a, *b))) {
            correct_middle_page_sum += update[update.len() / 2]
        } else {
            update.sort_by(|a, b| {
                if rules.contains(&(*a, *b)) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            incorrect_middle_page_sum += update[update.len() / 2];
        }
    }

    println!("{}", correct_middle_page_sum);
    println!("{}", incorrect_middle_page_sum);
}
