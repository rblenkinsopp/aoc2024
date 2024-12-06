// Advent of Code 2024: Day 2
use std::io::BufRead;
use aoc2024::get_input_reader;

#[inline]
fn check_report(report: &[i32], ignore_index: isize) -> bool {
    let mut report_direction: i32 = 0;
    let report_len = report.len() as isize;

    for i in 0..report_len {
        if i == ignore_index {
            continue;
        }

        let mut j = i + 1;
        if j == ignore_index {
            j += 1;
        }

        if j >= report_len {
            break;
        }


        let difference = unsafe{ report.get_unchecked(i as usize) - report.get_unchecked(j as usize) };

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
    for report in get_input_reader()
        .lines()
        .map(|line| unsafe {
            line.unwrap_unchecked()
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap_unchecked())
                .collect::<Vec<i32>>()
        }) {

        // Check the report unmodified - it's safe if it passes.
        if check_report(&report, -1) {
            safe_reports += 1;
            continue
        }

        // Attempt to solve the issue with the problem dampener if there is one.
        for index_to_remove in 0..report.len() {
            if check_report(&report, index_to_remove as isize) {
                dampened_safe_reports += 1;
                break;
            }
        }
    }

    println!("{}\n{}", safe_reports, safe_reports + dampened_safe_reports);
}
