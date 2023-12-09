use std::fs;

use crate::{utils, AdventError, ExclusivePart};

const INPUT_FILE: &str = "./resources/day09_input.txt";

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

    let sequence_trees: Vec<_> = input
        .lines()
        .map(|line| SequenceTree::parse_from_str(line))
        .collect();

    let mut prediction_sum = 0;
    for tree in sequence_trees {
        let prediction = tree.predict_next();
        prediction_sum += prediction;
    }

    Ok(prediction_sum.to_string())
}

fn part_two() -> Result<String, AdventError> {
    // read input file
    let input =
        fs::read_to_string(INPUT_FILE).map_err(|err| AdventError::Other(err.to_string()))?;

    let sequence_trees: Vec<_> = input
        .lines()
        .map(|line| SequenceTree::parse_from_str(line))
        .collect();

    let mut prediction_sum = 0;
    for tree in sequence_trees {
        let prediction = tree.predict_previous();
        prediction_sum += prediction;
    }

    Ok(prediction_sum.to_string())
}

#[derive(Debug)]
struct SequenceTree {
    layers: Vec<Vec<i64>>,
}

impl SequenceTree {
    fn parse_from_str(input: &str) -> Self {
        let nums = utils::integers_from_string::<i64>(input, " ");

        let mut layers = Vec::new();
        layers.push(nums);

        while !layers.last().unwrap().iter().all(|i| *i == 0) {
            let last_layer = layers.last().unwrap();

            let mut diff_layer = Vec::new();
            for i in 1..last_layer.len() {
                let diff = last_layer[i] - last_layer[i - 1];
                diff_layer.push(diff);
            }

            layers.push(diff_layer);
        }

        Self { layers }
    }

    fn predict_next(&self) -> i64 {
        let mut prediction = 0;
        for i in (0..self.layers.len() - 1).rev() {
            let this_layer = &self.layers[i];

            prediction = this_layer.last().unwrap() + prediction;
        }
        prediction
    }

    fn predict_previous(&self) -> i64 {
        let mut prediction = 0;
        for i in (0..self.layers.len() - 1).rev() {
            let this_layer = &self.layers[i];

            prediction = this_layer.first().unwrap() - prediction;
        }
        prediction
    }
}
