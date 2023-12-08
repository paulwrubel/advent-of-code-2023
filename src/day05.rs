use std::{
    fs,
    ops::{self, Sub},
    time::Instant,
};

use auto_ops::impl_op_ex;

use crate::{utils, AdventError, ExclusivePart};

const INPUT_FILE: &str = "./resources/day05_input.txt";

pub fn run(epart: ExclusivePart) -> Result<String, AdventError> {
    match epart {
        ExclusivePart::One => part_one(),
        ExclusivePart::Two => part_two(),
    }
}

fn part_one() -> Result<String, AdventError> {
    let start = Instant::now();
    // read input file
    let input =
        fs::read_to_string(INPUT_FILE).map_err(|err| AdventError::Other(err.to_string()))?;

    let almanac = Almanac::build_from_string(&input);

    let mut min_location = u64::MAX;
    for seed_id in almanac.seed_ids(false) {
        let location_id = almanac.seed_to_location(seed_id);

        // println!(
        //     "seed: {seed_id}, soil: {soil_id}, fertilizer: {fertilizer_id}, water: {water_id}, light: {light_id}, temperature: {temperature_id}, humidity: {humidity_id}, location: {location_id}"
        // )

        min_location = min_location.min(location_id);
    }

    println!("Elapsed: {:.2?}", start.elapsed());

    Ok(min_location.to_string())
}

fn part_two() -> Result<String, AdventError> {
    let start = Instant::now();

    // read input file
    let input =
        fs::read_to_string(INPUT_FILE).map_err(|err| AdventError::Other(err.to_string()))?;

    println!("building almanac...");
    let almanac = Almanac::build_from_string(&input);

    let len = almanac.seed_ids_len(true) as u64;
    println!("{} seeds", len);

    let ids = almanac.seed_ids(true);
    let mut processed_seeds = 0 as u64;
    let mut minimum_location = u64::MAX;
    for seed_id in ids {
        let location_id = almanac.seed_to_location(seed_id);

        minimum_location = minimum_location.min(location_id);
        processed_seeds += 1;

        // println!(
        //     "seed: {seed_id}, soil: {soil_id}, fertilizer: {fertilizer_id}, water: {water_id}, light: {light_id}, temperature: {temperature_id}, humidity: {humidity_id}, location: {location_id}"
        // )

        if processed_seeds % 30_000_000 == 0 {
            let duration_so_far = start.elapsed();
            // println!("b");
            let ratio_seeds_processed = processed_seeds as f64 / len as f64;
            // println!("{}", ratio_seeds_processed);

            let predicted_time = duration_so_far.mul_f64(1.0 / ratio_seeds_processed);
            // println!("b");
            let remaining_time = predicted_time - duration_so_far;
            // println!("b");

            println!(
                "processed seeds: {} / {} ({:>6.2}% | est. {} remaining...)",
                processed_seeds,
                len,
                ratio_seeds_processed * 100.0,
                utils::format_duration(remaining_time)
            );
        }
    }

    println!("Elapsed: {:.2?}", start.elapsed());

    Ok(minimum_location.to_string())
}

struct Almanac {
    seed_ids: Vec<u64>,

    seed_to_soil: Box<dyn Fn(u64) -> u64 + Sync + Send>,
    soil_to_fertilizer: Box<dyn Fn(u64) -> u64 + Sync + Send>,
    fertilizer_to_water: Box<dyn Fn(u64) -> u64 + Sync + Send>,
    water_to_light: Box<dyn Fn(u64) -> u64 + Sync + Send>,
    light_to_temperature: Box<dyn Fn(u64) -> u64 + Sync + Send>,
    temperature_to_humidity: Box<dyn Fn(u64) -> u64 + Sync + Send>,
    humidity_to_location: Box<dyn Fn(u64) -> u64 + Sync + Send>,
}

impl Almanac {
    fn build_from_string(input: &str) -> Self {
        let mut lines = input.lines();

        // parse seed ids

        let seed_ids = utils::integers_from_string::<u64>(
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

            seed_to_soil: Self::build_mapper_func(property_maps.remove(0)),
            soil_to_fertilizer: Self::build_mapper_func(property_maps.remove(0)),
            fertilizer_to_water: Self::build_mapper_func(property_maps.remove(0)),
            water_to_light: Self::build_mapper_func(property_maps.remove(0)),
            light_to_temperature: Self::build_mapper_func(property_maps.remove(0)),
            temperature_to_humidity: Self::build_mapper_func(property_maps.remove(0)),
            humidity_to_location: Self::build_mapper_func(property_maps.remove(0)),
        }
    }

    fn seed_ids(
        &self,
        interpret_as_ranges: bool,
    ) -> Box<dyn Iterator<Item = u64> + '_ + Send + Sync> {
        if interpret_as_ranges {
            let seed_ranges = self.get_seed_ranges();
            Box::new(seed_ranges.into_iter().flat_map(|range| range))
        } else {
            let seed_ids_clone = self.seed_ids.clone();
            Box::new(seed_ids_clone.into_iter())
        }
    }

    fn seed_ids_len(&self, interpret_as_ranges: bool) -> usize {
        if interpret_as_ranges {
            self.get_seed_ranges()
                .iter()
                .map(|range| range.size_hint().0)
                .sum()
        } else {
            self.seed_ids.len()
        }
    }

    fn get_seed_ranges(&self) -> Vec<ops::Range<u64>> {
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

    fn seed_to_location(&self, seed_id: u64) -> u64 {
        self.humidity_to_location(self.temperature_to_humidity(self.light_to_temperature(
            self.water_to_light(
                self.fertilizer_to_water(self.soil_to_fertilizer(self.seed_to_soil(seed_id))),
            ),
        )))
    }

    fn seed_to_soil(&self, seed_id: u64) -> u64 {
        (self.seed_to_soil)(seed_id)
    }

    fn soil_to_fertilizer(&self, soil_id: u64) -> u64 {
        (self.soil_to_fertilizer)(soil_id)
    }

    fn fertilizer_to_water(&self, fertilizer_id: u64) -> u64 {
        (self.fertilizer_to_water)(fertilizer_id)
    }

    fn water_to_light(&self, water_id: u64) -> u64 {
        (self.water_to_light)(water_id)
    }

    fn light_to_temperature(&self, light_id: u64) -> u64 {
        (self.light_to_temperature)(light_id)
    }

    fn temperature_to_humidity(&self, temperature_id: u64) -> u64 {
        (self.temperature_to_humidity)(temperature_id)
    }

    fn humidity_to_location(&self, humidity_id: u64) -> u64 {
        (self.humidity_to_location)(humidity_id)
    }

    fn build_mapper_func(
        property_map: Vec<UnparsedRange>,
    ) -> Box<dyn Fn(u64) -> u64 + Sync + Send> {
        // println!("property map: {:?}", property_map);

        let parsed_ranges: Vec<ParsedRange> = property_map
            .iter()
            .map(|unparsed_range| ParsedRange::from(*unparsed_range))
            .collect();

        // println!("parsed ranges: {:?}", parsed_ranges);

        Box::new(move |n| {
            for range in &parsed_ranges {
                // println!("checking range... {:?}", range);
                if range.contains(&n) {
                    let new_n = (n as i64 + range.offset) as u64;
                    // println!("mapping {} -> {} (RANGE: {:?})", n, new_n, range);
                    return new_n;
                }
            }
            // println!("mapping {} -> {} (DEFAULT)", n, n);
            n
        })
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
    pub source_range: ops::Range<u64>,
    pub offset: i64,
}

impl ParsedRange {
    fn contains(&self, n: &u64) -> bool {
        self.source_range.contains(n)
    }

    fn intersect(&self, other: &ParsedRange) -> Option<ParsedRange> {
        let left = self.source_range.start.max(other.source_range.start);
        let right = self.source_range.end.min(other.source_range.end);

        if left < right {
            Some(ParsedRange {
                source_range: left..right,
                offset: self.offset + other.offset,
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
                    }])
                } else if a.end == int.end {
                    // the right side will be clipped off, since that's where the intersection aligns
                    Some(vec![ParsedRange {
                        source_range: a.start..int.start,
                        offset: self.offset,
                    }])
                } else {
                    // we completely encompass the intersection, so we must segment ourself
                    Some(vec![
                        ParsedRange {
                            source_range: a.start..int.start,
                            offset: self.offset,
                        },
                        ParsedRange {
                            source_range: int.end..a.end,
                            offset: self.offset,
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

impl From<UnparsedRange> for ParsedRange {
    fn from(unparsed_range: UnparsedRange) -> Self {
        ParsedRange {
            source_range: unparsed_range.source_start
                ..(unparsed_range.source_start + unparsed_range.length),
            offset: unparsed_range.destination_start as i64 - unparsed_range.source_start as i64,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SortedDisjointIntervalList {
    pub intervals: Vec<ParsedRange>,
}

impl SortedDisjointIntervalList {
    pub fn new(mut intervals: Vec<ParsedRange>) -> Self {
        intervals.sort_by(|a, b| a.source_range.start.cmp(&b.source_range.start));
        Self { intervals }
    }

    pub fn intersect(&self, other: &Self) -> SortedDisjointIntervalList {
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

    pub fn subtract(&self, other: &Self) -> SortedDisjointIntervalList {
        let mut subtractions = Vec::new();
        for a in &self.intervals {
            let mut subtraction: Vec<ParsedRange> = vec![a.clone()];
            for b in &other.intervals {
                let a_comp = subtraction.last().unwrap();
                if let Some(s) = a_comp - b {
                    subtraction = s
                }
            }
            subtractions.extend(subtraction);
        }
        SortedDisjointIntervalList::new(subtractions)
    }

    pub fn merge(&self, other: &Self) -> SortedDisjointIntervalList {
        let intersection = self & other;
        let sub_self = self - &intersection;
        let sub_other = other - &intersection;

        SortedDisjointIntervalList::new(
            sub_self
                .intervals
                .iter()
                .chain(sub_other.intervals.iter())
                .chain(intersection.intervals.iter())
                .cloned()
                .collect(),
        )
    }
}

impl_op_ex!(&|a: &SortedDisjointIntervalList,
              b: &SortedDisjointIntervalList|
 -> SortedDisjointIntervalList { a.intersect(b) });

impl_op_ex!(-|a: &SortedDisjointIntervalList,
              b: &SortedDisjointIntervalList|
 -> SortedDisjointIntervalList { a.subtract(b) });

impl_op_ex!(+|a: &SortedDisjointIntervalList,
              b: &SortedDisjointIntervalList|
 -> SortedDisjointIntervalList { a.merge(b) });

mod tests {
    use super::*;

    #[test]
    fn basic_intersect_single() {
        let a = SortedDisjointIntervalList::new(vec![ParsedRange {
            source_range: 0..10,
            offset: 3,
        }]);
        let b = SortedDisjointIntervalList::new(vec![ParsedRange {
            source_range: 5..15,
            offset: -2,
        }]);

        let a_and_b = a & b;

        assert_eq!(
            a_and_b.intervals,
            vec![ParsedRange {
                source_range: 5..10,
                offset: 1,
            }]
        );
    }

    #[test]
    fn basic_subtraction_single() {
        let a = SortedDisjointIntervalList::new(vec![ParsedRange {
            source_range: 0..10,
            offset: 3,
        }]);
        let b = SortedDisjointIntervalList::new(vec![ParsedRange {
            source_range: 5..15,
            offset: -2,
        }]);

        let a_minus_b = a - b;

        assert_eq!(
            a_minus_b.intervals,
            vec![ParsedRange {
                source_range: 0..5,
                offset: 3,
            }]
        );
    }

    #[test]
    fn basic_intersect_double() {
        let a = SortedDisjointIntervalList::new(vec![
            ParsedRange {
                source_range: 0..10,
                offset: 3,
            },
            ParsedRange {
                source_range: 20..30,
                offset: 9,
            },
        ]);
        let b = SortedDisjointIntervalList::new(vec![
            ParsedRange {
                source_range: 5..15,
                offset: -2,
            },
            ParsedRange {
                source_range: 15..25,
                offset: 15,
            },
        ]);

        let a_and_b = a & b;

        assert_eq!(
            a_and_b.intervals,
            vec![
                ParsedRange {
                    source_range: 5..10,
                    offset: 1,
                },
                ParsedRange {
                    source_range: 20..25,
                    offset: 24,
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
            },
            ParsedRange {
                source_range: 20..30,
                offset: 9,
            },
        ]);
        let b = SortedDisjointIntervalList::new(vec![
            ParsedRange {
                source_range: 5..15,
                offset: -2,
            },
            ParsedRange {
                source_range: 15..25,
                offset: 15,
            },
        ]);

        let a_minus_b = a - b;

        assert_eq!(
            a_minus_b.intervals,
            vec![
                ParsedRange {
                    source_range: 0..5,
                    offset: 3,
                },
                ParsedRange {
                    source_range: 25..30,
                    offset: 9,
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
            },
            ParsedRange {
                source_range: 20..30,
                offset: 9,
            },
        ]);
        let b = SortedDisjointIntervalList::new(vec![ParsedRange {
            source_range: 5..25,
            offset: -4,
        }]);

        let a_and_b = a & b;

        assert_eq!(
            a_and_b.intervals,
            vec![
                ParsedRange {
                    source_range: 5..10,
                    offset: -1,
                },
                ParsedRange {
                    source_range: 20..25,
                    offset: 5,
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
            },
            ParsedRange {
                source_range: 20..30,
                offset: 9,
            },
            ParsedRange {
                source_range: 40..50,
                offset: 27,
            },
        ]);
        let b = SortedDisjointIntervalList::new(vec![ParsedRange {
            source_range: 5..45,
            offset: -2,
        }]);

        let a_and_b = a & b;

        assert_eq!(
            a_and_b.intervals,
            vec![
                ParsedRange {
                    source_range: 5..10,
                    offset: 1,
                },
                ParsedRange {
                    source_range: 20..30,
                    offset: 7,
                },
                ParsedRange {
                    source_range: 40..45,
                    offset: 25,
                },
            ]
        );
    }

    #[test]
    fn basic_merge_single() {
        let a = SortedDisjointIntervalList::new(vec![ParsedRange {
            source_range: 0..10,
            offset: 3,
        }]);
        let b = SortedDisjointIntervalList::new(vec![ParsedRange {
            source_range: 5..15,
            offset: -2,
        }]);

        let ab = a + b;

        assert_eq!(
            ab.intervals,
            vec![
                ParsedRange {
                    source_range: 0..5,
                    offset: 3,
                },
                ParsedRange {
                    source_range: 5..10,
                    offset: 1,
                },
                ParsedRange {
                    source_range: 10..15,
                    offset: -2,
                }
            ]
        )
    }
}
