Solutions to [Advent of Code 2023](https://adventofcode.com/2022) in the [Rust programming language](https://www.rust-lang.org/)

## Solving days

`cargo run <day> <part>`

`<part>` is optional, omitting it will solve both parts.

eg: 
- `cargo run 3 1` to run day 3, part 1
- `cargo run 3` to run day 3, both parts

Answers are written to the terminal along with a total runtime.

## Benchmarking

`cargo bench <day>`

eg:
- `cargo bench 3` to bench day 3

Benchmarks:
1. parsing
1. part 1
1. part 2
1. both parts combined

Note: I implement a method that avoids duplicate work between part 1 and part 2 sometimes,
so benchmarking both parts is NOT the same as adding part1 to part 2.