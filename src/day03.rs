use std::fs;

use crate::{AdventError, ExclusivePart};

const INPUT_FILE: &str = "./resources/day03_input.txt";

pub fn run(epart: ExclusivePart) -> Result<String, AdventError> {
    match epart {
        ExclusivePart::One => part_one(),
        ExclusivePart::Two => part_two(),
    }
}

#[derive(Debug)]
struct NumberLocation {
    row_index: usize,
    span_inclusive: (usize, usize),
    num_string: String,
}

struct AsteriskLocation {
    row_index: usize,
    column_index: usize,
}

fn part_one() -> Result<String, AdventError> {
    // read input file
    let input =
        fs::read_to_string(INPUT_FILE).map_err(|err| AdventError::Other(err.to_string()))?;

    // fill rows
    let rows: Vec<Vec<char>> = input
        .lines()
        .map(|line| {
            let row: Vec<char> = line.chars().collect();
            row
        })
        .collect();

    // find numbers
    let mut num_locations = Vec::new();
    for (y, row) in rows.iter().enumerate() {
        let mut c_index = 0;
        'char: loop {
            let c = match row.get(c_index) {
                Some(c) => c,
                None => {
                    // end of the row!
                    break 'char;
                }
            };
            if c.is_numeric() {
                // found a number! where does it end...?
                // println!("found a number at {y},{c_index}: {c}");
                let start_index = c_index;
                let mut sub_c_index = c_index + 1;
                let mut end_index = start_index;
                let mut num_string = String::new();
                num_string.push(*c);
                'num_char: loop {
                    let sub_c = match row.get(sub_c_index) {
                        Some(c) => c,
                        None => {
                            // end of the row!
                            break 'num_char;
                        }
                    };
                    if sub_c.is_numeric() {
                        // keep going
                        end_index = sub_c_index;
                        sub_c_index += 1;
                        num_string.push(*sub_c);
                    } else {
                        // found the end
                        break 'num_char;
                    }
                }
                num_locations.push(NumberLocation {
                    row_index: y,
                    span_inclusive: (start_index, end_index),
                    num_string,
                });
                c_index = sub_c_index;
            } else {
                // try next char
                c_index += 1;
                continue 'char;
            }
        }
    }

    let mut valid_part_nums = Vec::new();
    for location in num_locations {
        // unpack
        let NumberLocation { ref num_string, .. } = location;

        // println!("checking: row {row_index} from {x1} to {x2}: {num_string}");

        // check border for symbols
        let borders_symbol = is_num_bordering_symbol(&rows, &location);

        if borders_symbol {
            // println!("  adding {num_string}");
            valid_part_nums.push(num_string.parse::<u32>().unwrap())
        } else {
            // println!("  skipping {num_string}, no symbol found");
        }
    }

    let result = valid_part_nums.iter().sum::<u32>();

    Ok(result.to_string())
}

fn part_two() -> Result<String, AdventError> {
    // read input file
    let input =
        fs::read_to_string(INPUT_FILE).map_err(|err| AdventError::Other(err.to_string()))?;

    // fill rows
    let rows: Vec<Vec<char>> = input
        .lines()
        .map(|line| {
            let row: Vec<char> = line.chars().collect();
            row
        })
        .collect();

    // find asterisks
    let ast_locations = rows
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(column_index, char)| {
                    if *char == '*' {
                        Some(AsteriskLocation {
                            row_index,
                            column_index,
                        })
                    } else {
                        None
                    }
                })
        })
        .flatten();

    let potential_gears = ast_locations.map(|location| {
        let AsteriskLocation {
            row_index,
            column_index,
        } = location;

        let mut num_set: Vec<NumberLocation> = Vec::new();
        for (x, y) in get_neighbors(column_index as i32, row_index as i32) {
            // println!("  checking {x},{y} for * at {column_index},{row_index}");
            let neighbor_number_location = match get_number_location_from_coordinates(&rows, x, y) {
                Some(location) => location,
                None => {
                    continue;
                }
            };
            let (nx, ny) = (
                neighbor_number_location.span_inclusive.0,
                neighbor_number_location.row_index,
            );
            if num_set
                .iter()
                .any(|num_loc| num_loc.row_index == ny && num_loc.span_inclusive.0 == nx)
            {
                continue;
            }

            // println!(
            //     "  adding {} for * at {column_index},{row_index}",
            //     neighbor_number_location.num_string
            // );
            num_set.push(neighbor_number_location);
        }
        num_set
    });

    let gears = potential_gears.filter(|num_set| num_set.len() == 2);

    let ratios = gears.map(|gear| {
        // println!("{:?}", gear);
        let a = gear[0].num_string.parse::<u32>().unwrap();
        let b = gear[1].num_string.parse::<u32>().unwrap();
        a * b
    });

    let result: u32 = ratios.sum();

    Ok(result.to_string())
}

fn is_num_bordering_symbol(rows: &Vec<Vec<char>>, location: &NumberLocation) -> bool {
    // unpack
    let row_index = location.row_index as i32;
    let x1 = location.span_inclusive.0 as i32;
    let x2 = location.span_inclusive.1 as i32;

    // check left and right
    let this_row = &rows[row_index as usize];
    if x1 > 0 && this_row.get((x1 - 1) as usize).unwrap_or(&'.') != &'.' {
        return true;
    }
    if this_row.get((x2 + 1) as usize).unwrap_or(&'.') != &'.' {
        return true;
    }

    // check above row
    if let Some(above_row) = rows.get((row_index - 1) as usize) {
        for x in (x1 - 1)..=(x2 + 1) {
            if x >= 0 && above_row.get(x as usize).unwrap_or(&'.') != &'.' {
                return true;
            }
        }
    }

    // check below row
    if let Some(below_row) = rows.get((row_index + 1) as usize) {
        for x in (x1 - 1)..=(x2 + 1) {
            if x >= 0 && below_row.get(x as usize).unwrap_or(&'.') != &'.' {
                return true;
            }
        }
    }

    false
}

fn get_number_location_from_coordinates(
    rows: &Vec<Vec<char>>,
    x: i32,
    y: i32,
) -> Option<NumberLocation> {
    if x < 0 || y < 0 {
        return None;
    }
    // unpack
    let row_index = y as usize;
    let xprime = x as usize;

    // check for num
    let row = rows.get(row_index)?;
    let char = row.get(xprime)?;

    if !char.is_numeric() {
        return None;
    }

    // found a number! where does it start AND end...?
    let mut start_index = xprime;
    let mut end_index = xprime;

    // find start
    loop {
        if start_index == 0 {
            break;
        }
        let c = match row.get(start_index - 1) {
            Some(c) => c,
            None => {
                break;
            }
        };
        if !c.is_numeric() {
            break;
        }
        start_index -= 1;
    }

    // find end
    loop {
        let c = match row.get(end_index + 1) {
            Some(c) => c,
            None => {
                break;
            }
        };
        if !c.is_numeric() {
            break;
        }
        end_index += 1;
    }
    let mut num_string = String::new();
    for i in start_index..=end_index {
        num_string.push(row[i]);
    }

    Some(NumberLocation {
        row_index,
        span_inclusive: (start_index, end_index),
        num_string,
    })
}

fn get_neighbors(x: i32, y: i32) -> Vec<(i32, i32)> {
    vec![
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
}
