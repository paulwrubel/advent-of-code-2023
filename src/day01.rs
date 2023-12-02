use std::{fs, str::FromStr};

const INPUT_FILE: &str = "./resources/day01_input.txt";

#[derive(Clone, Copy, Debug)]
enum Extrema {
    First,
    Last,
}

pub fn run() -> (String, String) {
    ("???".to_string(), part_two())
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
            .expect("Couldn't find last digit (bad input?)");

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
    // read input file
    let input = fs::read_to_string(INPUT_FILE).expect("Error reading input file");

    let calibration_values = input.lines().map(|line| -> u32 {
        let first_digit = get_digit(line, Extrema::First);
        let last_digit = get_digit(line, Extrema::Last);

        let mut value_string = String::new();
        value_string.push_str(first_digit.to_string().as_str());
        value_string.push_str(last_digit.to_string().as_str());

        println!("{}: {}", value_string, line);

        value_string
            .parse()
            .expect("Couldn't parse concatenated digits! (how...?)")
    });

    calibration_values.sum::<u32>().to_string()
}

fn get_digit(s: &str, extrema: Extrema) -> u32 {
    let char = get_digit_char(s, extrema);
    let word = get_digit_word(s, extrema);

    if char.is_some() && word.is_some() {
        let (char_index, char_digit) = char.unwrap();
        let (word_index, word_digit) = word.unwrap();

        let use_char = match extrema {
            Extrema::First => char_index < word_index,
            Extrema::Last => char_index > word_index,
        };
        if use_char {
            char_digit
        } else {
            word_digit
        }
    } else if char.is_some() {
        char.unwrap().1
    } else if word.is_some() {
        word.unwrap().1
    } else {
        panic!(
            "Couldn't find extrema digit in: {} for extrema: {:?}",
            s, extrema
        )
    }
}

fn get_digit_char(s: &str, extrema: Extrema) -> Option<(usize, u32)> {
    match extrema {
        Extrema::First => s
            .char_indices()
            .find(|(_idx, char)| char.is_numeric())
            .map(|(idx, char)| (idx, char.to_digit(10).unwrap())),
        Extrema::Last => s
            .char_indices()
            .rfind(|(_idx, char)| char.is_numeric())
            .map(|(idx, char)| (idx, char.to_digit(10).unwrap())),
    }
}

fn get_digit_word(s: &str, ext: Extrema) -> Option<(usize, u32)> {
    let ext_find = |s: &str, literal: &str| match ext {
        Extrema::First => s.find(literal),
        Extrema::Last => s.rfind(literal),
    };
    vec![
        ext_find(s, "one"),
        ext_find(s, "two"),
        ext_find(s, "three"),
        ext_find(s, "four"),
        ext_find(s, "five"),
        ext_find(s, "six"),
        ext_find(s, "seven"),
        ext_find(s, "eight"),
        ext_find(s, "nine"),
    ]
    .iter()
    .enumerate()
    .fold(None, |acc, (vec_idx, s_idx_opt)| match s_idx_opt {
        Some(s_idx) => match acc {
            Some((acc_idx, _acc_digit)) => {
                let should_replace = match ext {
                    Extrema::First => *s_idx < acc_idx,
                    Extrema::Last => *s_idx > acc_idx,
                };
                if should_replace {
                    Some((*s_idx, (vec_idx as u32) + 1))
                } else {
                    acc
                }
            }
            None => Some((*s_idx, (vec_idx as u32) + 1)),
        },
        None => acc,
    })
}
