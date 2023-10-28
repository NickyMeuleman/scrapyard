use std::env;

use aoc_all::print_part;
use aoc_core::{Day, Part, Year, DAYS, LAST_YEAR};

fn main() {
    let mut args = env::args();

    // the first argument is the location the program is running, we don't need that
    args.next();

    let year: Option<Year> = {
        match args.next() {
            Some(year) => match year.parse() {
                Ok(val) => Year::try_new(val).ok(),
                Err(_) => None,
            },
            None => None,
        }
    };

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
            Some(part) => match part.parse() {
                Ok(val) => Part::new(val),
                Err(_) => Part::Both,
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
                for num in 1..=DAYS {
                    let day = Day::try_new(num).unwrap();
                    print_part(year, day, &part);
                    println!("\n");
                }
            }
        }
    } else {
        // run all years
        for num in 2015..=LAST_YEAR {
            let year = Year::try_new(num).unwrap();
            for num in 1..=DAYS {
                let day = Day::try_new(num).unwrap();
                print_part(year, day, &part);
                println!("\n");
            }
        }
    }
}
