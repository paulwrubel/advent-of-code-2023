use std::{
    fmt::Display,
    fs,
    ops::{self, Sub},
    time::Instant,
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
    let start = Instant::now();
    // read input file
    let input =
        fs::read_to_string(INPUT_FILE).map_err(|err| AdventError::Other(err.to_string()))?;

    let almanac = Almanac::build_from_string(&input);

    // let a = &almanac.seed_to_soil;
    // let b = &almanac.soil_to_fertilizer;
    // println!("A:\t\t{}", a);
    // println!("B:\t\t{}", b);
    // let intersection = &(a & b);
    // println!("A & B:\t\t{}", intersection);
    // println!("A - (A & B)\t{}", a - intersection);
    // println!("B - (A & B)\t{}", b - intersection);
    // println!("A + B\t\t{}", a + b);

    // let seed_to_soil = &almanac.seed_to_soil;
    // println!("seed_to_soil:\t\t\t{}", seed_to_soil);
    // println!();
    // let soil_to_fertilizer = &almanac.soil_to_fertilizer;
    // println!("soil_to_fertilizer:\t\t{}", soil_to_fertilizer);
    // let seed_to_fertilizer = seed_to_soil.merge_debug(soil_to_fertilizer);
    // println!("seed_to_fertilizer:\t\t{}", seed_to_fertilizer);
    // println!();
    // let fertilizer_to_water = &almanac.fertilizer_to_water;
    // println!("fertilizer_to_water:\t\t{}", fertilizer_to_water);
    // let seed_to_water = &seed_to_fertilizer + &fertilizer_to_water;
    // println!("seed_to_water:\t\t\t{}", seed_to_water);

    // let seed_to_water = &seed_to_fertilizer + &almanac.fertilizer_to_water;
    // let seed_to_light = &seed_to_water + &almanac.water_to_light;
    // let seed_to_temperature = &seed_to_light + &almanac.light_to_temperature;
    // let seed_to_humidity = &seed_to_temperature + &almanac.temperature_to_humidity;
    // let seed_to_location = &seed_to_humidity + &almanac.humidity_to_location;

    let seed_to_location = almanac.seed_to_location_intervals();

    let seed_id_ranges = almanac.seed_id_ranges(false);

    let final_ranges = &seed_id_ranges & &seed_to_location;

    println!("final_ranges:\t\t\t{}", final_ranges);

    let minimum_location = final_ranges.minimum_output().unwrap();

    // println!();
    // let mut min_location = i64::MAX;
    // for seed_id in almanac.seed_ids(false) {
    //     let seed_id = seed_id as i64;

    //     let location_id = seed_to_location.convert(seed_id);

    //     // let soil_id = seed_to_soil.convert(seed_id);
    //     // let fertilizer_id = seed_to_fertilizer.convert(seed_id);
    //     // let water_id = seed_to_water.convert(seed_id);
    //     // let light_id = seed_to_light.convert(seed_id);
    //     // let temperature_id = seed_to_temperature.convert(seed_id);
    //     // let humidity_id = seed_to_humidity.convert(seed_id);
    //     // let location_id = seed_to_location.convert(seed_id);

    //     // println!("seed: {seed_id}, soil: {soil_id}, fertilizer1: {fertilizer_id_1}, fertilizer2: {fertilizer_id_2}");

    //     // let location_id = almanac.seed_to_location(seed_id);

    //     // println!(
    //     //     "seed: {seed_id}, soil: {soil_id}, fertilizer: {fertilizer_id}, water: {water_id}, light: {light_id}, temperature: {temperature_id}, humidity: {humidity_id}, location: {location_id}"
    //     // );

    //     min_location = min_location.min(location_id);
    // }

    println!("Elapsed: {:.2?}", start.elapsed());

    Ok(minimum_location.to_string())
}

fn part_two() -> Result<String, AdventError> {
    let start = Instant::now();

    // read input file
    let input =
        fs::read_to_string(INPUT_FILE).map_err(|err| AdventError::Other(err.to_string()))?;

    println!("building almanac...");
    let almanac = Almanac::build_from_string(&input);

    let seed_to_location = almanac.seed_to_location_intervals();

    let seed_id_ranges = almanac.seed_id_ranges(true);

    let final_ranges = &seed_id_ranges & &seed_to_location;

    println!("final_ranges:\t\t\t{}", final_ranges);

    let minimum_location = final_ranges.minimum_output().unwrap();

    // let len = almanac.seed_ids_len(true) as u64;
    // println!("{} seeds", len);

    // let ids = almanac.seed_ids(true);
    // let mut processed_seeds = 0 as u64;
    // let mut minimum_location = i64::MAX;
    // for seed_id in ids {
    //     let seed_id = seed_id as i64;

    //     let location_id = almanac.seed_to_location(seed_id);

    //     minimum_location = minimum_location.min(location_id);
    //     processed_seeds += 1;

    //     // println!(
    //     //     "seed: {seed_id}, soil: {soil_id}, fertilizer: {fertilizer_id}, water: {water_id}, light: {light_id}, temperature: {temperature_id}, humidity: {humidity_id}, location: {location_id}"
    //     // )

    //     if processed_seeds % 30_000_000 == 0 {
    //         let duration_so_far = start.elapsed();
    //         // println!("b");
    //         let ratio_seeds_processed = processed_seeds as f64 / len as f64;
    //         // println!("{}", ratio_seeds_processed);

    //         let predicted_time = duration_so_far.mul_f64(1.0 / ratio_seeds_processed);
    //         // println!("b");
    //         let remaining_time = predicted_time - duration_so_far;
    //         // println!("b");

    //         println!(
    //             "processed seeds: {} / {} ({:>6.2}% | est. {} remaining...)",
    //             processed_seeds,
    //             len,
    //             ratio_seeds_processed * 100.0,
    //             utils::format_duration(remaining_time)
    //         );
    //     }
    // }

    println!("Elapsed: {:.2?}", start.elapsed());

    Ok(minimum_location.to_string())
}

struct Almanac {
    seed_ids: Vec<i64>,

    seed_to_soil_func: Box<dyn Fn(i64) -> i64 + Sync + Send>,
    soil_to_fertilizer_func: Box<dyn Fn(i64) -> i64 + Sync + Send>,
    fertilizer_to_water_func: Box<dyn Fn(i64) -> i64 + Sync + Send>,
    water_to_light_func: Box<dyn Fn(i64) -> i64 + Sync + Send>,
    light_to_temperature_func: Box<dyn Fn(i64) -> i64 + Sync + Send>,
    temperature_to_humidity_func: Box<dyn Fn(i64) -> i64 + Sync + Send>,
    humidity_to_location_func: Box<dyn Fn(i64) -> i64 + Sync + Send>,

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

        // let seed_to_soil: SortedDisjointIntervalList = property_maps[0].clone().into();

        // let soil_to_fertilizer: SortedDisjointIntervalList = property_maps[1].clone().into();
        // let seed_to_fertilizer = &seed_to_soil + &soil_to_fertilizer;

        // let fertilizer_to_water: SortedDisjointIntervalList = property_maps[2].clone().into();
        // let seed_to_water = &seed_to_fertilizer + &fertilizer_to_water;

        // let water_to_light: SortedDisjointIntervalList = property_maps[3].clone().into();
        // let seed_to_light = &seed_to_water + &water_to_light;

        // let light_to_temperature: SortedDisjointIntervalList = property_maps[4].clone().into();
        // let seed_to_temperature = &seed_to_light + &light_to_temperature;

        // let temperature_to_humidity: SortedDisjointIntervalList = property_maps[5].clone().into();
        // let seed_to_humidity = &seed_to_temperature + &temperature_to_humidity;

        // let humidity_to_location: SortedDisjointIntervalList = property_maps[6].clone().into();
        // let seed_to_location = &seed_to_humidity + &humidity_to_location;

        let seed_to_soil: SortedDisjointIntervalList = property_maps[0].clone().into();
        let soil_to_fertilizer: SortedDisjointIntervalList = property_maps[1].clone().into();
        let fertilizer_to_water: SortedDisjointIntervalList = property_maps[2].clone().into();
        let water_to_light: SortedDisjointIntervalList = property_maps[3].clone().into();
        let light_to_temperature: SortedDisjointIntervalList = property_maps[4].clone().into();
        let temperature_to_humidity: SortedDisjointIntervalList = property_maps[5].clone().into();
        let humidity_to_location: SortedDisjointIntervalList = property_maps[6].clone().into();

        Almanac {
            seed_ids,

            seed_to_soil_func: Self::build_mapper_func(property_maps.remove(0)),
            soil_to_fertilizer_func: Self::build_mapper_func(property_maps.remove(0)),
            fertilizer_to_water_func: Self::build_mapper_func(property_maps.remove(0)),
            water_to_light_func: Self::build_mapper_func(property_maps.remove(0)),
            light_to_temperature_func: Self::build_mapper_func(property_maps.remove(0)),
            temperature_to_humidity_func: Self::build_mapper_func(property_maps.remove(0)),
            humidity_to_location_func: Self::build_mapper_func(property_maps.remove(0)),

            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }

    fn seed_ids(
        &self,
        interpret_as_ranges: bool,
    ) -> Box<dyn Iterator<Item = i64> + '_ + Send + Sync> {
        if interpret_as_ranges {
            let seed_ranges = self.get_seed_ranges();
            Box::new(seed_ranges.into_iter().flat_map(|range| range))
        } else {
            let seed_ids_clone = self.seed_ids.clone();
            Box::new(seed_ids_clone.into_iter())
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

    fn seed_to_location(&self, seed_id: i64) -> i64 {
        self.humidity_to_location(self.temperature_to_humidity(self.light_to_temperature(
            self.water_to_light(
                self.fertilizer_to_water(self.soil_to_fertilizer(self.seed_to_soil(seed_id))),
            ),
        )))
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

    fn seed_to_soil(&self, seed_id: i64) -> i64 {
        self.seed_to_soil.convert(seed_id)
    }

    fn soil_to_fertilizer(&self, soil_id: i64) -> i64 {
        self.soil_to_fertilizer.convert(soil_id)
    }

    fn fertilizer_to_water(&self, fertilizer_id: i64) -> i64 {
        self.fertilizer_to_water.convert(fertilizer_id)
    }

    fn water_to_light(&self, water_id: i64) -> i64 {
        self.water_to_light.convert(water_id)
    }

    fn light_to_temperature(&self, light_id: i64) -> i64 {
        self.light_to_temperature.convert(light_id)
    }

    fn temperature_to_humidity(&self, temperature_id: i64) -> i64 {
        self.temperature_to_humidity.convert(temperature_id)
    }

    fn humidity_to_location(&self, humidity_id: i64) -> i64 {
        self.humidity_to_location.convert(humidity_id)
    }

    fn build_mapper_func(
        property_map: Vec<UnparsedRange>,
    ) -> Box<dyn Fn(i64) -> i64 + Sync + Send> {
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
                    let new_n = n + range.offset;
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
    pub source_range: ops::Range<i64>,
    pub offset: i64,
    pub shift: i64,
}

impl ParsedRange {
    fn contains(&self, n: &i64) -> bool {
        self.source_range.contains(n)
    }

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

    fn project_onto(&self, other: &ParsedRange) -> Option<ParsedRange> {
        let shift = self.offset;
        let a = &self.shift_by(shift);
        let b = other;

        let intersection = a & b;

        intersection.map(|i| i.shift_by(-shift))
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
    pub fn new(mut intervals: Vec<ParsedRange>) -> Self {
        intervals.sort_by(|a, b| a.source_range.start.cmp(&b.source_range.start));
        Self { intervals }
    }

    pub fn convert(&self, value: i64) -> i64 {
        match self.intervals.iter().find(|range| range.contains(&value)) {
            Some(range) => value + range.offset,
            None => value,
        }
    }

    pub fn minimum_input(&self) -> Option<i64> {
        self.intervals
            .iter()
            .map(|range| range.source_range.start)
            .min()
    }

    pub fn maximum_input(&self) -> Option<i64> {
        self.intervals
            .iter()
            .map(|range| range.source_range.end)
            .max()
    }

    pub fn minimum_output(&self) -> Option<i64> {
        self.intervals
            .iter()
            .map(|range| range.source_range.start + range.offset)
            .min()
    }

    pub fn maximum_output(&self) -> Option<i64> {
        self.intervals
            .iter()
            .map(|range| range.source_range.end + range.offset)
            .max()
    }

    pub fn shift_by_offsets(&self) -> Self {
        Self::new(
            self.intervals
                .iter()
                .map(|range| range.shift_by(range.offset))
                .collect(),
        )
    }

    pub fn unshift(&self) -> Self {
        Self::new(self.intervals.iter().map(|range| range.unshift()).collect())
    }

    pub fn project_onto(&self, other: &Self) -> Self {
        let mut ai: usize = 0;
        let mut bi: usize = 0;

        let mut a_list = self
            .intervals
            .iter()
            .map(|range| (range.shift_by(range.offset), range.offset))
            .collect::<Vec<_>>();
        a_list.sort_by(|a, b| a.0.source_range.start.cmp(&b.0.source_range.start));

        let b_list = &other.intervals;

        let mut projections = Vec::new();
        while ai < a_list.len() && bi < b_list.len() {
            let (a, shift) = &a_list[ai];
            let b = &b_list[bi];

            if let Some(projection) = a & b {
                projections.push((projection, *shift));
            }

            if a.source_range.end < b.source_range.end {
                ai += 1;
            } else {
                bi += 1;
            }
        }

        SortedDisjointIntervalList::new(
            projections
                .iter()
                .map(|(range, shift)| range.shift_by(-shift))
                .collect(),
        )
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

    pub fn merge(&self, other: &Self) -> SortedDisjointIntervalList {
        self.internal_merge(other, false)
    }

    pub fn merge_debug(&self, other: &Self) -> SortedDisjointIntervalList {
        self.internal_merge(other, true)
    }

    fn internal_merge_legacy(&self, other: &Self, debug: bool) -> SortedDisjointIntervalList {
        let intersection = self & other;
        let sub_self = self - &intersection;
        let sub_other = other - &intersection;

        let res = SortedDisjointIntervalList::new(
            sub_self
                .intervals
                .iter()
                .chain(sub_other.intervals.iter())
                .chain(intersection.intervals.iter())
                .cloned()
                .collect(),
        );

        if debug {
            println!("A:\t\t{}", self);
            println!("B:\t\t{}", other);
            println!("A & B:\t\t{}", intersection);
            println!("A - (A & B)\t{}", sub_self);
            println!("B - (A & B)\t{}", sub_other);
            println!("A + B\t\t{}", res);
        }

        res
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
                    source_range: 0..5,
                    offset: 3,
                    shift: 0,
                },
                ParsedRange {
                    source_range: 5..10,
                    offset: 1,
                    shift: 0,
                },
                ParsedRange {
                    source_range: 10..15,
                    offset: -2,
                    shift: 0,
                }
            ]
        )
    }
}
