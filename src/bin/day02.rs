// Advent of Code 2024: Day 2
use std::io::BufRead;
use aoc2024::get_input;

fn check_report(report: &Vec<i32>, ignore_index: Option<usize>) -> bool {
    let mut report_direction: i32 = 0;
    let report_len = report.len();

    for i in 0..report_len {
        if Some(i) == ignore_index {
            continue;
        }

        let mut j = i + 1;
        while Some(j) == ignore_index {
            j += 1;
        }

        if j >= report_len {
            break;
        }

        let difference = report[i] - report[j];

        // Check if unsafe due to magnitude of change.
        if !(1..4).contains(&difference.abs()) {
            return false;
        }

        // Check if unsafe due to not all increasing or decreasing.
        if report_direction != 0 && difference.signum() != report_direction {
            return false;
        }

        // Record the sequence direction.
        report_direction = difference.signum();
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
                .collect::<Vec<i32>>()
        }) {

        // Check the report unmodified - it's safe if it passes.
        if check_report(&report, None) {
            safe_reports += 1;
            continue
        }

        // Attempt to solve the issue with the problem dampener if there is one.
        for index_to_remove in 0..report.len() {
            if check_report(&report, index_to_remove.into()) {
                dampened_safe_reports += 1;
                break;
            }
        }
    }

    println!("{}", safe_reports);
    println!("{}", safe_reports + dampened_safe_reports);
}
