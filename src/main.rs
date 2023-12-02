use std::env;

use advent_of_code_2023::{run, AdventError, ExclusivePart, Parts};

#[derive(Debug, Clone, Copy)]
pub enum Days {
    Single(u32, Parts),
    All,
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let days = get_days(args)?;

    match days {
        Days::Single(day, parts) => match parts {
            Parts::Single(part) => run_and_print_day(day, part, false),
            Parts::Both => {
                run_and_print_day(day, ExclusivePart::One, false);
                run_and_print_day(day, ExclusivePart::Two, false);
            }
        },
        Days::All => {
            for day in 1..=25 {
                run_and_print_day(day, ExclusivePart::One, true);
                run_and_print_day(day, ExclusivePart::Two, true);
            }
        }
    }

    Ok(())
}

fn run_and_print_day(day: u32, part: ExclusivePart, hide_unimplemented: bool) {
    match run(day, part) {
        Ok(s) => println!("Day {day:>2}, part {part}: {s}"),
        Err(err) => match err {
            AdventError::Other(s) => {
                println!("Day {day:>2}, part {part} (!ERROR!): {s}");
            }
            AdventError::Unimplemented => {
                if !hide_unimplemented {
                    println!("Day {day:>2}, part {part} has not yet been implemented");
                }
            }
        },
    }
}

fn get_days(args: Vec<String>) -> Result<Days, String> {
    if args.len() > 1 {
        let mut day_components = args[1].split('.');

        let day_number = match day_components.next().unwrap().parse::<u32>() {
            Ok(n) => n,
            Err(err) => return Err(format!("Error parsing day number: {err}")),
        };

        let part = match day_components.next() {
            Some(part) => match part {
                "one" | "1" => Parts::Single(ExclusivePart::One),
                "two" | "2" => Parts::Single(ExclusivePart::Two),
                _ => return Err(format!("Unrecognized part: {part}")),
            },
            None => Parts::Both,
        };

        if day_number < 1 || day_number > 25 {
            Err(format!(
                "Day number must be between 1 and 25, inclusive. found {day_number}"
            ))
        } else {
            Ok(Days::Single(day_number, part))
        }
    } else {
        Ok(Days::All)
    }
}
