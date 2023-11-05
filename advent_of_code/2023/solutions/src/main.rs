use std::env;

use aoc2023::{print_part, Part, DAYS};
use aoc_core::Day;

fn main() {
    let mut args = env::args();

    // the first argument is the location the program is running, we don't need that
    args.next();

    let day: Option<Day> = {
        match args.next() {
            Some(day) => match day.parse() {
                Ok(val) => Day::try_new(val).ok(),
                Err(_) => None,
            },
            None => None,
        }
    };

    let part = {
        match args.next() {
            Some(part) => match &part[..] {
                "1" => Part::One,
                "2" => Part::Two,
                _ => Part::Both,
            },
            None => Part::Both,
        }
    };

    match day {
        Some(day) => {
            // run single day
            print_part(&day, &part);
        }
        None => {
            // run all days
            for num in 1..=DAYS {
                let day = Day::try_new(num).unwrap();
                print_part(&day, &part);
                println!("\n");
            }
        }
    }
}
