use std::fs;

use crate::UNIMPLEMENTED;

const INPUT_FILE: &str = "./resources/day01_input.txt";

pub fn run() -> (String, String) {
    (part_one(), part_two())
}

fn part_one() -> String {
    // read input file
    let input = fs::read_to_string(INPUT_FILE).expect("Error reading input file");

    let calibration_values = input.lines().map(|line| -> u32 {
        let first_digit = line
            .chars()
            .find(|char| char.is_numeric())
            .expect("Couldn't find first digit (bad input?)");

        let last_digit = line
            .chars()
            .rev()
            .find(|char| char.is_numeric())
            .expect("Couldn't find first digit (bad input?)");

        let mut value_string = String::new();
        value_string.push(first_digit);
        value_string.push(last_digit);

        value_string
            .parse()
            .expect("Couldn't parse concatenated digits! (how...?)")
    });

    calibration_values.sum::<u32>().to_string()
}

fn part_two() -> String {
    UNIMPLEMENTED.to_string()
}
