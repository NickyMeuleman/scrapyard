Solutions to [Advent of Code 2021](https://adventofcode.com/2021) in the [Rust programming language](https://www.rust-lang.org/)

## Using the CLI to solve days locally

`cargo run <day> <part>`

`<part>` is optional, omitting it will solve both parts.

eg: 
- `cargo run 3 1` to run day 3, part 1
- `cargo run 3` to run day 3, both parts

Answers are written to the terminal along with a total runtime.

## Using this crate as a dependency

This repo does not contain my personal input files because [the creator prefers people not to share inputs](https://twitter.com/ericwastl/status/1465805354214830081).
The official subreddit [repeats this request](https://www.reddit.com/r/adventofcode/wiki/faqs/copyright/puzzle_texts/).
And [discourages collecting inputs](https://www.reddit.com/r/adventofcode/wiki/faqs/copyright/inputs/)

The files in the `inputs` directory should have filenames following the `day<number>.txt` where `<number>` is 2 digits.
eg. `day01.txt` to `day25.txt`.

Each day of Advent of Code, you can download the input at the bottom of that days question.
For days where the official site provides the input inline, create a textfile with that input to run the CLI/benchmarks.

Two example usages of this crate:
1. Using the `print_part` function requires inputs to be stored in `inputs/day<num>.txt`
1. For more control, use the `solve_part` function.

```rust
use aoc2021::{print_part, solve_part, Answer, Day, Part};

fn main() {
    let day = Day::try_new(1).unwrap();
    let part = Part::Both;

    // Using the builtin printing utility, input has to be in inputs/day<num>.txt
    print_part(&day, &part);

    // OR, more manual usage
    let input = include_str!("../inputs/day01.txt");
    let answer = solve_part(&day, &part, input);
    match answer {
        Ok(answer) => match answer {
            Answer::Part(res) => println!("{}", res),
            Answer::Both(solution) => println!("{}\n{}", solution.part1, solution.part2),
        },
        Err(err) => println!("{}", err.to_string()),
    }
}
```

## Benchmarking

`cargo bench <filter>`

eg:
- `cargo bench "Day: 01` to bench day 1
- `cargo bench 3` to bench day 3

Benchmarks:
1. parsing
1. part 1
1. part 2
1. both parts combined

Note: I implement a method that avoids duplicate work between part 1 and part 2 sometimes,
so benchmarking both parts is NOT the same as adding part1 to part 2.