use aoc2024::get_input_as_string;
use regex::Regex;

fn main() {
    let data = get_input_as_string();

    // Compile a suitable regex to match uncorrupted multiplications.
    let regex = Regex::new(r"(?m)(do|don't|mul)\((?:(\d{1,3}),(\d{1,3}))?\)").unwrap();

    // Iterate over the mul statements and accumulate the result, tracking mul enablement.
    let mut part1_result = 0;
    let mut part2_result = 0;
    let mut bool_mul_enabled = true;

    // Nice version
    //for instruction in regex.captures_iter(&data) {
    //    match instruction.get(1).unwrap().as_str() {
    //        "do" => bool_mul_enabled = true,
    //        "don't" => bool_mul_enabled = false,
    //        "mul" => {
    //            let (a, b) = (instruction[2].parse::<i32>().unwrap(), instruction[3].parse::<i32>().unwrap());
    //            let mul_result = a * b;
    //            part1_result += mul_result;
    //            if bool_mul_enabled {
    //                part2_result += mul_result;
    //            }
    //        },
    //        &_ => panic!()
    //    }
    //}

    // Hacky version for speed.
    for instruction in regex.captures_iter(&data) {
        if instruction.get(2).is_none() {
            bool_mul_enabled = instruction.get(1).unwrap().len() == 2;
        } else {
            let (a, b) = (instruction[2].parse::<i32>().unwrap(), instruction[3].parse::<i32>().unwrap());
            let mul_result = a * b;
            part1_result += mul_result;
            if bool_mul_enabled {
                part2_result += mul_result;
            }
        }
    }

    println!("{}", part1_result);
    println!("{}", part2_result);
}