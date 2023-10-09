use aoc2023::{print_day, DAYS};
use std::env;

fn main() {
    let mut args = env::args();

    let day: Option<u8> = {
        // the first argument is the location the program is running, we don't need that
        args.next();
        // the second argument should be the day
        match args.next() {
            Some(day) => day.parse().ok(),
            None => None,
        }
    };

    match day {
        Some(num) => {
            // run single day
            print_day(num);
        }
        None => {
            // run all days
            for num in 1..=DAYS {
                print_day(num);
                println!("\n");
            }
        }
    }
}
