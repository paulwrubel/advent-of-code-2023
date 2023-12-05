use std::{collections::HashMap, fs};

use crate::{AdventError, ExclusivePart};

const INPUT_FILE: &str = "./resources/day04_input.txt";

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

    // parse input
    let card_matches = input.lines().map(|line| {
        let (_, nums) = line.split_once(':').unwrap();
        let (winners, owned) = nums.split_once('|').unwrap();

        let winners: Vec<&str> = winners
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect();
        let owned = owned.trim().split(' ').filter(|s| !s.is_empty());

        let matches = owned
            .filter(move |owned| winners.iter().any(|winner| **winner == **owned))
            .map(|s| s.to_owned());

        matches
    });

    let card_points = card_matches.map(|matching_nums| {
        let len = matching_nums.count() as u32;
        let points = if len > 0 { (2 as u32).pow(len) / 2 } else { 0 };

        // println!("len: {}, points: {}", len, points);

        points
    });

    let result = card_points.sum::<u32>();

    Ok(result.to_string())
}

fn part_two() -> Result<String, AdventError> {
    // read input file
    let input =
        fs::read_to_string(INPUT_FILE).map_err(|err| AdventError::Other(err.to_string()))?;

    // parse input
    let card_matches: Vec<Vec<String>> = input
        .lines()
        .map(|line| {
            let (_, nums) = line.split_once(':').unwrap();
            let (winners, owned) = nums.split_once('|').unwrap();

            let winners: Vec<&str> = winners
                .trim()
                .split(' ')
                .filter(|s| !s.is_empty())
                .collect();
            let owned = owned.trim().split(' ').filter(|s| !s.is_empty());

            let matches = owned
                .filter(move |owned| winners.iter().any(|winner| **winner == **owned))
                .map(|s| s.to_owned());

            matches.collect()
        })
        .collect();

    let mut card_counts = HashMap::new();
    // pre-fill counts with originals
    for card_num in 1..=card_matches.len() {
        card_counts.insert(card_num as u32, 1);
    }

    // process each card, include any aquired copies
    let max_card_num = card_matches.len() as u32;
    for (index, card_match_count) in card_matches.iter().enumerate() {
        let card_num = index as u32 + 1;

        let current_card_instance_count = *card_counts.get(&card_num).unwrap();
        // println!(
        //     "processing card #{} with {} instances",
        //     card_num, current_card_instance_count
        // );

        let match_count = card_match_count.len() as u32;
        if match_count == 0 {
            continue;
        }
        for copy_num in (card_num + 1)..=((card_num + match_count).min(max_card_num)) {
            // add a copy of the cards we won
            let (_old_count, _new_count) =
                if let Some(current_won_card_count) = card_counts.get_mut(&copy_num) {
                    let old_instance_count = *current_won_card_count;
                    let new_instance_count = *current_won_card_count + current_card_instance_count;
                    *current_won_card_count = new_instance_count;
                    (old_instance_count, new_instance_count)
                } else {
                    card_counts.insert(copy_num, current_card_instance_count);
                    (0, match_count)
                };
            // println!(
            //     "incrementing #{} by {} (COPY) [{} -> {}]",
            //     copy_num, current_card_instance_count, old_count, new_count,
            // );
        }
    }

    let total_copies = card_counts
        .iter()
        .fold(0 as u32, |acc, (_, copies)| acc + *copies);

    Ok(total_copies.to_string())
}
