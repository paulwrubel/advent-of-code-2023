use std::{collections::HashMap, fs};

use crate::{utils::integers_from_string, AdventError, ExclusivePart};

const INPUT_FILE: &str = "./resources/day02_input.txt";

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

    let mut possible_ids = Vec::new();

    // parse input
    for line in input.lines() {
        let mut split = line.split(':');
        let game_id = integers_from_string(split.next().unwrap(), " ")[0];
        let reveals: Vec<&str> = split.next().unwrap().split(';').collect();

        let mut reveals = reveals.iter().map(|s| {
            let mut colors = HashMap::new();
            for c in s.split(',') {
                let (num, color) = c.trim().split_once(' ').unwrap();
                colors.insert(color, num.parse::<i32>().unwrap());
            }
            colors
        });

        if !reveals.any(|m| {
            *m.get("red").unwrap_or(&0) > 12
                || *m.get("green").unwrap_or(&0) > 13
                || *m.get("blue").unwrap_or(&0) > 14
        }) {
            possible_ids.push(game_id);
        }
    }

    let result = possible_ids.iter().sum::<i32>().to_string();

    Ok(result)
}

fn part_two() -> Result<String, AdventError> {
    // read input file
    let input =
        fs::read_to_string(INPUT_FILE).map_err(|err| AdventError::Other(err.to_string()))?;

    let mut powers = Vec::new();

    // parse input
    for line in input.lines() {
        let mut split = line.split(':');
        let _game_id = integers_from_string(split.next().unwrap(), " ")[0];
        let reveals: Vec<&str> = split.next().unwrap().split(';').collect();

        let reveals = reveals.iter().map(|s| {
            let mut colors = HashMap::new();
            for c in s.split(',') {
                let (num, color) = c.trim().split_once(' ').unwrap();
                colors.insert(color, num.parse::<i32>().unwrap());
            }
            colors
        });

        let mut mins = HashMap::new();
        for m in reveals {
            let red = m.get("red").unwrap_or(&0);
            if red > mins.get("red").get_or_insert(&0) {
                mins.insert("red", *red);
            }

            let green = m.get("green").unwrap_or(&0);
            if green > mins.get("green").get_or_insert(&0) {
                mins.insert("green", *green);
            }

            let blue = m.get("blue").unwrap_or(&0);
            if blue > mins.get("blue").get_or_insert(&0) {
                mins.insert("blue", *blue);
            }
        }
        let mut power = 1;
        for v in mins.values() {
            power *= v;
        }
        powers.push(power);
    }

    let result = powers.iter().sum::<i32>().to_string();

    Ok(result)
}
