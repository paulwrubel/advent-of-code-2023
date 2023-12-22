use core::fmt;
use std::{collections::HashSet, fs, time};

use itertools::Itertools;

use crate::{
    utils::{CardinalDirection, Grid, Point2D, QuadraticEquation},
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

    map.step(steps)?;
    let possibilities = map.num_possible_locations();

    // println!("after {} steps:\n{}\n", steps, map);

    Ok(possibilities.to_string())
}

// Take 1: 639051584885908.5
// Take 2: 639051580070841 (CORRECT)
// Take 3:  (OPTIMIZED)

fn part_two() -> Result<String, AdventError> {
    part_two_stepping()
    // part_two_pathfinding()
}

fn part_two_stepping() -> Result<String, AdventError> {
    // read input file
    let input = fs::read_to_string(INPUT_FILE)?;

    let mut map = Map::parse(&input)?;
    let steps_per_data_point = 2 * map.tiles.width() as u64;
    let pre_steps = map.tiles.width() as u64 / 2;

    // println!("{}", map);

    map.tile(4)?;

    // println!("{}", map);

    let mut total_steps = 0;
    let mut data_points = [Point2D::default(); 3];

    // pre-step
    let start = time::Instant::now();
    map.step(pre_steps)?;
    println!("pre-step took {:?}", start.elapsed());
    total_steps += pre_steps;

    for i in 0..3 {
        if i > 0 {
            map.step(steps_per_data_point)?;
            total_steps += steps_per_data_point;
        }
        let data_point_y = map.num_possible_locations() as f64;
        let data_point = Point2D::new(total_steps as f64, data_point_y);
        println!("{}: {}", i, data_point);
        data_points[i as usize] = data_point;
    }

    let quadratic = QuadraticEquation::from_points(&data_points)?;

    let solution = quadratic.solve_for_y(26501365.0).round();

    Ok(solution.to_string())
}

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
    possible_locations: Grid<bool>,
    steps_taken: u64,
}

impl Map {
    fn parse(input: &str) -> Result<Self, String> {
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();

        let mut possible_locations = Grid::new_empty(width, height);
        let mut tiles = Grid::new_empty(width, height);
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let tile = Tile::parse(c)?;
                tiles.set(&(x as i64, y as i64).into(), tile)?;

                if tile == Tile::Start {
                    possible_locations.set(&(x as i64, y as i64).into(), true)?;
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
        horizontal.possible_locations =
            Grid::new_empty(horizontal.tiles.width(), horizontal.tiles.height());

        for _ in 0..times {
            // tile horizontally
            self.tiles
                .append_in_direction(&CardinalDirection::East, horizontal.tiles.clone())?;
            self.possible_locations.append_in_direction(
                &CardinalDirection::East,
                horizontal.possible_locations.clone(),
            )?;

            self.tiles
                .append_in_direction(&CardinalDirection::West, horizontal.tiles.clone())?;
            self.possible_locations.append_in_direction(
                &CardinalDirection::West,
                horizontal.possible_locations.clone(),
            )?;
        }

        let mut vertical = self.clone();
        vertical.tiles.set(&start_location, Tile::GardenPlot)?;
        vertical.possible_locations =
            Grid::new_empty(vertical.tiles.width(), vertical.tiles.height());

        for _ in 0..times {
            // tile vertically
            self.tiles
                .append_in_direction(&CardinalDirection::North, vertical.tiles.clone())?;
            self.possible_locations.append_in_direction(
                &CardinalDirection::North,
                vertical.possible_locations.clone(),
            )?;

            self.tiles
                .append_in_direction(&CardinalDirection::South, vertical.tiles.clone())?;
            self.possible_locations.append_in_direction(
                &CardinalDirection::South,
                vertical.possible_locations.clone(),
            )?;
        }
        Ok(())
    }

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
        self.possible_locations
            .entries_matching(|is_possible| *is_possible)
            .count()
    }

    fn step(&mut self, times: u64) -> Result<(), String> {
        for _ in 0..times {
            self.step_once()?;
        }
        Ok(())
    }

    fn step_once(&mut self) -> Result<(), String> {
        // get current locations
        let start = time::Instant::now();
        let current_locations = self
            .possible_locations
            .entries_matching(|is_possible| *is_possible)
            .map(|entry| entry.point)
            .collect_vec();
        let elapsed = start.elapsed();
        // println!("getting current locations took {:?}", elapsed);

        // get all possible next locations, unfiltered
        //
        // this will include locations over rocks and off the map
        let start = time::Instant::now();
        let mut next_locations = HashSet::new();
        for location in current_locations.iter() {
            for neighbor in location.orthogonal_neighbors() {
                if self.tiles.is_within_bounds(&neighbor)
                    && *self.tiles.must_get(&neighbor) != Tile::Rocks
                {
                    next_locations.insert(neighbor);
                }
            }
            self.possible_locations.set(location, false)?;
        }
        let elapsed = start.elapsed();
        // println!("getting next locations took {:?}", elapsed);

        // // filter out locations that are not possible
        // let start = time::Instant::now();
        // let next_locations = next_locations.into_iter().filter(|location| {
        //     // must be on the map and not be rocks
        // });
        // let elapsed = start.elapsed();
        // println!("filtering next locations took {:?}", elapsed);

        // reset current locations
        // self.possible_locations
        //     .set_all_matching(|is_possible| *is_possible, false)?;

        // set next locations
        for location in next_locations {
            self.possible_locations.set(&location, true)?;
        }

        // update steps
        self.steps_taken += 1;

        Ok(())
    }

    fn get_display_string(&self) -> Result<String, String> {
        let mut display_grid = self.tiles.clone().map_all(|tile| match tile {
            Tile::Start => 'S',
            Tile::GardenPlot => '.',
            Tile::Rocks => '#',
        })?;
        for point in self
            .possible_locations
            .entries_matching(|is_possible| *is_possible)
        {
            display_grid.set(&point.point, 'O')?;
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
