wc -c www/pkg/aoc2023_wasm_bg.wasm

after codegen-units = 1
34174 www/pkg/aoc2023_wasm_bg.wasm

added strip = true
33985 www/pkg/aoc2023_wasm_bg.wasm

## Building the WASM
1. `cd` into `wasm`
2. `wasm-pack build --scope nickymeuleman --target web --out-dir www/pkg`

## Using the WASM site
1. `cd` into `wasm/www`
2. `npm run dev`

## Using the CLI
1. `cd` into `core`
2. `cargo run <day> <part>`

eg: 
- `cargo run 3 1` to run day 3 part 1
- `cargo run 3` to run day 3 both parts

Answers are written to the console along with a total runtime

## Benchmarking
1. `cd` into root
2. `cargo run <day>`

eg:
- `cargo bench 3` to bench day 3

Benchmarks:
1. parsing
1. part 1
1. part 2
1. both parts combined

Note: I implement a method that avoids duplicate work between part 1 and part 2 sometimes,
so benchmarking both parts is NOT the same as adding part1 to part 2.