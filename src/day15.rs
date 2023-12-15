use std::fs;

use crate::{AdventError, ExclusivePart};

const INPUT_FILE: &str = "./resources/day15_input.txt";

pub fn run(epart: ExclusivePart) -> Result<String, AdventError> {
    match epart {
        ExclusivePart::One => part_one(),
        ExclusivePart::Two => part_two(),
    }
}

fn part_one() -> Result<String, AdventError> {
    // read input file
    let input = fs::read_to_string(INPUT_FILE)?;

    let initialization_sequence = input
        .replace("\n", "")
        .split(",")
        .map(|s| InitializationStep::parse(s))
        .collect::<Result<Vec<InitializationStep>, String>>()?;

    let mut sum_of_hashes = 0;
    for step in initialization_sequence {
        sum_of_hashes += step.raw.hash();
    }

    Ok(sum_of_hashes.to_string())
}

fn part_two() -> Result<String, AdventError> {
    // read input file
    let input = fs::read_to_string(INPUT_FILE)?;

    let initialization_sequence = input
        .replace("\n", "")
        .split(",")
        .map(|s| InitializationStep::parse(s))
        .collect::<Result<Vec<InitializationStep>, String>>()?;

    let mut light_boxes = LightBoxes::new();
    for step in initialization_sequence {
        light_boxes.apply(&step);
    }

    let total_focusing_power = light_boxes.total_focusing_power();

    Ok(total_focusing_power.to_string())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LightBoxes([LightBox; 256]);

impl LightBoxes {
    fn new() -> Self {
        let mut light_boxes = Vec::with_capacity(256);
        for index in 0..256 {
            light_boxes.push(LightBox::new(index));
        }
        Self(light_boxes.try_into().unwrap())
    }

    fn apply(&mut self, step: &InitializationStep) {
        match step.operation {
            InitializationOperation::Remove => {
                let box_index = step.label.hash();
                let light_box = &mut self.0[box_index as usize];

                light_box.remove_lens_if_present(&step.label);
            }
            InitializationOperation::AddOrReplace(focal_length) => {
                let box_index = step.label.hash();
                let light_box = &mut self.0[box_index as usize];

                light_box.add_or_replace_lens(Lens {
                    label: step.label.clone(),
                    focal_length,
                });
            }
        }
    }

    fn total_focusing_power(&self) -> u64 {
        let mut focusing_power = 0;
        for light_box in &self.0 {
            focusing_power += light_box.focusing_power();
        }
        focusing_power
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LightBox {
    index: u64,
    lenses: Vec<Lens>,
}

impl LightBox {
    fn new(index: u64) -> Self {
        Self {
            index,
            lenses: Vec::new(),
        }
    }

    fn remove_lens_if_present(&mut self, label: &str) {
        self.lenses.retain(|l| l.label != label);
    }

    fn add_or_replace_lens(&mut self, lens: Lens) {
        match self.lenses.iter().position(|l| l.label == lens.label) {
            Some(i) => {
                self.lenses[i] = lens;
            }
            None => {
                self.lenses.push(lens);
            }
        }
    }

    fn focusing_power(&self) -> u64 {
        let mut focusing_power = 0;
        for (i, lens) in self.lenses.iter().enumerate() {
            focusing_power += (1 + self.index) * (i as u64 + 1) * lens.focal_length
        }
        focusing_power
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Lens {
    label: String,
    focal_length: u64,
}

struct InitializationStep {
    raw: String,
    label: String,
    operation: InitializationOperation,
}

impl InitializationStep {
    fn parse(input: &str) -> Result<Self, String> {
        if input.contains("-") {
            Ok(Self {
                raw: input.to_string(),
                label: input[..input.len() - 1].to_string(),
                operation: InitializationOperation::Remove,
            })
        } else if input.contains("=") {
            let (label, focal_length) = input.split_once("=").unwrap();
            Ok(Self {
                raw: input.to_string(),
                label: label.to_string(),
                operation: InitializationOperation::AddOrReplace(
                    focal_length
                        .parse()
                        .map_err(|err| format!("invalid focal length: {}", err))?,
                ),
            })
        } else {
            Err("invalid input".to_string())
        }
    }
}

enum InitializationOperation {
    Remove,
    AddOrReplace(u64),
}

trait HASH {
    fn hash(&self) -> u64;
}

impl HASH for &str {
    fn hash(&self) -> u64 {
        let mut current_value = 0;
        for c in self.chars() {
            current_value += c as u64;
            current_value *= 17;
            current_value %= 256;
        }
        current_value
    }
}

impl HASH for String {
    fn hash(&self) -> u64 {
        self.as_str().hash()
    }
}
