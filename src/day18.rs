use std::fs;

use itertools::Itertools;

use crate::{
    utils::{
        CardinalDirection, Grid, GridEntry, GridPoint, OrdinalDirection, RelativeDirection, Winding,
    },
    AdventError, ExclusivePart,
};

const INPUT_FILE: &str = "./resources/day18_input.txt";

pub fn run(epart: ExclusivePart) -> Result<String, AdventError> {
    match epart {
        ExclusivePart::One => part_one(),
        ExclusivePart::Two => part_two(),
    }
}

fn part_one() -> Result<String, AdventError> {
    // read input file
    let input = fs::read_to_string(INPUT_FILE)?;

    let dig_plan = DigPlan::parse(&input, false)?;

    let mut lagoon = Lagoon::parse(&dig_plan)?;

    lagoon.fill_interior()?;

    let num_tiles = lagoon.get_num_dug_out_terrain_tiles();

    Ok(num_tiles.to_string())
}

fn part_two() -> Result<String, AdventError> {
    // read input file
    let input = fs::read_to_string(INPUT_FILE)?;

    println!("parsing plan...");
    let dig_plan = DigPlan::parse(&input, true)?;

    let num_tiles = dig_plan.shoelace_area()?;

    Ok(num_tiles.to_string())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Lagoon {
    map: Grid<TerrainTile>,
    trench_points: Vec<GridPoint>,
    winding: Winding,
}

impl Lagoon {
    fn parse(dig_plan: &DigPlan) -> Result<Self, String> {
        let mut map = Grid::new_empty(1, 1);
        let mut trench_points = Vec::new();

        // start in the top left, in a dug out cube
        let mut current_pos: GridPoint = (0, 0).into();
        map.set(current_pos, TerrainTile::Trench)?;
        trench_points.push(current_pos);

        // dig out the cubes according to the plan
        let previous_direction: Option<CardinalDirection> = None;
        let mut left_turns = 0;
        let mut right_turns = 0;
        for (_i, step) in dig_plan.steps.iter().enumerate() {
            // println!("starting step {}: {:?} as {}", i + 1, step, current_pos);

            if let Some(previous_direction) = previous_direction {
                match previous_direction.relative_direction_to(&step.direction) {
                    RelativeDirection::Left => {
                        left_turns += 1;
                    }
                    RelativeDirection::Right => {
                        right_turns += 1;
                    }
                    _ => {}
                }
            }

            for _ in 0..step.distance {
                current_pos = current_pos.neighbor_in_direction(step.direction);
                map.set_expand(current_pos, TerrainTile::Trench)?;
                trench_points.push(current_pos);
            }
        }

        let winding = if left_turns > right_turns {
            Winding::Left
        } else {
            Winding::Right
        };

        Ok(Lagoon {
            map,
            trench_points,
            winding,
        })
    }

    fn fill_interior(&mut self) -> Result<(), String> {
        for i in 0..self.trench_points.len() - 1 {
            let current = self.trench_points[i];
            let next = self.trench_points[i + 1];

            let direction = current.direction_to_orthogonal(&next)?;
            let offset_direction = direction.turn(self.winding.into());

            let current_offset = current.neighbor_in_direction(offset_direction);
            let next_offset = next.neighbor_in_direction(offset_direction);

            let is_floodable = |t: &TerrainTile| *t == TerrainTile::Surface;

            self.map
                .flood(current_offset, &|| TerrainTile::Interior, &is_floodable)?;
            self.map
                .flood(next_offset, &|| TerrainTile::Interior, &is_floodable)?;
        }
        Ok(())
    }

    fn get_num_dug_out_terrain_tiles(&self) -> usize {
        let mut count = 0;
        for GridEntry { value, .. } in self.map.entries() {
            if *value == TerrainTile::Trench || *value == TerrainTile::Interior {
                count += 1;
            }
        }
        count
    }
}

impl std::fmt::Display for Lagoon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start = self.trench_points[0];
        let direction = start
            .direction_to_orthogonal(&self.trench_points[1])
            .unwrap();
        write!(
            f,
            "start: {}, direction: {}, winding: {}\n{}",
            start, direction, self.winding, self.map
        )
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
enum TerrainTile {
    #[default]
    Surface,
    Trench,
    Interior,
}

impl std::fmt::Display for TerrainTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TerrainTile::Surface => ".",
                TerrainTile::Trench => "#",
                TerrainTile::Interior => "#",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DigPlan {
    steps: Vec<DigPlanStep>,
    corners: Vec<GridPoint>,
    winding: Winding,
}

impl DigPlan {
    fn parse(input: &str, extract_from_color: bool) -> Result<Self, String> {
        let mut steps = Vec::new();
        let mut corners = Vec::new();

        let mut current_position: GridPoint = (0, 0).into();
        let mut previous_direction: Option<CardinalDirection> = None;
        let mut left_turns = 0;
        let mut right_turns = 0;
        for line in input.lines() {
            // parse the actual step
            let step = DigPlanStep::parse(line, extract_from_color)?;

            // push this position as a corner and update the current position
            corners.push(current_position);
            current_position = current_position
                .neighbor_in_direction_distance(step.direction, step.distance as i64);

            // check for winding
            if let Some(previous_direction) = previous_direction {
                match previous_direction.relative_direction_to(&step.direction) {
                    RelativeDirection::Left => {
                        left_turns += 1;
                    }
                    RelativeDirection::Right => {
                        right_turns += 1;
                    }
                    _ => {}
                }
            }
            previous_direction = Some(step.direction);

            steps.push(step);
        }

        let winding = if left_turns > right_turns {
            Winding::Left
        } else {
            Winding::Right
        };

        Ok(DigPlan {
            steps,
            corners,
            winding,
        })
    }

    fn true_corners(&self) -> Result<Vec<GridPoint>, String> {
        let mut true_corners = Vec::new();

        let relative_winding: RelativeDirection = self.winding.into();

        for i in 0..self.corners.len() {
            let previous_corner = if i == 0 {
                &self.corners[self.corners.len() - 1]
            } else {
                &self.corners[i - 1]
            };
            let this_corner = &self.corners[i];
            let next_corner = &self.corners[(i + 1) % self.corners.len()];

            // direction to neighboring corners
            // this determines what kind of corner WE are
            let direction_backwards = this_corner.direction_to_orthogonal(previous_corner)?;
            let direction_forwards = this_corner.direction_to_orthogonal(next_corner)?;

            // this is which way we turned here.
            let relative_direction = direction_backwards
                .opposite()
                .relative_direction_to(&direction_forwards);

            let mut outside_corner = direction_backwards.halfway_to(&direction_forwards)?;

            // if the way we turned matches the winding, it means we turned INWARDS,
            // so we have to shift outwards to include the boundary
            if relative_direction == relative_winding {
                outside_corner = outside_corner.opposite();
            }

            let mut true_corner = this_corner.clone();
            match outside_corner {
                OrdinalDirection::NorthEast => {
                    true_corner.x += 1;
                    true_corner.y -= 1;
                }
                OrdinalDirection::SouthEast => {
                    true_corner.x += 1;
                }
                OrdinalDirection::SouthWest => {}
                OrdinalDirection::NorthWest => {
                    true_corner.y -= 1;
                }
            }

            true_corners.push(true_corner);
        }
        Ok(true_corners)
    }

    fn shoelace_area(&self) -> Result<u64, String> {
        let corners = self.true_corners()?;

        let mut area: i64 = 0;
        for i in 0..corners.len() {
            let j = (i + 1) % corners.len();

            let a = corners[i];
            let b = corners[j];

            area += a.x * b.y - a.y * b.x;
        }

        Ok((area.abs() / 2) as u64)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DigPlanStep {
    direction: CardinalDirection,
    distance: u64,
    color: String,
}

impl DigPlanStep {
    fn parse(input: &str, extract_from_color: bool) -> Result<Self, String> {
        let parts = input.split_whitespace().collect_vec();

        if parts.len() != 3 {
            return Err(format!("invalid input (wrong number of parts): {}", input));
        }

        if extract_from_color {
            Self::parse_from_color(parts)
        } else {
            Self::parse_from_direction_and_distance(parts)
        }
    }

    fn parse_from_direction_and_distance(parts: Vec<&str>) -> Result<Self, String> {
        let direction = match parts[0] {
            "U" => CardinalDirection::North,
            "D" => CardinalDirection::South,
            "L" => CardinalDirection::West,
            "R" => CardinalDirection::East,
            _ => return Err(format!("invalid direction: {}", parts[0])),
        };

        let distance = parts[1].parse::<u64>().map_err(|err| err.to_string())?;

        let color = parts[2][2..8].to_string();

        Ok(DigPlanStep {
            direction,
            distance,
            color,
        })
    }

    fn parse_from_color(parts: Vec<&str>) -> Result<Self, String> {
        // parse as hexadecimal number

        let distance = u64::from_str_radix(&parts[2][2..7], 16).map_err(|err| err.to_string())?;

        let direction_char = parts[2]
            .chars()
            .nth(7)
            .ok_or(format!("invalid color: {}", parts[2]))?;

        let direction = match direction_char {
            '3' => CardinalDirection::North,
            '1' => CardinalDirection::South,
            '2' => CardinalDirection::West,
            '0' => CardinalDirection::East,
            _ => return Err(format!("invalid direction: {}", parts[0])),
        };

        let color = parts[2].to_string();

        Ok(DigPlanStep {
            direction,
            distance,
            color,
        })
    }
}
