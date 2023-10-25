use std::env;

use aoc2023::{print_part, Part, DAYS};

fn main() {
    let mut args = env::args();

    // the first argument is the location the program is running, we don't need that
    args.next();

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

    match day {
        Some(num) => {
            // run single day
            print_part(num, &part);
        }
        None => {
            // run all days
            for num in 1..=DAYS {
                print_part(num, &part);
                println!("\n");
            }
        }
    }
}
