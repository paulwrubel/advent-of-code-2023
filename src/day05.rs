use std::{
    fmt::Display,
    fs,
    ops::{self},
};

use auto_ops::{impl_op, impl_op_ex};

use crate::{utils, AdventError, ExclusivePart};

const INPUT_FILE: &str = "./resources/day05_input.txt";

pub fn run(epart: ExclusivePart) -> Result<String, AdventError> {
    match epart {
        ExclusivePart::One => part_one(),
        ExclusivePart::Two => part_two(),
    }
}

fn part_one() -> Result<String, AdventError> {
    // read input file
    let input =
        fs::read_to_string(INPUT_FILE).map_err(|err| AdventError::Other(err.to_string()))?;

    let almanac = Almanac::build_from_string(&input);

    let seed_to_location = almanac.seed_to_location_intervals();
    let seed_id_ranges = almanac.seed_id_ranges(false);
    let final_ranges = &seed_id_ranges & &seed_to_location;

    let minimum_location = final_ranges.minimum_output().unwrap();

    Ok(minimum_location.to_string())
}

fn part_two() -> Result<String, AdventError> {
    // read input file
    let input =
        fs::read_to_string(INPUT_FILE).map_err(|err| AdventError::Other(err.to_string()))?;

    let almanac = Almanac::build_from_string(&input);

    let seed_to_location = almanac.seed_to_location_intervals();
    let seed_id_ranges = almanac.seed_id_ranges(true);
    let final_ranges = &seed_id_ranges & &seed_to_location;

    let minimum_location = final_ranges.minimum_output().unwrap();

    Ok(minimum_location.to_string())
}

struct Almanac {
    seed_ids: Vec<i64>,

    seed_to_soil: SortedDisjointIntervalList,
    soil_to_fertilizer: SortedDisjointIntervalList,
    fertilizer_to_water: SortedDisjointIntervalList,
    water_to_light: SortedDisjointIntervalList,
    light_to_temperature: SortedDisjointIntervalList,
    temperature_to_humidity: SortedDisjointIntervalList,
    humidity_to_location: SortedDisjointIntervalList,
    // seed_to_fertilizer: SortedDisjointIntervalList,
    // seed_to_water: SortedDisjointIntervalList,
    // seed_to_light: SortedDisjointIntervalList,
    // seed_to_temperature: SortedDisjointIntervalList,
    // seed_to_humidity: SortedDisjointIntervalList,
    // seed_to_location: SortedDisjointIntervalList,
}

impl Almanac {
    fn build_from_string(input: &str) -> Self {
        let mut lines = input.lines();

        // parse seed ids

        let seed_ids = utils::integers_from_string::<i64>(
            lines.next().unwrap().split_once(':').unwrap().1.trim(),
            " ",
        );

        // skip blank line and seed map line
        lines.next();
        lines.next();

        // parse mapping data
        let mut property_maps = Vec::new();
        let mut range_maps = Vec::new();
        for line in lines {
            // skip blank lines
            if line.is_empty() {
                continue;
            }

            // switch to next map if line contains "map"
            if line.contains("map") {
                property_maps.push(range_maps);
                range_maps = Vec::new();
                continue;
            }

            // parse range mapping
            let mapping_nums = utils::integers_from_string::<u64>(line, " ");
            range_maps.push(UnparsedRange {
                destination_start: mapping_nums[0],
                source_start: mapping_nums[1],
                length: mapping_nums[2],
            });
        }

        // add last map
        property_maps.push(range_maps);

        Almanac {
            seed_ids,

            seed_to_soil: property_maps.remove(0).into(),
            soil_to_fertilizer: property_maps.remove(0).into(),
            fertilizer_to_water: property_maps.remove(0).into(),
            water_to_light: property_maps.remove(0).into(),
            light_to_temperature: property_maps.remove(0).into(),
            temperature_to_humidity: property_maps.remove(0).into(),
            humidity_to_location: property_maps.remove(0).into(),
        }
    }

    fn seed_id_ranges(&self, interpret_as_ranges: bool) -> SortedDisjointIntervalList {
        let seed_ranges: Vec<ParsedRange> = if interpret_as_ranges {
            self.get_seed_ranges()
                .into_iter()
                .map(|range| ParsedRange {
                    source_range: range,
                    offset: 0,
                    shift: 0,
                })
                .collect()
        } else {
            self.seed_ids
                .clone()
                .into_iter()
                .map(|seed_id| ParsedRange {
                    source_range: seed_id..(seed_id + 1),
                    offset: 0,
                    shift: 0,
                })
                .collect()
        };
        SortedDisjointIntervalList::new(seed_ranges)
    }

    fn get_seed_ranges(&self) -> Vec<ops::Range<i64>> {
        let mut seed_iter = self.seed_ids.iter();
        let mut iter_ranges = Vec::new();
        // println!("getting seed ranges...");
        loop {
            // maybe get new range
            let start = match seed_iter.next() {
                Some(seed_range_start) => *seed_range_start,
                None => break,
            };
            // if we had a first, we always have a second
            let length = *seed_iter.next().unwrap();

            // add range
            iter_ranges.push(start..(start + length));
        }
        iter_ranges
    }

    fn seed_to_location_intervals(&self) -> SortedDisjointIntervalList {
        let seed_to_fertilizer = &self.seed_to_soil + &self.soil_to_fertilizer;
        let seed_to_water = &seed_to_fertilizer + &self.fertilizer_to_water;
        let seed_to_light = &seed_to_water + &self.water_to_light;
        let seed_to_temperature = &seed_to_light + &self.light_to_temperature;
        let seed_to_humidity = &seed_to_temperature + &self.temperature_to_humidity;
        let seed_to_location = &seed_to_humidity + &self.humidity_to_location;

        seed_to_location
    }
}

#[derive(Debug, Clone, Copy)]
struct UnparsedRange {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedRange {
    pub source_range: ops::Range<i64>,
    pub offset: i64,
    pub shift: i64,
}

impl ParsedRange {
    fn shift_by(&self, shift: i64) -> ParsedRange {
        ParsedRange {
            source_range: self.source_range.start + shift..self.source_range.end + shift,
            offset: self.offset,
            shift: self.shift + shift,
        }
    }

    fn unshift(&self) -> ParsedRange {
        self.shift_by(-self.shift)
    }

    fn intersect(&self, other: &ParsedRange) -> Option<ParsedRange> {
        let a = &self.source_range;
        let b = &other.source_range;

        let left = a.start.max(b.start);
        let right = a.end.min(b.end);

        if left < right {
            Some(ParsedRange {
                source_range: left..right,
                offset: self.offset + other.offset,
                shift: self.shift + other.shift,
            })
        } else {
            None
        }
    }

    fn subtract(&self, other: &ParsedRange) -> Option<Vec<ParsedRange>> {
        match self.intersect(other) {
            // if there's some intersection, figure out which part to remove
            Some(intersection) => {
                let a = &self.source_range;
                let int = &intersection.source_range;

                if a.start == int.start && a.end == int.end {
                    // we are encompassed by the other range, so we are removed when subtracted
                    None
                } else if a.start == int.start {
                    // the left side will be clipped off, since that's where the intersection aligns
                    Some(vec![ParsedRange {
                        source_range: int.end..a.end,
                        offset: self.offset,
                        shift: self.shift,
                    }])
                } else if a.end == int.end {
                    // the right side will be clipped off, since that's where the intersection aligns
                    Some(vec![ParsedRange {
                        source_range: a.start..int.start,
                        offset: self.offset,
                        shift: self.shift,
                    }])
                } else {
                    // we completely encompass the intersection, so we must segment ourself
                    Some(vec![
                        ParsedRange {
                            source_range: a.start..int.start,
                            offset: self.offset,
                            shift: self.shift,
                        },
                        ParsedRange {
                            source_range: int.end..a.end,
                            offset: self.offset,
                            shift: self.shift,
                        },
                    ])
                }
            }
            // no intersection means nothing to subtract! :)
            None => Some(vec![self.clone()]),
        }
    }
}

impl_op_ex!(&|a: &ParsedRange, b: &ParsedRange| -> Option<ParsedRange> { a.intersect(b) });

impl_op_ex!(-|a: &ParsedRange, b: &ParsedRange| -> Option<Vec<ParsedRange>> { a.subtract(b) });

impl Display for ParsedRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:+})", self.source_range, self.offset)
    }
}

impl From<UnparsedRange> for ParsedRange {
    fn from(unparsed_range: UnparsedRange) -> Self {
        ParsedRange {
            source_range: unparsed_range.source_start as i64
                ..(unparsed_range.source_start + unparsed_range.length) as i64,
            offset: unparsed_range.destination_start as i64 - unparsed_range.source_start as i64,
            shift: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SortedDisjointIntervalList {
    pub intervals: Vec<ParsedRange>,
}

impl SortedDisjointIntervalList {
    fn new(mut intervals: Vec<ParsedRange>) -> Self {
        intervals.sort_by(|a, b| a.source_range.start.cmp(&b.source_range.start));
        Self { intervals }
    }

    fn minimum_output(&self) -> Option<i64> {
        self.intervals
            .iter()
            .map(|range| range.source_range.start + range.offset)
            .min()
    }

    fn shift_by_offsets(&self) -> Self {
        Self::new(
            self.intervals
                .iter()
                .map(|range| range.shift_by(range.offset))
                .collect(),
        )
    }

    fn unshift(&self) -> Self {
        Self::new(self.intervals.iter().map(|range| range.unshift()).collect())
    }

    fn intersect(&self, other: &Self) -> SortedDisjointIntervalList {
        let mut ai: usize = 0;
        let mut bi: usize = 0;

        let mut intersections = Vec::new();
        while ai < self.intervals.len() && bi < other.intervals.len() {
            let a = &self.intervals[ai];
            let b = &other.intervals[bi];

            if let Some(intersection) = a & b {
                intersections.push(intersection);
            }

            if a.source_range.end < b.source_range.end {
                ai += 1;
            } else {
                bi += 1;
            }
        }

        SortedDisjointIntervalList::new(intersections)
    }

    fn subtract(&self, other: &Self) -> SortedDisjointIntervalList {
        let mut subtractions = Vec::new();
        for a in &self.intervals {
            let mut subtraction: Vec<ParsedRange> = vec![a.clone()];
            for b in &other.intervals {
                if let Some(a_comp) = subtraction.last() {
                    match a_comp - b {
                        Some(s) => subtraction = s,
                        None => subtraction = vec![],
                    }
                }
            }
            subtractions.extend(subtraction);
        }
        SortedDisjointIntervalList::new(subtractions)
    }

    fn merge(&self, other: &Self) -> SortedDisjointIntervalList {
        self.internal_merge(other, false)
    }

    fn internal_merge(&self, other: &Self, debug: bool) -> SortedDisjointIntervalList {
        let self_shifted = self.shift_by_offsets();
        let projection_intersection = &self_shifted & &other;
        let sub_self = &self_shifted - &projection_intersection;
        let sub_other = other - &projection_intersection;

        let res = SortedDisjointIntervalList::new(
            sub_self
                .intervals
                .iter()
                .chain(sub_other.intervals.iter())
                .chain(projection_intersection.intervals.iter())
                .cloned()
                .collect(),
        )
        .unshift();

        if debug {
            println!("A\t\t{}", self);
            println!("B\t\t{}", other);
            println!("A shift\t\t{}", self.shift_by_offsets());
            println!("A proj B\t{}", projection_intersection);
            println!("A - (A proj B)\t{}", sub_self);
            println!("B - (A proj B)\t{}", sub_other);
            println!("A + B\t\t{}", res);
        }

        res
    }
}

impl From<Vec<ParsedRange>> for SortedDisjointIntervalList {
    fn from(intervals: Vec<ParsedRange>) -> Self {
        SortedDisjointIntervalList::new(intervals)
    }
}

impl From<Vec<UnparsedRange>> for SortedDisjointIntervalList {
    fn from(intervals: Vec<UnparsedRange>) -> Self {
        SortedDisjointIntervalList::new(
            intervals
                .iter()
                .map(|unparsed_range| ParsedRange::from(*unparsed_range))
                .collect(),
        )
    }
}

impl_op!(&|a: &SortedDisjointIntervalList,
           b: &SortedDisjointIntervalList|
 -> SortedDisjointIntervalList { a.intersect(b) });

impl_op!(-|a: &SortedDisjointIntervalList,
           b: &SortedDisjointIntervalList|
 -> SortedDisjointIntervalList { a.subtract(b) });

impl_op!(+|a: &SortedDisjointIntervalList,
              b: &SortedDisjointIntervalList|
 -> SortedDisjointIntervalList { a.merge(b) });

impl Display for SortedDisjointIntervalList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let range_strs = self.intervals.iter().map(|range| range.to_string());
        let str = range_strs.collect::<Vec<String>>().join(", ");
        write!(f, "[{}]", str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_intersect_single() {
        let a = SortedDisjointIntervalList::new(vec![ParsedRange {
            source_range: 0..10,
            offset: 3,
            shift: 0,
        }]);
        let b = SortedDisjointIntervalList::new(vec![ParsedRange {
            source_range: 5..15,
            offset: -2,
            shift: 0,
        }]);

        let a_and_b = &a & &b;

        assert_eq!(
            a_and_b.intervals,
            vec![ParsedRange {
                source_range: 5..10,
                offset: 1,
                shift: 0,
            }]
        );
    }

    #[test]
    fn basic_subtraction_single() {
        let a = SortedDisjointIntervalList::new(vec![ParsedRange {
            source_range: 0..10,
            offset: 3,
            shift: 0,
        }]);
        let b = SortedDisjointIntervalList::new(vec![ParsedRange {
            source_range: 5..15,
            offset: -2,
            shift: 0,
        }]);

        let a_minus_b = &a - &b;

        assert_eq!(
            a_minus_b.intervals,
            vec![ParsedRange {
                source_range: 0..5,
                offset: 3,
                shift: 0,
            }]
        );
    }

    #[test]
    fn basic_intersect_double() {
        let a = SortedDisjointIntervalList::new(vec![
            ParsedRange {
                source_range: 0..10,
                offset: 3,
                shift: 0,
            },
            ParsedRange {
                source_range: 20..30,
                offset: 9,
                shift: 0,
            },
        ]);
        let b = SortedDisjointIntervalList::new(vec![
            ParsedRange {
                source_range: 5..15,
                offset: -2,
                shift: 0,
            },
            ParsedRange {
                source_range: 15..25,
                offset: 15,
                shift: 0,
            },
        ]);

        let a_and_b = &a & &b;

        assert_eq!(
            a_and_b.intervals,
            vec![
                ParsedRange {
                    source_range: 5..10,
                    offset: 1,
                    shift: 0,
                },
                ParsedRange {
                    source_range: 20..25,
                    offset: 24,
                    shift: 0,
                },
            ]
        );
    }

    #[test]
    fn basic_subtraction_double() {
        let a = SortedDisjointIntervalList::new(vec![
            ParsedRange {
                source_range: 0..10,
                offset: 3,
                shift: 0,
            },
            ParsedRange {
                source_range: 20..30,
                offset: 9,
                shift: 0,
            },
        ]);
        let b = SortedDisjointIntervalList::new(vec![
            ParsedRange {
                source_range: 5..15,
                offset: -2,
                shift: 0,
            },
            ParsedRange {
                source_range: 15..25,
                offset: 15,
                shift: 0,
            },
        ]);

        let a_minus_b = &a - &b;

        assert_eq!(
            a_minus_b.intervals,
            vec![
                ParsedRange {
                    source_range: 0..5,
                    offset: 3,
                    shift: 0,
                },
                ParsedRange {
                    source_range: 25..30,
                    offset: 9,
                    shift: 0,
                },
            ]
        );
    }

    #[test]
    fn basic_intersect_single_overlapping() {
        let a = SortedDisjointIntervalList::new(vec![
            ParsedRange {
                source_range: 0..10,
                offset: 3,
                shift: 0,
            },
            ParsedRange {
                source_range: 20..30,
                offset: 9,
                shift: 0,
            },
        ]);
        let b = SortedDisjointIntervalList::new(vec![ParsedRange {
            source_range: 5..25,
            offset: -4,
            shift: 0,
        }]);

        let a_and_b = &a & &b;

        assert_eq!(
            a_and_b.intervals,
            vec![
                ParsedRange {
                    source_range: 5..10,
                    offset: -1,
                    shift: 0,
                },
                ParsedRange {
                    source_range: 20..25,
                    offset: 5,
                    shift: 0,
                },
            ]
        );
    }

    #[test]
    fn basic_intersect_double_overlapping() {
        let a = SortedDisjointIntervalList::new(vec![
            ParsedRange {
                source_range: 0..10,
                offset: 3,
                shift: 0,
            },
            ParsedRange {
                source_range: 20..30,
                offset: 9,
                shift: 0,
            },
            ParsedRange {
                source_range: 40..50,
                offset: 27,
                shift: 0,
            },
        ]);
        let b = SortedDisjointIntervalList::new(vec![ParsedRange {
            source_range: 5..45,
            offset: -2,
            shift: 0,
        }]);

        let a_and_b = &a & &b;

        assert_eq!(
            a_and_b.intervals,
            vec![
                ParsedRange {
                    source_range: 5..10,
                    offset: 1,
                    shift: 0,
                },
                ParsedRange {
                    source_range: 20..30,
                    offset: 7,
                    shift: 0,
                },
                ParsedRange {
                    source_range: 40..45,
                    offset: 25,
                    shift: 0,
                },
            ]
        );
    }

    #[test]
    fn basic_merge_single() {
        let a = SortedDisjointIntervalList::new(vec![ParsedRange {
            source_range: 0..10,
            offset: 3,
            shift: 0,
        }]);
        let b = SortedDisjointIntervalList::new(vec![ParsedRange {
            source_range: 5..15,
            offset: -2,
            shift: 0,
        }]);

        let ab = &a + &b;

        assert_eq!(
            ab.intervals,
            vec![
                ParsedRange {
                    source_range: 0..2,
                    offset: 3,
                    shift: 0,
                },
                ParsedRange {
                    source_range: 2..10,
                    offset: 1,
                    shift: 0,
                },
                ParsedRange {
                    source_range: 13..15,
                    offset: -2,
                    shift: 0,
                }
            ]
        )
    }
}
