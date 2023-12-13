use core::fmt;
use std::{char, collections::HashSet, fs, io::Write};

use crate::{AdventError, ExclusivePart};

const INPUT_FILE: &str = "./resources/day10_input.txt";

pub fn run(epart: ExclusivePart) -> Result<String, AdventError> {
    match epart {
        ExclusivePart::One => part_one(),
        ExclusivePart::Two => part_two(),
    }
}

fn part_one() -> Result<String, AdventError> {
    // read input file
    let input = fs::read_to_string(INPUT_FILE)?;

    let pipe_map = PipeMap::parse_from_str(&input)?;

    let (Coordinates { x: _x, y: _y }, distance_from_start) =
        pipe_map.find_furthest_point_from_start()?;

    Ok(distance_from_start.to_string())
}

fn part_two() -> Result<String, AdventError> {
    // read input file
    let input = fs::read_to_string(INPUT_FILE)?;

    let pipe_map = PipeMap::parse_from_str(&input)?;

    let num_enclosed_tiles = pipe_map.find_num_enclosed_tiles(false)?;

    Ok(num_enclosed_tiles.to_string())
}

struct PipeMap {
    grid: Vec<Vec<Tile>>,
}

impl PipeMap {
    fn parse_from_str(input: &str) -> Result<Self, String> {
        // find size of grid
        // this is assumed to be the same for all lines / columns
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        // pre-fill the grid
        let mut grid = vec![vec![Tile::Ground; height]; width];

        // actually fill the grid
        for x in 0..width {
            let mut lines = input.lines();
            for y in 0..height {
                let line = lines
                    .next()
                    .ok_or("Invalid input! Too many lines? This shouldn't be possible!")?;
                let char = line.chars().nth(x).ok_or(
                    "Invalid input! Too many characters in line! Are you sure the lines are all the same length?",
                )?;

                grid[x][y] = Tile::parse(char)
                    .map_err(|err| format!("Couldn't parse tile character: {}", err.to_string()))?;
            }
        }

        Ok(Self { grid })
    }

    fn find_start_and_directions(&self) -> Result<(Coordinates, Tile), String> {
        for x in 0..self.grid.len() {
            for y in 0..self.grid[x].len() {
                let x = x as i64;
                let y = y as i64;
                let coords = Coordinates { x, y };
                let tile = self.get_or_ground(coords);
                if tile == Tile::Start {
                    let coords = Coordinates { x, y };

                    let north = self.get_or_ground(Coordinates { x, y: y - 1 });
                    let south = self.get_or_ground(Coordinates { x, y: y + 1 });
                    let east = self.get_or_ground(Coordinates { x: x + 1, y });
                    let west = self.get_or_ground(Coordinates { x: x - 1, y });

                    let mut match_num = 0;
                    match north {
                        Tile::NorthSouth | Tile::SouthEast | Tile::SouthWest => {
                            match_num += 1;
                        }
                        _ => {}
                    }
                    match east {
                        Tile::EastWest | Tile::NorthWest | Tile::SouthWest => {
                            match_num += 2;
                        }
                        _ => {}
                    }
                    match south {
                        Tile::NorthSouth | Tile::NorthEast | Tile::NorthWest => {
                            match_num += 4;
                        }
                        _ => {}
                    }
                    match west {
                        Tile::EastWest | Tile::NorthEast | Tile::SouthEast => {
                            match_num += 8;
                        }
                        _ => {}
                    }

                    match match_num {
                        3 => return Ok((coords, Tile::NorthEast)),
                        5 => return Ok((coords, Tile::NorthSouth)),
                        9 => return Ok((coords, Tile::NorthWest)),
                        6 => return Ok((coords, Tile::SouthEast)),
                        10 => return Ok((coords, Tile::EastWest)),
                        12 => return Ok((coords, Tile::SouthWest)),
                        _ => return Err(format!("Invalid match number: {}", match_num)),
                    }
                }
            }
        }
        Err(format!("Could not find start point!"))
    }

    fn get_or_ground(&self, coords: Coordinates) -> Tile {
        if coords.x < 0 || coords.y < 0 {
            return Tile::Ground;
        }
        *self
            .grid
            .get(coords.x as usize)
            .and_then(|row| row.get(coords.y as usize))
            .unwrap_or(&Tile::Ground)
    }

    fn find_furthest_point_from_start(&self) -> Result<(Coordinates, u64), String> {
        let (path, _) = self.get_path_coords_and_winding()?;

        let middle_index = path.len() / 2;
        let location = path[middle_index];

        let distance = middle_index as u64;

        Ok((location, distance))
    }

    fn get_path_coords_and_winding(&self) -> Result<(Vec<Coordinates>, Winding), String> {
        let (start_coords, direction) = self.find_start_and_directions()?;

        let [mut current_location, _] = direction.get_neighboring_coords(start_coords)?;

        let mut path = vec![start_coords];
        let mut left_turn_counter = 0;
        while start_coords != current_location {
            let previous_location = *path.last().unwrap();

            path.push(current_location);
            let tile = self.get_or_ground(current_location);

            match self.get_turn_direction(previous_location, current_location, tile)? {
                Direction::Left => {
                    left_turn_counter += 1;
                }
                Direction::Right => {
                    left_turn_counter -= 1;
                }
                _ => {}
            }

            let neighbors = tile.get_neighboring_coords(current_location)?;

            current_location = if neighbors[0] == previous_location {
                neighbors[1]
            } else {
                neighbors[0]
            };
        }

        let winding = if left_turn_counter > 0 {
            Winding::Left
        } else {
            Winding::Right
        };

        Ok((path, winding))
    }

    fn get_turn_direction(
        &self,
        previous_location: Coordinates,
        current_location: Coordinates,
        tile: Tile,
    ) -> Result<Direction, String> {
        let cardinal = previous_location.get_cardinal_to(current_location)?;

        match cardinal {
            Cardinal::North => match tile {
                Tile::SouthEast => Ok(Direction::Right),
                Tile::SouthWest => Ok(Direction::Left),
                Tile::NorthSouth => Ok(Direction::Straight),
                _ => Err(format!(
                    "Invalid tile ({:?}) if travelling {:?} from {:?}",
                    tile, cardinal, previous_location
                )),
            },
            Cardinal::South => match tile {
                Tile::NorthEast => Ok(Direction::Left),
                Tile::NorthWest => Ok(Direction::Right),
                Tile::NorthSouth => Ok(Direction::Straight),
                _ => Err(format!(
                    "Invalid tile ({:?}) if travelling {:?} from {:?}",
                    tile, cardinal, previous_location
                )),
            },
            Cardinal::East => match tile {
                Tile::EastWest => Ok(Direction::Straight),
                Tile::NorthWest => Ok(Direction::Left),
                Tile::SouthWest => Ok(Direction::Right),
                _ => Err(format!(
                    "Invalid tile ({:?}) if travelling {:?} from {:?}",
                    tile, cardinal, previous_location
                )),
            },
            Cardinal::West => match tile {
                Tile::EastWest => Ok(Direction::Straight),
                Tile::NorthEast => Ok(Direction::Right),
                Tile::SouthEast => Ok(Direction::Left),
                _ => Err(format!(
                    "Invalid tile ({:?}) if travelling {:?} from {:?}",
                    tile, cardinal, previous_location
                )),
            },
        }
    }

    fn flood_fill(
        &self,
        start_coords: Coordinates,
        path_coords: &Vec<Coordinates>,
        flooded: &mut HashSet<Coordinates>,
    ) {
        // check if this location even makes sense
        if !start_coords.is_within_grid_bounds(&self.grid) {
            return;
        }

        // check if we've already flooded this location
        if path_coords.contains(&start_coords) || flooded.contains(&start_coords) {
            return;
        }

        flooded.insert(start_coords);

        // begin the flood!
        for neighbor in start_coords.get_all_neighboring_coords() {
            self.flood_fill(neighbor, path_coords, flooded);
        }
    }

    fn find_num_enclosed_tiles(&self, debug: bool) -> Result<u64, String> {
        self.find_num_enclosed_tiles_flood_fill(debug)
    }

    fn _find_num_enclosed_tiles_raycast(&self, debug: bool) -> Result<u64, String> {
        let (path_coords, _winding) = self.get_path_coords_and_winding()?;

        // raycast from all coords
        let mut flooded = HashSet::new();
        for x in 0..self.grid.len() {
            let column = &self.grid[x];
            for y in 0..column.len() {
                let coords = Coordinates {
                    x: x as i64,
                    y: y as i64,
                };

                // ignore if this coordinate is in the path
                // since it can't possibly be enclosed
                if path_coords.contains(&coords) {
                    continue;
                }

                // cast time!
                // we cast downwards here
                let mut intersections = 0;
                for ny in (y + 1)..column.len() {
                    let cast_coords = Coordinates {
                        x: x as i64,
                        y: ny as i64,
                    };

                    let cast_tile = self.get_or_ground(cast_coords);

                    if path_coords.contains(&cast_coords) && cast_tile != Tile::NorthSouth {
                        intersections += 1;
                    }
                }

                // if odd, we are enclosed
                if intersections % 2 == 1 {
                    flooded.insert(coords);
                }
            }
        }

        if debug {
            self.debug_print_grid(&path_coords, &flooded)?;
        }

        let num_enclosed_tiles = flooded.len();

        Ok(num_enclosed_tiles as u64)
    }

    fn find_num_enclosed_tiles_flood_fill(&self, debug: bool) -> Result<u64, String> {
        let (path_coords, winding) = self.get_path_coords_and_winding()?;

        // get bordering interior coords
        let mut all_interior_coords = vec![];
        for i in 0..path_coords.len() - 1 {
            let this_location = path_coords[i];
            let next_location = path_coords[i + 1];

            let (interior_a, interior_b) = match this_location.get_cardinal_to(next_location)? {
                Cardinal::North => match winding {
                    Winding::Left => (
                        Coordinates {
                            x: this_location.x - 1,
                            y: this_location.y,
                        },
                        Coordinates {
                            x: next_location.x - 1,
                            y: next_location.y,
                        },
                    ),
                    Winding::Right => (
                        Coordinates {
                            x: this_location.x + 1,
                            y: this_location.y,
                        },
                        Coordinates {
                            x: next_location.x + 1,
                            y: next_location.y,
                        },
                    ),
                },
                Cardinal::South => match winding {
                    Winding::Left => (
                        Coordinates {
                            x: this_location.x + 1,
                            y: this_location.y,
                        },
                        Coordinates {
                            x: next_location.x + 1,
                            y: next_location.y,
                        },
                    ),
                    Winding::Right => (
                        Coordinates {
                            x: this_location.x - 1,
                            y: this_location.y,
                        },
                        Coordinates {
                            x: next_location.x - 1,
                            y: next_location.y,
                        },
                    ),
                },
                Cardinal::East => match winding {
                    Winding::Left => (
                        Coordinates {
                            x: this_location.x,
                            y: this_location.y - 1,
                        },
                        Coordinates {
                            x: next_location.x,
                            y: next_location.y - 1,
                        },
                    ),
                    Winding::Right => (
                        Coordinates {
                            x: this_location.x,
                            y: this_location.y + 1,
                        },
                        Coordinates {
                            x: next_location.x,
                            y: next_location.y + 1,
                        },
                    ),
                },
                Cardinal::West => match winding {
                    Winding::Left => (
                        Coordinates {
                            x: this_location.x,
                            y: this_location.y + 1,
                        },
                        Coordinates {
                            x: next_location.x,
                            y: next_location.y + 1,
                        },
                    ),
                    Winding::Right => (
                        Coordinates {
                            x: this_location.x,
                            y: this_location.y - 1,
                        },
                        Coordinates {
                            x: next_location.x,
                            y: next_location.y - 1,
                        },
                    ),
                },
            };

            if interior_a.is_within_grid_bounds(&self.grid) {
                all_interior_coords.push(interior_a);
            }
            if interior_b.is_within_grid_bounds(&self.grid) {
                all_interior_coords.push(interior_b);
            }
        }

        // loop over the interior coords and flood-fill
        let mut flooded: HashSet<Coordinates> = HashSet::new();
        for coords in all_interior_coords {
            self.flood_fill(coords, &path_coords, &mut flooded);
        }

        if debug {
            self.debug_print_grid(&path_coords, &flooded)?;
        }

        let num_enclosed_tiles = flooded.len();

        Ok(num_enclosed_tiles as u64)
    }

    fn debug_print_grid(
        &self,
        path_coords: &Vec<Coordinates>,
        flooded: &HashSet<Coordinates>,
    ) -> Result<(), String> {
        let mut debug_file =
            fs::File::create("./day10_debug.txt").map_err(|err| err.to_string())?;

        for y in 0..self.grid[0].len() {
            for x in 0..self.grid.len() {
                let coords = Coordinates {
                    x: x as i64,
                    y: y as i64,
                };
                let tile = self.get_or_ground(coords);
                let tile_string = Fancify(tile).to_string();

                let path_str = tile_string;
                let flood_str = "*";
                let else_str = " ";

                if path_coords.contains(&coords) {
                    write!(debug_file, "{}", path_str).map_err(|err| err.to_string())?;
                } else if flooded.contains(&coords) {
                    write!(debug_file, "{}", flood_str).map_err(|err| err.to_string())?;
                } else {
                    write!(debug_file, "{}", else_str).map_err(|err| err.to_string())?;
                }
            }
            write!(debug_file, "\n").map_err(|err| err.to_string())?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Start,
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
}

impl Tile {
    fn parse(c: char) -> Result<Self, String> {
        match c {
            'S' => Ok(Self::Start),
            '|' => Ok(Self::NorthSouth),
            '-' => Ok(Self::EastWest),
            'L' => Ok(Self::NorthEast),
            'J' => Ok(Self::NorthWest),
            'F' => Ok(Self::SouthEast),
            '7' => Ok(Self::SouthWest),
            '.' => Ok(Self::Ground),
            _ => Err(format!("Invalid tile: {}", c)),
        }
    }

    fn get_neighboring_coords(&self, coords: Coordinates) -> Result<[Coordinates; 2], String> {
        let Coordinates { x, y } = coords;
        match self {
            Self::NorthSouth => Ok([Coordinates { x, y: y - 1 }, Coordinates { x, y: y + 1 }]),
            Self::EastWest => Ok([Coordinates { x: x + 1, y }, Coordinates { x: x - 1, y }]),
            Self::NorthEast => Ok([Coordinates { x, y: y - 1 }, Coordinates { x: x + 1, y }]),
            Self::NorthWest => Ok([Coordinates { x, y: y - 1 }, Coordinates { x: x - 1, y }]),
            Self::SouthEast => Ok([Coordinates { x, y: y + 1 }, Coordinates { x: x + 1, y }]),
            Self::SouthWest => Ok([Coordinates { x, y: y + 1 }, Coordinates { x: x - 1, y }]),
            _ => Err(format!("No neighbors for tile: {:?}", self)),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Tile::Start => "S",
            Tile::NorthSouth => "|",
            Tile::EastWest => "-",
            Tile::NorthEast => "L",
            Tile::NorthWest => "J",
            Tile::SouthEast => "F",
            Tile::SouthWest => "7",
            Tile::Ground => ".",
        };

        write!(f, "{}", str)
    }
}

struct Fancify(Tile);

impl fmt::Display for Fancify {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self.0 {
            Tile::Start => "\u{25CF}",
            Tile::NorthSouth => "\u{2502}",
            Tile::EastWest => "\u{2500}",
            Tile::NorthEast => "\u{2514}",
            Tile::NorthWest => "\u{2518}",
            Tile::SouthEast => "\u{250C}",
            Tile::SouthWest => "\u{2510}",
            Tile::Ground => "\u{25D8}",
        };

        write!(f, "{}", str)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinates {
    x: i64,
    y: i64,
}

impl Coordinates {
    fn get_cardinal_to(&self, other: Coordinates) -> Result<Cardinal, String> {
        let x_diff = other.x - self.x;
        let y_diff = other.y - self.y;
        if x_diff == 1 && y_diff == 0 {
            Ok(Cardinal::East)
        } else if x_diff == -1 && y_diff == 0 {
            Ok(Cardinal::West)
        } else if x_diff == 0 && y_diff == 1 {
            Ok(Cardinal::South)
        } else if x_diff == 0 && y_diff == -1 {
            Ok(Cardinal::North)
        } else {
            Err(format!(
                "Cannot get cardinal from {:?} to {:?} (not neighboring orthogonally)",
                self, other
            ))
        }
    }

    fn get_all_neighboring_coords(&self) -> [Coordinates; 4] {
        [
            // north
            Coordinates {
                x: self.x,
                y: self.y - 1,
            },
            // south
            Coordinates {
                x: self.x,
                y: self.y + 1,
            },
            // east
            Coordinates {
                x: self.x + 1,
                y: self.y,
            },
            // west
            Coordinates {
                x: self.x - 1,
                y: self.y,
            },
        ]
    }

    fn is_within_bounds(&self, max_x_exclusive: i64, max_y_exclusive: i64) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < max_x_exclusive && self.y < max_y_exclusive
    }

    fn is_within_grid_bounds(&self, grid: &Vec<Vec<Tile>>) -> bool {
        self.is_within_bounds(grid.len() as i64, grid[0].len() as i64)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Straight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Winding {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cardinal {
    North,
    South,
    East,
    West,
}
