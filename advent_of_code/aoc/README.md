Workspace containing code related to [Advent of Code 2023](https://adventofcode.com/2023).

What the directories in this reporitory contain:

`<year>/solutions`:
- Solution code per day.
- Small CLI tool to call those solutions.
- Benchmarks for each solution.

`<year>/wasm`:
- A WebAssembly API to use the solutions on different inputs.
- A sample website that uses the built WASM.

`<year>/inputs`:
- Holds input files for each day, used by the [core] CLI and benchmarks.
    - These files should be provided by the user of this repo.

`all/solutions`:
- Small CLI tool to call solutions for every year.

`all/wasm`:
- A WebAssembly API to use the solutions on different inputs.
- A sample website that uses the built WASM.

`core`:
- Shared code, includes types and pieces of code

---

This repo does not contain my personal input files because [the creator prefers people not to share inputs](https://twitter.com/ericwastl/status/1465805354214830081).
The official subreddit [repeats this request](https://www.reddit.com/r/adventofcode/wiki/faqs/copyright/puzzle_texts/).
And [discourages collecting inputs](https://www.reddit.com/r/adventofcode/wiki/faqs/copyright/inputs/)

The files in `<year>/inputs` directories should have filenames following the `day<number>.txt` where `<number>` is 2 digits.
eg. `day01.txt` to `day25.txt`.

Each day of Advent of Code, you can download the input at the bottom of that days question.
For day where the official site provides the input inline, create a textfile with that input to run the CLI/benchmarks in [core].

---

More information about each crate can be found in its readme.