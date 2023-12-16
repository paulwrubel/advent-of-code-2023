use std::{collections::HashSet, fs};

use itertools::Itertools;

use crate::{
    utils::{Grid, GridEntry},
    AdventError, ExclusivePart,
};

const INPUT_FILE: &str = "./resources/day16_input.txt";

pub fn run(epart: ExclusivePart) -> Result<String, AdventError> {
    match epart {
        ExclusivePart::One => part_one(),
        ExclusivePart::Two => part_two(),
    }
}

fn part_one() -> Result<String, AdventError> {
    // read input file
    let input = fs::read_to_string(INPUT_FILE)?;

    let contraption = Contraption::parse(&input)?;

    let resolved_contraption = contraption.resolve_beams_starting_from(BeamData {
        coords: (0, 0),
        direction: BeamDirection::East,
    })?;

    let energized_tiles = resolved_contraption.energized_tiles();

    Ok(energized_tiles.len().to_string())
}

fn part_two() -> Result<String, AdventError> {
    // read input file
    let input = fs::read_to_string(INPUT_FILE)?;

    let contraption = Contraption::parse(&input)?;

    let starting_beam = contraption.find_ideal_starting_beam()?;

    println!("starting beam: {:?}", starting_beam);

    let resolved_contraption = contraption.resolve_beams_starting_from(starting_beam)?;

    let energized_tiles = resolved_contraption.energized_tiles();

    Ok(energized_tiles.len().to_string())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Contraption {
    tiles: Grid<Tile>,
}

impl Contraption {
    fn parse(input: &str) -> Result<Self, String> {
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();

        let mut tiles = Grid::new_empty(width, height);
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                tiles.set(x as i64, y as i64, Tile::parse(c)?)?;
            }
        }

        Ok(Self { tiles })
    }

    fn find_ideal_starting_beam(&self) -> Result<BeamData, String> {
        let edge_beams = self.get_all_possible_edge_beams();
        let mut optimal_beam = None;
        let mut optimal_energized_tiles = 0;
        for beam in edge_beams {
            let resolved_contraption = self.resolve_beams_starting_from(beam)?;
            let energized_tile_count = resolved_contraption.energized_tiles().len();
            if energized_tile_count > optimal_energized_tiles {
                optimal_beam = Some(beam);
                optimal_energized_tiles = energized_tile_count;
            }
        }
        optimal_beam.ok_or("No optimal beam found!".to_string())
    }

    fn get_all_possible_edge_beams(&self) -> Vec<BeamData> {
        let mut starting_beams = Vec::new();
        for x in 0..self.tiles.width() {
            // add top edge
            starting_beams.push(BeamData {
                coords: (x as i64, 0),
                direction: BeamDirection::South,
            });
            // add bottom edge
            starting_beams.push(BeamData {
                coords: (x as i64, (self.tiles.height() - 1) as i64),
                direction: BeamDirection::North,
            });
        }
        for y in 0..self.tiles.height() {
            // add left edge
            starting_beams.push(BeamData {
                coords: (0, y as i64),
                direction: BeamDirection::East,
            });
            // add right edge
            starting_beams.push(BeamData {
                coords: ((self.tiles.width() - 1) as i64, y as i64),
                direction: BeamDirection::West,
            });
        }
        starting_beams
    }

    fn resolve_beams_starting_from(
        &self,
        starting_beam: BeamData,
    ) -> Result<ResolvedContraption, String> {
        let tile_data = self
            .tiles
            .clone()
            .take_data()
            .iter()
            .map(|row| row.iter().map(|t| TileData::new(*t)).collect_vec())
            .collect_vec();
        let tile_data_grid = Grid::from_data(tile_data)?;
        let mut resolved_contraption = ResolvedContraption {
            tiles: tile_data_grid,
        };

        let mut to_resolve = HashSet::new();
        to_resolve.insert(starting_beam);

        resolved_contraption
            .tiles
            .get_mut(starting_beam.coords.0, starting_beam.coords.1)
            .unwrap()
            .beams
            .insert(starting_beam.direction);

        while !to_resolve.is_empty() {
            // println!("to_resolve: {:?}", to_resolve);
            let active_beam = *to_resolve.iter().next().unwrap();

            let produced_beams = active_beam.resolve(&resolved_contraption.tiles);

            for beam in produced_beams {
                let affected_tile = resolved_contraption
                    .tiles
                    .get_mut(beam.coords.0, beam.coords.1)
                    .unwrap();

                if !affected_tile.beams.contains(&beam.direction) {
                    to_resolve.insert(beam);
                    affected_tile.beams.insert(beam.direction);
                }
            }

            to_resolve.remove(&active_beam);
        }

        Ok(resolved_contraption)
    }
}

struct ResolvedContraption {
    tiles: Grid<TileData>,
}

impl ResolvedContraption {
    fn energized_tiles(&self) -> Vec<GridEntry<&TileData>> {
        self.tiles
            .entries()
            .filter(
                |GridEntry {
                     x: _x,
                     y: _y,
                     value,
                 }| !value.beams.is_empty(),
            )
            .collect_vec()
    }

    fn energized_tiles_string(&self) -> String {
        let mut s = String::new();
        for (y, row) in self.tiles.rows_iter().enumerate() {
            for tile in row {
                if tile.beams.is_empty() {
                    s.push('.');
                } else {
                    s.push('#');
                }
            }
            if y < self.tiles.height() - 1 {
                s.push('\n');
            }
        }
        s
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BeamData {
    coords: (i64, i64),
    direction: BeamDirection,
}

impl BeamData {
    fn resolve(&self, tiles: &Grid<TileData>) -> HashSet<BeamData> {
        let this_tile_data = tiles.get(self.coords.0, self.coords.1).unwrap();

        let new_directions = this_tile_data.tile.new_beam_directions_from(self.direction);

        let new_beams_unfiltered_vec = new_directions
            .into_iter()
            .map(|new_direction| BeamData {
                coords: new_direction.new_coords_from(self.coords),
                direction: new_direction,
            })
            .collect_vec();

        let mut new_beams = HashSet::new();
        for beam in new_beams_unfiltered_vec {
            if tiles.is_within_bounds(beam.coords.0, beam.coords.1) {
                new_beams.insert(beam);
            }
        }

        new_beams
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct TileData {
    tile: Tile,
    beams: HashSet<BeamDirection>,
}

impl TileData {
    fn new(tile: Tile) -> Self {
        Self {
            tile,
            beams: HashSet::new(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    #[default]
    EmptySpace,
    Mirror(MirrorDirection),
    Splitter(SplitterDirection),
}

impl Tile {
    fn parse(c: char) -> Result<Self, String> {
        match c {
            '.' => Ok(Self::EmptySpace),
            '/' => Ok(Self::Mirror(MirrorDirection::Forward)),
            '\\' => Ok(Self::Mirror(MirrorDirection::Backward)),
            '-' => Ok(Self::Splitter(SplitterDirection::Horizontal)),
            '|' => Ok(Self::Splitter(SplitterDirection::Vertical)),
            _ => Err(format!("unexpected character: {}", c)),
        }
    }

    fn new_beam_directions_from(&self, direction: BeamDirection) -> Vec<BeamDirection> {
        match self {
            Self::EmptySpace => vec![direction],
            Self::Mirror(MirrorDirection::Forward) => match direction {
                BeamDirection::North => vec![BeamDirection::East],
                BeamDirection::South => vec![BeamDirection::West],
                BeamDirection::East => vec![BeamDirection::North],
                BeamDirection::West => vec![BeamDirection::South],
            },
            Self::Mirror(MirrorDirection::Backward) => match direction {
                BeamDirection::North => vec![BeamDirection::West],
                BeamDirection::South => vec![BeamDirection::East],
                BeamDirection::East => vec![BeamDirection::South],
                BeamDirection::West => vec![BeamDirection::North],
            },
            Self::Splitter(SplitterDirection::Horizontal) => match direction {
                BeamDirection::North | BeamDirection::South => {
                    vec![BeamDirection::East, BeamDirection::West]
                }
                BeamDirection::East => vec![BeamDirection::East],
                BeamDirection::West => vec![BeamDirection::West],
            },
            Self::Splitter(SplitterDirection::Vertical) => match direction {
                BeamDirection::North => vec![BeamDirection::North],
                BeamDirection::South => vec![BeamDirection::South],
                BeamDirection::East | BeamDirection::West => {
                    vec![BeamDirection::North, BeamDirection::South]
                }
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BeamDirection {
    North,
    South,
    East,
    West,
}

impl BeamDirection {
    fn new_coords_from(&self, coords: (i64, i64)) -> (i64, i64) {
        match self {
            Self::North => (coords.0, coords.1 - 1),
            Self::South => (coords.0, coords.1 + 1),
            Self::East => (coords.0 + 1, coords.1),
            Self::West => (coords.0 - 1, coords.1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MirrorDirection {
    Forward,
    Backward,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SplitterDirection {
    Horizontal,
    Vertical,
}
