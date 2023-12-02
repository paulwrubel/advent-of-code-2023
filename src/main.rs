use std::env;

use advent_of_code_2023::{run, UNIMPLEMENTED};

pub enum Days {
    Single(u32),
    All,
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let days = get_days(args)?;

    match days {
        Days::Single(day) => {
            run_and_print_day(day, false);
        }
        Days::All => {
            for day in 1..=25 {
                run_and_print_day(day, true);
            }
        }
    }

    Ok(())
}

fn run_and_print_day(day: u32, hide_unimplemented: bool) {
    let (part_one, part_two) = run(day);
    if !(hide_unimplemented && part_one == UNIMPLEMENTED) {
        println!("Day {day:>2}, part one: {part_one}");
    }
    if !(hide_unimplemented && part_two == UNIMPLEMENTED) {
        println!("Day {day:>2}, part two: {part_two}");
    }
}

fn get_days(args: Vec<String>) -> Result<Days, String> {
    if args.len() > 1 {
        let day_number = match args[1].parse::<u32>() {
            Ok(n) => n,
            Err(err) => return Err(format!("Error parsing day number: {err}")),
        };

        if day_number < 1 || day_number > 25 {
            Err(format!(
                "Day number must be between 1 and 25, inclusive. found {day_number}"
            ))
        } else {
            Ok(Days::Single(day_number))
        }
    } else {
        Ok(Days::All)
    }
}
