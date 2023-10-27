use std::env;

use aoc_all::print_part;
use aoc_core::{Part, DAYS, LAST_YEAR};

fn main() {
    let mut args = env::args();

    // the first argument is the location the program is running, we don't need that
    args.next();

    let year: Option<u32> = {
        match args.next() {
            Some(year) => year.parse().ok(),
            None => None,
        }
    };

    let day: Option<u8> = {
        match args.next() {
            Some(day) => day.parse().ok(),
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

    if let Some(year) = year {
        // run one year
        match day {
            Some(num) => {
                // run single day
                print_part(year, num, &part);
            }
            None => {
                // run all days
                for day in 1..=DAYS {
                    print_part(year, day, &part);
                    println!("\n");
                }
            }
        }
    } else {
        // run all years
        for year in 2015..=LAST_YEAR {
            for day in 1..=DAYS {
                print_part(year, day, &part);
                println!("\n");
            }
        }
    }
}
