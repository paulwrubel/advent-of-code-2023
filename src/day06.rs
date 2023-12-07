use std::fs;

use crate::{utils, AdventError, ExclusivePart};

const INPUT_FILE: &str = "./resources/day06_input.txt";

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

    let mut lines = input.lines();

    let times = utils::integers_from_string::<u64>(
        lines.next().unwrap().split_once(':').unwrap().1.trim(),
        " ",
    );
    let distances = utils::integers_from_string::<u64>(
        lines.next().unwrap().split_once(':').unwrap().1.trim(),
        " ",
    );

    if times.len() != distances.len() {
        return Err(AdventError::Other(
            "times and distances don't match".to_string(),
        ));
    }

    let mut races = Vec::new();
    for i in 0..times.len() {
        races.push(Race {
            time_limit_ms: times[i],
            distance_record_mm: distances[i],
        })
    }

    let num_way_to_win = races.iter().map(|race| {
        let range = race.find_record_breaking_range();
        println!("{} to {}", range.0, range.1);
        (range.1 - range.0) + 1
    });

    let result: u64 = num_way_to_win.product();

    Ok(result.to_string())
}

fn part_two() -> Result<String, AdventError> {
    // read input file
    let input =
        fs::read_to_string(INPUT_FILE).map_err(|err| AdventError::Other(err.to_string()))?;

    let mut lines = input.lines();

    let time = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .replace(" ", "")
        .parse::<u64>()
        .map_err(|err| AdventError::Other(err.to_string()))?;

    let distance = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .replace(" ", "")
        .parse::<u64>()
        .map_err(|err| AdventError::Other(err.to_string()))?;

    let race = Race {
        time_limit_ms: time,
        distance_record_mm: distance,
    };

    let range = race.find_record_breaking_range();
    println!("{} to {}", range.0, range.1);
    let winning_options_count = (range.1 - range.0) + 1;

    Ok(winning_options_count.to_string())
}

struct Race {
    time_limit_ms: u64,
    distance_record_mm: u64,
}

impl Race {
    fn find_record_breaking_range(&self) -> (u64, u64) {
        let options = 0..=self.time_limit_ms;

        let mut min_winning_distance: u64 = 0;
        let mut max_winning_distance: u64 = 0;
        for option in options {
            let total_distance = option * (self.time_limit_ms - option);
            if total_distance > self.distance_record_mm && min_winning_distance == 0 {
                min_winning_distance = option;
            } else if total_distance > self.distance_record_mm {
                max_winning_distance = option;
            }
        }

        (min_winning_distance, max_winning_distance)
    }
}
