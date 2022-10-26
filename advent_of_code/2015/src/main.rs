use aoc2015::utils;
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
            for num in 1..=utils::DAYS {
                print_day(num);
                println!("\n");
            }
        }
    }
}

fn print_day(num: u8) {
    println!("Day{:02}", num);
    let input = utils::get_input(num);
    let utils::Solution { part1, part2 } = aoc2015::solve_sync(num, input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
