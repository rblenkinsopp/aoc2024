use aoc2024::get_input_as_string;
use regex::Regex;

fn main() {
    let data = get_input_as_string();

    // Compile a suitable regex to match uncorrupted multiplications.
    let regex = Regex::new(r"(?m)(do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\))").unwrap();

    // Iterate over the mul statements and accumulate the result, tracking mul enablement.
    let mut part1_result = 0;
    let mut part2_result = 0;
    let mut bool_mul_enabled = true;

    // Hacky version for speed.
    for instruction in regex.captures_iter(&data) {
        match instruction.get(0).unwrap().len() {
            4 => bool_mul_enabled = true,  // Do() 
            7 => bool_mul_enabled = false, // Don't()
            _ => {
                let (a, b) = (instruction[2].parse::<i32>().unwrap(), instruction[3].parse::<i32>().unwrap());
                let mul_result = a * b;
                part1_result += mul_result;
                if bool_mul_enabled {
                    part2_result += mul_result;
                }
            }
        }
    }

    println!("{}", part1_result);
    println!("{}", part2_result);
}