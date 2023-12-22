use core::fmt;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

use crate::{
    utils::{CardinalDirection, Grid, GridPoint, Point2D, QuadraticEquation},
    AdventError, ExclusivePart,
};

const INPUT_FILE: &str = "./resources/day21_input.txt";

pub fn run(epart: ExclusivePart) -> Result<String, AdventError> {
    match epart {
        ExclusivePart::One => part_one(),
        ExclusivePart::Two => part_two(),
    }
}

fn part_one() -> Result<String, AdventError> {
    // read input file
    let input = fs::read_to_string(INPUT_FILE)?;

    let steps = 64;

    let mut map = Map::parse(&input)?;

    // let possibilities = map.num_possible_locations_pathfinding(steps)?;

    map.step_bulk(steps)?;
    let possibilities = map.num_possible_locations();

    // println!("after {} steps:\n{}\n", steps, map);

    Ok(possibilities.to_string())
}

// Take 1: 639051584885908.5
// Take 2: 639051580070841 (CORRECT)
// Take 3: 639051580070841 (OPTIMIZED) [also, really more like take 20]

fn part_two() -> Result<String, AdventError> {
    part_two_stepping()
}

fn part_two_stepping() -> Result<String, AdventError> {
    // read input file
    let input = fs::read_to_string(INPUT_FILE)?;

    let mut map = Map::parse(&input)?;
    let steps_per_data_point = 2 * map.tiles.width() as u64;
    let pre_steps = map.tiles.width() as u64 / 2;

    map.tile(4)?;

    let mut total_steps = 0;
    let mut data_points = [Point2D::default(); 3];

    map.step_bulk(pre_steps)?;
    total_steps += pre_steps;

    for i in 0..3 {
        if i > 0 {
            map.step_bulk(steps_per_data_point)?;
            total_steps += steps_per_data_point;
        }
        let data_point_y = map.num_possible_locations() as f64;
        let data_point = Point2D::new(total_steps as f64, data_point_y);
        data_points[i as usize] = data_point;
    }

    let quadratic = QuadraticEquation::from_points(&data_points)?;

    let solution = quadratic.solve_for_y(26501365.0).round();

    Ok(solution.to_string())
}

#[allow(dead_code)]
fn part_two_pathfinding() -> Result<String, AdventError> {
    // read input file
    let input = fs::read_to_string(INPUT_FILE)?;

    let mut map = Map::parse(&input)?;

    let width = map.tiles.width() as u64;
    let x_points = (0..3).map(|i| (width / 2) + (i * width));

    map.tile(2)?;

    let mut data_points = [Point2D::default(); 3];

    let mut cache = Grid::new_empty(map.tiles.width(), map.tiles.height());
    cache.set_top_left(map.tiles.top_left().clone());
    for (i, x) in x_points.enumerate() {
        // let y = map.num_possible_locations_pathfinding(x)?;
        let y = map.num_possible_locations_pathfinding_with_cache(x, &mut cache)?;
        data_points[i as usize] = Point2D::new(x as f64, y as f64);
    }

    let quadratic = QuadraticEquation::from_points(&data_points)?;

    let solution = quadratic.solve_for_y(26501365.0).round();

    Ok(solution.to_string())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    tiles: Grid<Tile>,
    possible_locations: HashSet<GridPoint>,
    steps_taken: u64,
}

impl Map {
    fn parse(input: &str) -> Result<Self, String> {
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();

        let mut possible_locations = HashSet::new();
        let mut tiles = Grid::new_empty(width, height);
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let tile = Tile::parse(c)?;
                tiles.set(&(x as i64, y as i64).into(), tile)?;

                if tile == Tile::Start {
                    possible_locations.insert((x as i64, y as i64).into());
                }
            }
        }

        Ok(Map {
            tiles,
            possible_locations,
            steps_taken: 0,
        })
    }

    fn tile(&mut self, times: u64) -> Result<(), String> {
        let start_location = self
            .tiles
            .find(|tile| *tile == Tile::Start)
            .map(|entry| entry.point)
            .ok_or(format!("No start location"))?;

        let mut horizontal = self.clone();
        horizontal.tiles.set(&start_location, Tile::GardenPlot)?;
        horizontal.possible_locations.clear();

        for _ in 0..times {
            // tile horizontally
            self.tiles
                .append_in_direction(&CardinalDirection::East, horizontal.tiles.clone())?;
            // self.possible_locations.append_in_direction(
            //     &CardinalDirection::East,
            //     horizontal.possible_locations.clone(),
            // )?;

            self.tiles
                .append_in_direction(&CardinalDirection::West, horizontal.tiles.clone())?;
            // self.possible_locations.append_in_direction(
            //     &CardinalDirection::West,
            //     horizontal.possible_locations.clone(),
            // )?;
        }

        let mut vertical = self.clone();
        vertical.tiles.set(&start_location, Tile::GardenPlot)?;
        // vertical.possible_locations =
        //     Grid::new_empty(vertical.tiles.width(), vertical.tiles.height());

        for _ in 0..times {
            // tile vertically
            self.tiles
                .append_in_direction(&CardinalDirection::North, vertical.tiles.clone())?;
            // self.possible_locations.append_in_direction(
            //     &CardinalDirection::North,
            //     vertical.possible_locations.clone(),
            // )?;

            self.tiles
                .append_in_direction(&CardinalDirection::South, vertical.tiles.clone())?;
            // self.possible_locations.append_in_direction(
            //     &CardinalDirection::South,
            //     vertical.possible_locations.clone(),
            // )?;
        }
        Ok(())
    }

    #[allow(dead_code)]
    fn num_possible_locations_pathfinding(&self, steps: u64) -> Result<u64, String> {
        let start = self
            .tiles
            .find(|tile| *tile == Tile::Start)
            .ok_or("No start location")?
            .point;
        let mut possibilities = 0;
        for entry in self.tiles.entries_matching(|tile| *tile != Tile::Rocks) {
            // check if we even need to both pathfinding
            // since we definitely could only reach tiles
            // that are AT MOST the manhattan distance steps away
            if entry.point.manhattan_distance_to(&start) <= steps {
                // it might be possible, so we have to try pathfinding
                let true_distance =
                    match self
                        .tiles
                        .astar_distance_othogonal(&entry.point, &start, |tile| *tile != Tile::Rocks)
                    {
                        Some(distance) => distance,
                        None => {
                            // no path? no way we could step here!
                            continue;
                        }
                    };

                if true_distance <= steps && true_distance % 2 == steps % 2 {
                    possibilities += 1;
                }
            }
        }
        Ok(possibilities)
    }

    fn num_possible_locations_pathfinding_with_cache(
        &self,
        steps: u64,
        cache: &mut Grid<Option<u64>>,
    ) -> Result<u64, String> {
        let start = self
            .tiles
            .find(|tile| *tile == Tile::Start)
            .ok_or("No start location")?
            .point;
        let mut possibilities = 0;
        println!("pathfinding with cache: {} steps", steps);
        for entry in self.tiles.entries_matching(|tile| *tile != Tile::Rocks) {
            // check if we even need to both pathfinding
            // since we definitely could only reach tiles
            // that are AT MOST the manhattan distance steps away
            let manhattan_distance = entry.point.manhattan_distance_to(&start);
            if manhattan_distance <= steps {
                println!(
                    "\tdoing the hard work! from {} to {} (md: {})",
                    entry.point, start, manhattan_distance
                );
                // it might be possible, so we have to try pathfinding
                let true_distance = match self.tiles.astar_distance_othogonal_with_cache(
                    &entry.point,
                    &start,
                    |tile| *tile != Tile::Rocks,
                    cache,
                ) {
                    Some(distance) => distance,
                    None => {
                        // no path? no way we could step here!
                        continue;
                    }
                };

                if true_distance <= steps && true_distance % 2 == steps % 2 {
                    possibilities += 1;
                }
            }
        }
        Ok(possibilities)
    }

    fn num_possible_locations(&self) -> usize {
        self.possible_locations.len()
    }

    fn step_bulk(&mut self, times: u64) -> Result<(), String> {
        let mut steppable = HashMap::new();
        for location in self.possible_locations.iter() {
            steppable.insert(*location, 0);
        }

        let mut frontier = self.possible_locations.clone();
        let mut next_frontier = HashSet::new();
        for step_count in 0..times {
            for location in frontier.drain() {
                for neighbor in location.orthogonal_neighbors() {
                    if self.tiles.is_within_bounds(&neighbor)
                        && *self.tiles.must_get(&neighbor) != Tile::Rocks
                        && !steppable.contains_key(&neighbor)
                    {
                        next_frontier.insert(neighbor);
                    }
                }
                steppable.insert(location, step_count);
            }
            frontier.extend(next_frontier.drain());
        }
        for location in frontier.drain() {
            steppable.insert(location, times);
        }

        self.possible_locations.clear();
        for (location, step_count) in steppable {
            if step_count % 2 == times % 2 {
                self.possible_locations.insert(location);
            }
        }

        Ok(())
    }

    fn get_display_string(&self) -> Result<String, String> {
        let mut display_grid = self.tiles.clone().map_all(|tile| match tile {
            Tile::Start => 'S',
            Tile::GardenPlot => '.',
            Tile::Rocks => '#',
        })?;
        for point in self.possible_locations.iter() {
            display_grid.set(&point, 'O')?;
        }

        Ok(display_grid.to_string())
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.get_display_string()
                .map_err(|_| fmt::Error::default())?
        )
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Start,
    #[default]
    GardenPlot,
    Rocks,
}

impl Tile {
    fn parse(c: char) -> Result<Self, String> {
        match c {
            'S' => Ok(Tile::Start),
            '.' => Ok(Tile::GardenPlot),
            '#' => Ok(Tile::Rocks),
            _ => Err(format!("Invalid tile: {}", c)),
        }
    }
}
