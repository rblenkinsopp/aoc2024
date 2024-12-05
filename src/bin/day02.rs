// Advent of Code 2024: Day 2
use std::io::BufRead;
use aoc2024::get_input;

fn check_report(report: &Vec<i32>) -> bool {
    let mut report_direction: i32 = 0;
    for i in 0..report.len() - 1 {
        let difference = report[i] - report[i + 1];

        // Check if unsafe due to magnitude of change.
        if !(1..4).contains(&difference.abs()) {
            return false
        }

        // Check if unsafe due to not all increasing or decreasing.
        if report_direction != 0 && difference.signum() != report_direction {
            return false
        }

        // Record the sequence direction.
        report_direction = difference.signum()
    }
    true
}

fn main() {
    // Track number of safe reports and safe when dampened report.
    let mut safe_reports = 0;
    let mut dampened_safe_reports = 0;

    // Parse the data into a list of reports.
    for report in get_input()
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        }) {

        // Check the report unmodified - it's safe if it passes.
        if check_report(&report) {
            safe_reports += 1;
            continue
        }

        // Attempt to solve the issue with the problem dampener if there is one.
        for index_to_remove in 0..report.len() {
            let mut modified_report = report.clone();
            modified_report.remove(index_to_remove);

            if check_report(&modified_report) {
                dampened_safe_reports += 1;
                break;
            }
        }
    }

    println!("{}", safe_reports);
    println!("{}", dampened_safe_reports);
}
