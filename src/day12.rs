use std::{collections::HashMap, fs};

use itertools::Itertools;

use crate::{AdventError, ExclusivePart};

const INPUT_FILE: &str = "./resources/day12_input.txt";

pub fn run(epart: ExclusivePart) -> Result<String, AdventError> {
    match epart {
        ExclusivePart::One => part_one(),
        ExclusivePart::Two => part_two(),
    }
}

fn part_one() -> Result<String, AdventError> {
    // read input file
    let input = fs::read_to_string(INPUT_FILE)?;

    let debug = false;
    let brute_force = false;

    let spring_rows = input
        .lines()
        .map(|line| SpringRow::parse(line))
        .collect::<Result<Vec<SpringRow>, String>>()?;

    let mut sum_of_possible_arrangements = 0;
    for (_i, row) in spring_rows.iter().enumerate() {
        sum_of_possible_arrangements += row.possible_arrangements_count(debug, brute_force);
    }

    Ok(sum_of_possible_arrangements.to_string())
}

fn part_two() -> Result<String, AdventError> {
    // read input file
    let input = fs::read_to_string(INPUT_FILE)?;

    let debug = false;
    let brute_force = false;
    let times = 5;

    let spring_rows = input
        .lines()
        .map(|line| SpringRow::parse(line))
        .collect::<Result<Vec<SpringRow>, String>>()?;

    let spring_rows: Vec<SpringRow> = spring_rows
        .into_iter()
        .map(|row| row.into_unfolded(times))
        .collect();

    let mut sum_of_possible_arrangements = 0;
    for (_i, row) in spring_rows.iter().enumerate() {
        sum_of_possible_arrangements += row.possible_arrangements_count(debug, brute_force);
    }

    Ok(sum_of_possible_arrangements.to_string())
}

#[derive(Debug, Clone)]
struct SpringRow {
    springs: Vec<SpringCondition>,
    damaged_spring_groups: Vec<u64>,
}

impl SpringRow {
    fn parse(input: &str) -> Result<SpringRow, String> {
        let (springs, damaged_spring_groups) = input
            .split_once(' ')
            .ok_or("invalid input: no space in row")?;

        let springs: Vec<SpringCondition> = springs
            .chars()
            .map(|c| SpringCondition::parse(c))
            .collect::<Result<Vec<SpringCondition>, String>>()?;

        let damaged_spring_groups: Vec<u64> = damaged_spring_groups
            .split(',')
            .map(|s| s.parse())
            .collect::<Result<Vec<u64>, _>>()
            .map_err(|err| err.to_string())?;

        Ok(SpringRow {
            springs,
            damaged_spring_groups,
        })
    }

    fn into_unfolded(self, times: u64) -> Self {
        let mut springs_unfolded = Vec::new();
        let mut damaged_spring_groups_unfolded = Vec::new();
        for _ in 0..times {
            springs_unfolded.push(self.springs.clone());
            damaged_spring_groups_unfolded.append(&mut self.damaged_spring_groups.clone());
        }

        let springs_unfolded: Vec<SpringCondition> =
            Itertools::intersperse(springs_unfolded.into_iter(), vec![SpringCondition::Unknown])
                .flatten()
                .collect();

        Self {
            springs: springs_unfolded,
            damaged_spring_groups: damaged_spring_groups_unfolded,
        }
    }

    fn possible_arrangements_count(&self, debug: bool, brute_force: bool) -> u64 {
        if brute_force {
            self.possible_arrangements_brute_force().len() as u64
        } else {
            self.possible_arrangements_count_progressive_memoization(debug)
        }
    }

    fn possible_arrangements_count_progressive_memoization(&self, debug: bool) -> u64 {
        let mut memo_table = HashMap::new();
        self.possible_arrangements_at(
            0,
            debug,
            0,
            self.damaged_spring_groups.clone(),
            &mut memo_table,
        )
    }

    fn possible_arrangements_brute_force(&self) -> Vec<KnownSpringRow> {
        let total_damaged_springs = self.damaged_spring_groups.iter().sum::<u64>();
        let total_known_damaged_springs = self
            .springs
            .iter()
            .filter(|&s| *s == SpringCondition::Known(KnownSpringCondition::Damaged))
            .count();

        let num_placeable = total_damaged_springs as usize - total_known_damaged_springs;

        let unknown_positions = self.springs.iter().enumerate().filter_map(|(i, s)| {
            if *s == SpringCondition::Unknown {
                Some(i)
            } else {
                None
            }
        });

        let possible_damage_placement_indices =
            unknown_positions.into_iter().combinations(num_placeable);

        let possible_damaged_placements =
            possible_damage_placement_indices.map(|damaged_placement_indices| {
                KnownSpringRow::from_spring_row_and_damaged_placement_indices(
                    self,
                    &damaged_placement_indices,
                )
                .unwrap()
            });

        let valid_damaged_placements = possible_damaged_placements
            .into_iter()
            .filter(|row| row.is_valid());

        valid_damaged_placements.collect()
    }

    fn possible_arrangements_at(
        &self,
        level: usize,
        debug: bool,
        index: usize,
        groups: Vec<u64>,
        memo_table: &mut HashMap<(usize, usize), u64>,
    ) -> u64 {
        let indent = 2;

        if debug {
            println!(
                "{}checking {} with {} groups",
                " ".repeat(level),
                index,
                groups.len()
            );
        }
        if groups.len() == 0 {
            // we have nothing left to place!
            if index == 0
                || self
                    .springs
                    .iter()
                    .skip(index - 1)
                    .any(|s| *s == SpringCondition::Known(KnownSpringCondition::Damaged))
            {
                // if there are any damaged springs left in the list, but there's nothing left
                // for us to place, we can't make an arrangement
                if debug {
                    println!(
                        "{}nothing left to place at {}, but there's still '#'! returning 0",
                        " ".repeat(level),
                        index
                    );
                }
                return 0;
            } else {
                // if there AREN'T any damaged springs left in the list, we return
                // 1 as a sort-of "base case", indicating that whatever arrangement led us here is valid
                if debug {
                    println!(
                        "{}nothing left to place at {}. we're done here! returning 1",
                        " ".repeat(level),
                        index
                    );
                }
                return 1;
            }
        }

        if index >= self.springs.len() {
            // we're off the row!
            // there's no possible arrangement that we can make
            if debug {
                println!("{}off the row at {}! returning 0", " ".repeat(level), index);
            }
            return 0;
        }

        let spring = self.springs[index];
        if spring == SpringCondition::Known(KnownSpringCondition::Operational) {
            // if we're operational, there's no info to learn here
            // so just return whatever arrangements are available starting 1 after us
            let next_arrangements =
                self.possible_arrangements_at(level + indent, debug, index + 1, groups, memo_table);
            if debug {
                println!(
                    "{}found '.' at {}, using next arrangements ({})",
                    " ".repeat(level),
                    index,
                    next_arrangements
                );
            }
            return next_arrangements;
        }

        // the input validation and base-cases are done! time for some calculation...

        // but first, let's see if we've already calculated this arrangement
        if let Some(arrangements_count) = memo_table.get(&(index, groups.len())) {
            if debug {
                println!(
                    "{}found {} arrangements at {} with {} groups (from cache)",
                    " ".repeat(level),
                    arrangements_count,
                    index,
                    groups.len()
                );
            }
            return *arrangements_count;
        }

        // okay, now it's ACTUALLY time to calculate this arrangement

        // since we know this index is either Unknown or Damaged, we can get the
        // number of possible arrangements assuming we consume the first group here
        let first_group_len = groups[0] as usize;
        let mut can_consume = true;
        for i in index..first_group_len + index {
            if i >= self.springs.len() {
                can_consume = false;
                break;
            }
            let spring = self.springs[i];
            if spring == SpringCondition::Known(KnownSpringCondition::Operational) {
                can_consume = false;
                break;
            }
        }
        if can_consume
            && index + first_group_len < self.springs.len()
            && self.springs[index + first_group_len]
                == SpringCondition::Known(KnownSpringCondition::Damaged)
        {
            can_consume = false;
        }
        let consume_arrangements_count = if can_consume {
            self.possible_arrangements_at(
                level + indent,
                debug,
                index + first_group_len + 1,
                groups[1..].to_vec(),
                memo_table,
            )
        } else {
            0
        };

        // in addition to that, we should check if we are Unknown specifically, since
        // that means we could possible just skip this index and check
        // the next one, consuming nothing.
        let skip_arrangements_count = if spring == SpringCondition::Unknown {
            self.possible_arrangements_at(
                level + indent,
                debug,
                index + 1,
                groups.clone(),
                memo_table,
            )
        } else {
            0
        };

        let arrangements_count = consume_arrangements_count + skip_arrangements_count;
        if debug {
            println!(
                "{}found {} arrangements at {} with {} groups ({} consume + {} skip)",
                " ".repeat(level),
                arrangements_count,
                index,
                groups.len(),
                consume_arrangements_count,
                skip_arrangements_count
            );
        }

        // now that we've calculated this arrangement, we can cache it
        if debug {
            println!(
                "{}saving {} arrangements at {} with {} groups",
                " ".repeat(level),
                arrangements_count,
                index,
                groups.len()
            );
        }
        memo_table.insert((index, groups.len()), arrangements_count);

        // and we're done
        arrangements_count
    }
}

impl std::fmt::Display for SpringRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.springs.iter() {
            write!(f, "{}", c)?;
        }
        write!(f, " ")?;
        let comma_string = ",".to_string();
        for string in Itertools::intersperse(
            self.damaged_spring_groups.iter().map(|g| g.to_string()),
            comma_string,
        ) {
            write!(f, "{}", string)?;
        }
        Ok(())
    }
}

struct KnownSpringRow {
    springs: Vec<KnownSpringCondition>,
    damaged_spring_groups: Vec<u64>,
}

impl KnownSpringRow {
    fn from_spring_row_and_damaged_placement_indices(
        row: &SpringRow,
        placement_indices: &Vec<usize>,
    ) -> Result<KnownSpringRow, String> {
        let mut springs = row.springs.clone();

        // fill the unknowns with the damaged springs at the specified positions
        for i in placement_indices {
            if springs[*i] == SpringCondition::Unknown {
                springs[*i] = SpringCondition::Known(KnownSpringCondition::Damaged);
            } else {
                return Err(format!(
                    "invalid placement: cannot place damaged spring onto known index: {}",
                    i
                ));
            }
        }

        // replace the remaining unknowns with operational springs
        let springs: Vec<KnownSpringCondition> = springs
            .iter()
            .map(|s| match s {
                SpringCondition::Known(k) => *k,
                SpringCondition::Unknown => KnownSpringCondition::Operational,
            })
            .collect();

        // everything is known!
        Ok(Self {
            springs,
            damaged_spring_groups: row.damaged_spring_groups.clone(),
        })
    }

    fn is_valid(&self) -> bool {
        let mut damage_group_lengths = Vec::new();
        for (is_damage_group, group) in &self
            .springs
            .iter()
            .group_by(|s| **s == KnownSpringCondition::Damaged)
        {
            if is_damage_group {
                damage_group_lengths.push(group.count() as u64);
            }
        }

        if damage_group_lengths.len() != self.damaged_spring_groups.len() {
            return false;
        }

        for i in 0..self.damaged_spring_groups.len() {
            if damage_group_lengths[i] != self.damaged_spring_groups[i] {
                return false;
            }
        }

        true
    }
}

impl std::fmt::Display for KnownSpringRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.springs.iter() {
            write!(f, "{}", c)?;
        }
        write!(f, " ")?;
        let comma_string = ",".to_string();
        for string in Itertools::intersperse(
            self.damaged_spring_groups.iter().map(|g| g.to_string()),
            comma_string,
        ) {
            write!(f, "{}", string)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpringCondition {
    Unknown,
    Known(KnownSpringCondition),
}

impl SpringCondition {
    fn parse(c: char) -> Result<SpringCondition, String> {
        match c {
            '?' => Ok(Self::Unknown),
            _ => {
                let known = KnownSpringCondition::parse(c)?;
                Ok(Self::Known(known))
            }
        }
    }
}

impl std::fmt::Display for SpringCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpringCondition::Unknown => write!(f, "?"),
            SpringCondition::Known(k) => write!(f, "{}", k),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum KnownSpringCondition {
    Operational,
    Damaged,
}

impl KnownSpringCondition {
    fn parse(c: char) -> Result<KnownSpringCondition, String> {
        match c {
            '.' => Ok(KnownSpringCondition::Operational),
            '#' => Ok(KnownSpringCondition::Damaged),
            _ => Err(format!("invalid spring condition: {}", c)),
        }
    }
}

impl std::fmt::Display for KnownSpringCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KnownSpringCondition::Operational => write!(f, "."),
            KnownSpringCondition::Damaged => write!(f, "#"),
        }
    }
}
