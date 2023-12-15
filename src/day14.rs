use std::{
    fs,
    hash::{Hash, Hasher},
};

use crate::{utils::Grid, AdventError, ExclusivePart};

const INPUT_FILE: &str = "./resources/day14_input.txt";

pub fn run(epart: ExclusivePart) -> Result<String, AdventError> {
    match epart {
        ExclusivePart::One => part_one(),
        ExclusivePart::Two => part_two(),
    }
}

fn part_one() -> Result<String, AdventError> {
    // read input file
    let input = fs::read_to_string(INPUT_FILE)?;

    let platform = Platform::parse(&input)?;
    let platform = platform.tilt_in_cardinal_direction(CardinalDirection::North)?;

    let rounded_load = platform.load_from_rounded_rocks();
    Ok(rounded_load.to_string())
}

fn part_two() -> Result<String, AdventError> {
    // read input file
    let input = fs::read_to_string(INPUT_FILE)?;

    let cycles = 1_000_000_000;

    let platform = Platform::parse(&input)?;
    let mut platform_cycler = PlatformCycler::new(platform);
    let cycled_platform = platform_cycler.cycle_n_times(cycles)?;

    let rounded_load = cycled_platform.load_from_rounded_rocks();
    Ok(rounded_load.to_string())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Platform {
    spaces: Grid<Space>,
}

impl Platform {
    fn parse(input: &str) -> Result<Self, String> {
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();

        let mut spaces = Grid::new_empty(width, height);
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                spaces.set(x as i64, y as i64, Space::parse(c)?)?;
            }
        }

        Ok(Self { spaces })
    }

    fn cycle(&self) -> Result<Self, String> {
        self.tilt_in_cardinal_direction(CardinalDirection::North)?
            .tilt_in_cardinal_direction(CardinalDirection::West)?
            .tilt_in_cardinal_direction(CardinalDirection::South)?
            .tilt_in_cardinal_direction(CardinalDirection::East)
    }

    fn tilt_in_cardinal_direction(&self, direction: CardinalDirection) -> Result<Self, String> {
        let mut new_spaces = self.spaces.clone();
        match direction {
            CardinalDirection::North | CardinalDirection::South => {
                for (x, column) in self.spaces.columns_iter().enumerate() {
                    let column = column.copied().collect();
                    let tilted_column = match direction {
                        CardinalDirection::North => self.tilt_column_north(column),
                        CardinalDirection::South => self.tilt_column_south(column),
                        _ => unreachable!(),
                    }?;

                    for (y, space) in tilted_column.iter().enumerate() {
                        new_spaces.set(x as i64, y as i64, *space)?;
                    }
                }
            }
            CardinalDirection::East | CardinalDirection::West => {
                for (y, row) in self.spaces.rows_iter().enumerate() {
                    let row = row.copied().collect();
                    let tilted_row = match direction {
                        CardinalDirection::East => self.tilt_row_east(row),
                        CardinalDirection::West => self.tilt_row_west(row),
                        _ => unreachable!(),
                    }?;

                    for (x, space) in tilted_row.iter().enumerate() {
                        new_spaces.set(x as i64, y as i64, *space)?;
                    }
                }
            }
        }

        Ok(Self { spaces: new_spaces })
    }

    fn tilt_column_north(&self, column: Vec<Space>) -> Result<Vec<Space>, String> {
        let mut tilted_column = column.clone();
        for y in 0..column.len() {
            let space = &tilted_column[y];
            if space == &Space::RoundRock {
                let mut first_empty_index = None;

                // find the final empty index north of this index
                let check_vec = &tilted_column[..y];
                for (y, space) in check_vec.iter().enumerate().rev() {
                    if *space == Space::Empty {
                        first_empty_index = Some(y);
                    } else {
                        break;
                    }
                }

                if let Some(first_empty_index) = first_empty_index {
                    // swap the empty index with this index
                    tilted_column[first_empty_index] = Space::RoundRock;
                    tilted_column[y] = Space::Empty;
                }
            }
        }
        Ok(tilted_column)
    }

    fn tilt_column_south(&self, column: Vec<Space>) -> Result<Vec<Space>, String> {
        let mut tilted_column = column.clone();
        for y in (0..column.len()).rev() {
            let space = &tilted_column[y];
            if space == &Space::RoundRock {
                let mut first_empty_index = None;

                // find the final empty index south of this index
                for (y, space) in tilted_column.iter().enumerate().skip(y + 1) {
                    if *space == Space::Empty {
                        first_empty_index = Some(y);
                    } else {
                        break;
                    }
                }

                if let Some(first_empty_index) = first_empty_index {
                    // swap the empty index with this index
                    tilted_column[first_empty_index] = Space::RoundRock;
                    tilted_column[y] = Space::Empty;
                }
            }
        }
        Ok(tilted_column)
    }

    fn tilt_row_east(&self, row: Vec<Space>) -> Result<Vec<Space>, String> {
        let mut tilted_row = row.clone();
        for x in (0..row.len()).rev() {
            let space = &tilted_row[x];
            if space == &Space::RoundRock {
                let mut first_empty_index = None;

                // find the final empty index east of this index
                for (x, space) in tilted_row.iter().enumerate().skip(x + 1) {
                    if *space == Space::Empty {
                        first_empty_index = Some(x);
                    } else {
                        break;
                    }
                }

                if let Some(first_empty_index) = first_empty_index {
                    // swap the empty index with this index
                    tilted_row[first_empty_index] = Space::RoundRock;
                    tilted_row[x] = Space::Empty;
                }
            }
        }
        Ok(tilted_row)
    }

    fn tilt_row_west(&self, row: Vec<Space>) -> Result<Vec<Space>, String> {
        let mut tilted_row = row.clone();
        for x in 0..row.len() {
            let space = &tilted_row[x];
            if space == &Space::RoundRock {
                let mut first_empty_index = None;

                // find the final empty index east of this index
                let check_vec = &tilted_row[..x];
                for (x, space) in check_vec.iter().enumerate().rev() {
                    if *space == Space::Empty {
                        first_empty_index = Some(x);
                    } else {
                        break;
                    }
                }

                if let Some(first_empty_index) = first_empty_index {
                    // swap the empty index with this index
                    tilted_row[first_empty_index] = Space::RoundRock;
                    tilted_row[x] = Space::Empty;
                }
            }
        }
        Ok(tilted_row)
    }

    fn load_from_rounded_rocks(&self) -> i64 {
        let rows_rev = self.spaces.rows_iter().rev();

        let mut load = 0;
        for (i, row) in rows_rev.enumerate() {
            let rounded_in_row = row.filter(|space| **space == Space::RoundRock).count();

            load += (rounded_in_row as i64) * (i as i64 + 1);
        }

        load
    }
}

struct PlatformCycler {
    platform_progression: Vec<Platform>,
}

impl PlatformCycler {
    fn new(starting_platform: Platform) -> Self {
        Self {
            platform_progression: vec![starting_platform],
        }
    }

    fn cycle_n_times(&mut self, n: usize) -> Result<Platform, String> {
        for _ in 0..n {
            let current_platform = self.platform_progression.last().unwrap();
            let next_cycled_platform = current_platform.cycle()?;
            if let Some(pos) = self
                .platform_progression
                .iter()
                .position(|p| p.clone() == next_cycled_platform)
            {
                let cycle_length = self.platform_progression.len() - pos;
                let cycle_offset = ((n - pos) % cycle_length) + pos;
                return Ok(self.platform_progression[cycle_offset].clone());
            }
            self.platform_progression.push(next_cycled_platform);
        }
        Ok(self.platform_progression.last().unwrap().clone())
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.spaces.rows_iter().enumerate() {
            for space in row {
                write!(f, "{}", space)?;
            }
            if y != self.spaces.height() - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Hash for Platform {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let str = self.to_string();
        str.hash(state);
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Space {
    RoundRock,
    SquareRock,
    #[default]
    Empty,
}

impl Space {
    fn parse(c: char) -> Result<Self, String> {
        match c {
            'O' => Ok(Self::RoundRock),
            '#' => Ok(Self::SquareRock),
            '.' => Ok(Self::Empty),
            _ => Err(format!("unexpected character: {}", c)),
        }
    }
}

impl std::fmt::Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RoundRock => write!(f, "O"),
            Self::SquareRock => write!(f, "#"),
            Self::Empty => write!(f, "."),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CardinalDirection {
    North,
    South,
    East,
    West,
}
