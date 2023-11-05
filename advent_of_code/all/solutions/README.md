Solutions to [Advent of Code](https://adventofcode.com) in the [Rust programming language](https://www.rust-lang.org/)

## Using the CLI to solve days locally

`cargo run <year> <day> <part>`

- `<day>` is optional, omitting it will solve all days.
- `<part>` is optional, omitting it will solve both parts.

eg: 
- `cargo run 2022 3 1` to run year 2022, day 3, part 1
- `cargo run 2023` to run both parts for all days in 2023
- `cargo run 2023 20` to run year 2023, day 20, both parts

Answers are written to the terminal along with a total runtime.

## Using this crate as a dependency

This repo does not contain my personal input files because [the creator prefers people not to share inputs](https://twitter.com/ericwastl/status/1465805354214830081).
The official subreddit [repeats this request](https://www.reddit.com/r/adventofcode/wiki/faqs/copyright/puzzle_texts/).
And [discourages collecting inputs](https://www.reddit.com/r/adventofcode/wiki/faqs/copyright/inputs/)

The files in the `<year>/inputs` directory should have filenames following the `day<number>.txt` where `<number>` is 2 digits.
eg. `day01.txt` to `day25.txt`.

Each day of Advent of Code, you can download the input at the bottom of that days question.
For days where the official site provides the input inline, create a textfile with that input to run the CLI.

Two example usages of this crate:
1. Using the `print_part` function requires inputs to be stored in `<year>/inputs/day<num>.txt`
1. For more control, use the `solve_part` function.

```rust
use aoc_all::{print_part, solve_part, Answer, Day, Part, Year};

fn main() {
    let year = Year::try_new(2023).unwrap();
    let day = Day::try_new(1).unwrap();
    let part = Part::Both;

    // Using the builtin printing utility, input has to be in inputs/day<num>.txt
    print_part(&year, &day, &part);

    // OR, more manual usage
    let input = include_str!("../2023/inputs/day01.txt");
    let answer = solve_part(&year, &day, &part, input);
    match answer {
        Ok(answer) => match answer {
            Answer::Part(res) => println!("{}", res),
            Answer::Both(solution) => println!("{}\n{}", solution.part1, solution.part2),
        },
        Err(err) => println!("{}", err.to_string()),
    }
}
```