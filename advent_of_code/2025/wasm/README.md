Solutions to [Advent of Code 2025](https://adventofcode.com/2025) in the [Rust programming language](https://www.rust-lang.org/), usable through [WebAssemly](https://webassembly.org/).

Use [wasm-pack](https://github.com/rustwasm/wasm-pack) on this crate to generate the WASM-code and the required glue code to use the produced WASM on a website.

On the Rust side, the [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) crate handles the logic about how that glue should behave.

I wrote [a blogpost where you can learn more about WASM](https://nickymeuleman.netlify.app/blog/webassembly).
Among other things, it explains how the interaction between WASM and JavaScript works.

Includes an example usage of the compiled WASM where a `solve` function is called from JavaScript.

That function takes a day number and the user input as a string and calls the corresponding solution.

## Compiling the WASM

`wasm-pack build --target web --out-dir www/pkg`

This will place the output in the `www/pkg` directory, where it is imported by the JavaScript website.

### Optional optimization

If the version of day 10 that uses [`good_lp`](https://docs.rs/good_lp/latest/good_lp/) is used,
optimization has to happen seperately with a couple of flags explicitly enabled in [binaryen `wasm-opt`](https://github.com/WebAssembly/binaryen).

`wasm-opt www/pkg/aoc2025_wasm_bg.wasm \
    -o www/pkg/aoc2025_wasm_bg_opt.wasm \
    -Os \
    --enable-bulk-memory \
    --enable-nontrapping-float-to-int`

`mv www/pkg/aoc2025_wasm_bg.wasm www/pkg/aoc2025_wasm_bg_unopt.wasm`

`mv www/pkg/aoc2025_wasm_bg_opt.wasm www/pkg/aoc2025_wasm_bg.wasm`

That way you have both the unoptimized and the optimized version of the `.wasm`.

## Using the WASM site

1. `cd` into `www`
1. `npm i`
1. `npm run dev`
1. Choose which day to solve
1. Choose which part (or both parts) to solve.
1. Choose your input file
