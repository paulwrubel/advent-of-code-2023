use std::fs;

use crate::{AdventError, ExclusivePart};

const INPUT_FILE: &str = "./resources/day01_input.txt";

#[derive(Clone, Copy, Debug)]
enum Extrema {
    First,
    Last,
}

pub fn run(epart: ExclusivePart) -> Result<String, AdventError> {
    match epart {
        ExclusivePart::One => part_one(),
        ExclusivePart::Two => part_two(),
    }
}

fn part_one() -> Result<String, AdventError> {
    // read input file
    let input =
        fs::read_to_string(INPUT_FILE).map_err(|err| AdventError::Other(err.to_string()))?;

    let calibration_values: Result<Vec<u32>, AdventError> = input
        .lines()
        .map(|line| -> Result<u32, AdventError> {
            let first_digit = match line.chars().find(|char| char.is_numeric()) {
                Some(char) => char,
                None => {
                    return Err(AdventError::Other(
                        "Couldn't find first digit (bad input?)".to_string(),
                    ))
                }
            };

            let last_digit = match line.chars().rev().find(|char| char.is_numeric()) {
                Some(char) => char,
                None => {
                    return Err(AdventError::Other(
                        "Couldn't find last digit (bad input?)".to_string(),
                    ))
                }
            };

            let mut value_string = String::new();
            value_string.push(first_digit);
            value_string.push(last_digit);

            value_string
                .parse::<u32>()
                .map_err(|err| AdventError::Other(err.to_string()))
        })
        .collect();

    Ok(calibration_values?.iter().sum::<u32>().to_string())
}

fn part_two() -> Result<String, AdventError> {
    // read input file
    let input = fs::read_to_string(INPUT_FILE).expect("Error reading input file");

    let calibration_values: Result<Vec<u32>, AdventError> = input
        .lines()
        .map(|line| -> Result<u32, AdventError> {
            let first_digit = get_digit(line, Extrema::First).ok_or(AdventError::Other(
                "Couldn't find first digit (bad input?)".to_string(),
            ))?;
            let last_digit = get_digit(line, Extrema::Last).ok_or(AdventError::Other(
                "Couldn't find last digit (bad input?)".to_string(),
            ))?;

            let mut value_string = String::new();
            value_string.push_str(first_digit.to_string().as_str());
            value_string.push_str(last_digit.to_string().as_str());

            value_string
                .parse::<u32>()
                .map_err(|err| AdventError::Other(err.to_string()))
        })
        .collect();

    Ok(calibration_values?.iter().sum::<u32>().to_string())
}

fn get_digit(s: &str, extrema: Extrema) -> Option<u32> {
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
            Some(char_digit)
        } else {
            Some(word_digit)
        }
    } else if char.is_some() {
        Some(char.unwrap().1)
    } else if word.is_some() {
        Some(word.unwrap().1)
    } else {
        None
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
