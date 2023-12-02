use std::fmt::Display;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

#[derive(Debug, Clone, Copy)]
pub enum Parts {
    Both,
    Single(ExclusivePart),
}

#[derive(Debug, Clone, Copy)]
pub enum ExclusivePart {
    One,
    Two,
}

impl Display for ExclusivePart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExclusivePart::One => write!(f, "one"),
            ExclusivePart::Two => write!(f, "two"),
        }
    }
}

pub enum AdventError {
    Unimplemented,
    Other(String),
}

pub fn run(day: u32, part: ExclusivePart) -> Result<String, AdventError> {
    match day {
        1 => day01::run(part),
        2 => day02::run(part),
        3 => day03::run(part),
        4 => day04::run(part),
        5 => day05::run(part),
        6 => day06::run(part),
        7 => day07::run(part),
        8 => day08::run(part),
        9 => day09::run(part),
        10 => day10::run(part),
        11 => day11::run(part),
        12 => day12::run(part),
        13 => day13::run(part),
        14 => day14::run(part),
        15 => day15::run(part),
        16 => day16::run(part),
        17 => day17::run(part),
        18 => day18::run(part),
        19 => day19::run(part),
        20 => day20::run(part),
        21 => day21::run(part),
        22 => day22::run(part),
        23 => day23::run(part),
        24 => day24::run(part),
        25 => day25::run(part),
        _ => panic!("Day {} not implemented", day),
    }
}
