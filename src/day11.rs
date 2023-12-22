use std::fs;

use crate::{
    utils::{Grid, GridEntry, GridPoint},
    AdventError, ExclusivePart,
};

const INPUT_FILE: &str = "./resources/day11_input.txt";

pub fn run(epart: ExclusivePart) -> Result<String, AdventError> {
    match epart {
        ExclusivePart::One => part_one(),
        ExclusivePart::Two => part_two(),
    }
}

fn part_one() -> Result<String, AdventError> {
    // read input file
    let input = fs::read_to_string(INPUT_FILE)?;

    let mut galaxy_map = GalaxyMap::parse_from_str(&input)?;

    galaxy_map.set_empty_space_expansion_scalar(2);

    let total_distance = galaxy_map.find_sum_of_all_galaxy_pair_distances();

    Ok(total_distance.to_string())
}

fn part_two() -> Result<String, AdventError> {
    // read input file
    let input = fs::read_to_string(INPUT_FILE)?;

    let mut galaxy_map = GalaxyMap::parse_from_str(&input)?;

    galaxy_map.set_empty_space_expansion_scalar(1_000_000);

    let total_distance = galaxy_map.find_sum_of_all_galaxy_pair_distances();

    Ok(total_distance.to_string())
}

struct GalaxyMap {
    data: Grid<Sector>,
    empty_row_indices: Vec<usize>,
    empty_column_indices: Vec<usize>,
    empty_space_expansion_scalar: u64,
}

impl GalaxyMap {
    fn parse_from_str(input: &str) -> Result<Self, String> {
        // assumed to all be the same length!
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        let mut data = Grid::new_empty(width, height);
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                data.set(
                    &GridPoint {
                        x: x as i64,
                        y: y as i64,
                    },
                    Sector::try_parse(c)?,
                )?;
            }
        }

        let mut empty_row_indices = Vec::new();
        for (y, mut row) in data.rows_iter().enumerate() {
            if row.all(|s| *s == Sector::EmptySpace) {
                // this might hurt...
                empty_row_indices.push(y);
            }
        }

        let mut empty_column_indices = Vec::new();
        for (x, mut column) in data.columns_iter().enumerate() {
            if column.all(|s| *s == Sector::EmptySpace) {
                // this might hurt...
                empty_column_indices.push(x);
            }
        }

        Ok(Self {
            data,
            empty_row_indices,
            empty_column_indices,
            empty_space_expansion_scalar: 1,
        })
    }

    fn set_empty_space_expansion_scalar(&mut self, scalar: u64) {
        self.empty_space_expansion_scalar = scalar;
    }

    fn find_sum_of_all_galaxy_pair_distances(&self) -> i64 {
        self.find_galaxy_pairs()
            .iter()
            .map(|pair| {
                pair.true_distance(
                    &self.empty_row_indices,
                    &self.empty_column_indices,
                    self.empty_space_expansion_scalar,
                )
            })
            .sum()
    }

    fn find_galaxy_pairs(&self) -> Vec<GalaxyPair> {
        let mut galaxy_pairs = Vec::new();

        let galaxy_coords = self.find_galaxy_coords();
        for ai in 0..galaxy_coords.len() - 1 {
            for bi in ai + 1..galaxy_coords.len() {
                galaxy_pairs.push(GalaxyPair {
                    a: galaxy_coords[ai],
                    b: galaxy_coords[bi],
                })
            }
        }

        galaxy_pairs
    }

    fn find_galaxy_coords(&self) -> Vec<(i64, i64)> {
        self.data
            .entries()
            .filter_map(
                |GridEntry {
                     point: GridPoint { x, y },
                     value,
                 }| {
                    if *value == Sector::Galaxy {
                        Some((x as i64, y as i64))
                    } else {
                        None
                    }
                },
            )
            .collect()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Sector {
    #[default]
    EmptySpace,
    Galaxy,
}

impl Sector {
    fn try_parse(c: char) -> Result<Self, String> {
        match c {
            '.' => Ok(Sector::EmptySpace),
            '#' => Ok(Sector::Galaxy),
            _ => Err(format!("unexpected character: {}", c)),
        }
    }
}

impl std::fmt::Display for Sector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sector::EmptySpace => write!(f, "."),
            Sector::Galaxy => write!(f, "#"),
        }
    }
}

struct GalaxyPair {
    a: (i64, i64),
    b: (i64, i64),
}

impl GalaxyPair {
    fn true_distance(
        &self,
        empty_row_indices: &Vec<usize>,
        empty_column_indices: &Vec<usize>,
        expansion_scalar: u64,
    ) -> i64 {
        self.true_x_distance(empty_column_indices, expansion_scalar)
            + self.true_y_distance(empty_row_indices, expansion_scalar)
    }

    fn true_x_distance(&self, empty_column_indices: &Vec<usize>, expansion_scalar: u64) -> i64 {
        let min_x = self.a.0.min(self.b.0) as usize;
        let max_x = self.a.0.max(self.b.0) as usize;
        let crossed_expanded_columns = empty_column_indices
            .iter()
            .filter(|x| min_x < **x && **x < max_x)
            .count() as u64;

        (max_x - min_x - crossed_expanded_columns as usize) as i64
            + (crossed_expanded_columns * expansion_scalar) as i64
    }

    fn true_y_distance(&self, empty_row_indices: &Vec<usize>, expansion_scalar: u64) -> i64 {
        let min_y = self.a.1.min(self.b.1) as usize;
        let max_y = self.a.1.max(self.b.1) as usize;
        let crossed_expanded_rows = empty_row_indices
            .iter()
            .filter(|y| min_y < **y && **y < max_y)
            .count() as u64;

        (max_y - min_y - crossed_expanded_rows as usize) as i64
            + (crossed_expanded_rows * expansion_scalar) as i64
    }
}
