# Advent of Code 2021 solver

Solutions to every problem of [advent of code 2021](https://adventofcode.com/2021) written in Rust.

Meant to be compiled to WebAssembly and used on the web.

Exposes a `solve` function to WASM that takes in a day number and an input string, returns the answers in a struct.

After optimizing the output `.wasm` file for size, the solutions run significantly slower.
This consisted of:

- Using the `"-Oz"` flag for `wasm-opt` and the `"z"` flag for `opt-level` in `Cargo.toml`.
- Replacing the faster [`hashbrown`](https://docs.rs/hashbrown/latest/hashbrown/) with the standard `HashMap` and `HashSet` found in [`std::collections`](https://doc.rust-lang.org/std/collections/index.html).
- Replacing usage of the [`format!`](https://doc.rust-lang.org/std/macro.format.html) macro with equivalent logic.

Day19 might pose a practical time constraint now since the browser blocks while executing a function.

Output of `wc -c pkg/aoc2021_bg.wasm`: `207800 pkg/aoc2021_bg.wasm`

and after making `solve` an `async` function so it returns a `Promise` in JS:
`218328 pkg/aoc2021_bg.wasm`

That is, unfortunately, a huge increase in size.

## Day 01

### Part 1

#### Benchmark

```
[1.0398 us 1.0458 us 1.0528 us]
```

After size optimizations
```
[1.3266 us 1.3475 us 1.3709 us]
```

### Part 2

#### Benchmark

```
[4.0967 us 4.1735 us 4.2572 us]
```

After size optimizations
```
[8.6006 us 8.6983 us 8.8200 us]
```

## Day 02

### Part 1

#### Benchmark

```
[1.5897 us 1.6265 us 1.6736 us]
```

After size optimizations
```
[1.3061 us 1.3429 us 1.3882 us]
```

### Part 2

#### Benchmark

```
[1.2958 us 1.3273 us 1.3613 us]
```

After size optimizations
```
[1.3980 us 1.4292 us 1.4611 us]
```

## Day 03

### Part 1

#### Benchmark

```
[64.042 us 66.476 us 69.339 us]
```

After size optimizations
```
[68.542 us 71.163 us 73.841 us]
```

### Part 2

#### Benchmark

```
[164.05 us 169.21 us 174.88 us]
```

After size optimizations
```
[191.49 us 200.65 us 211.10 us]
```

## Day 04

### Part 1

#### Benchmark

```
[207.48 us 212.46 us 218.10 us]
```

After size optimizations
```
[261.79 us 268.69 us 276.97 us]
```

### Part 2

#### Benchmark

```
[677.47 us 694.06 us 713.34 us]
```

After size optimizations
```
[797.75 us 813.56 us 832.00 us]
```

## Day 05

### Part 1

#### Benchmark

```
[4.2040 ms 4.3197 ms 4.4431 ms]
```

After size optimizations
```
[11.803 ms 11.873 ms 11.954 ms]
```

### Part 2

#### Benchmark

```
[7.8825 ms 8.0998 ms 8.3261 ms]
```

After size optimizations
```
[24.712 ms 24.826 ms 24.957 ms]
```

## Day 06

### Part 1

#### Benchmark

```
[5.6814 us 5.7887 us 5.9209 us]
```

After size optimizations
```
[5.6522 us 5.6756 us 5.7004 us]
```

### Part 2

#### Benchmark

```
[6.3091 us 6.3808 us 6.4785 us]
```

After size optimizations
```
[6.5222 us 6.5428 us 6.5657 us]
```

## Day 07

### Part 1

#### Benchmark

```
[1.1787 ms 1.1845 ms 1.1910 ms]
```

After size optimizations
```
[1.3380 ms 1.3426 ms 1.3479 ms]
```

### Part 2

#### Benchmark

```
[2.0990 ms 2.1065 ms 2.1141 ms]
```

After size optimizations
```
[5.8722 ms 5.8925 ms 5.9152 ms]
```

## Day 08

### Part 1

#### Benchmark

```
[921.17 ns 934.28 ns 949.27 ns]
```

After size optimizations
```
[1.1516 us 1.1571 us 1.1635 us]
```

### Part 2

#### Benchmark

```
[145.11 us 147.86 us 150.99 us]
```

After size optimizations
```
[549.63 us 551.68 us 553.91 us]
```

## Day 09

### Part 1

#### Benchmark

```
[73.821 us 74.566 us 75.644 us
```

After size optimizations
```
[82.482 us 82.798 us 83.152 us]
```

### Part 2

#### Benchmark

```
[893.55 us 896.08 us 898.92 us]
```

After size optimizations
```
[2.8614 ms 2.8894 ms 2.9288 ms]
```

## Day 10

### Part 1

#### Benchmark

```
[74.966 us 76.239 us 77.746 us]
```

After size optimizations
```
[68.886 us 69.470 us 70.317 us]
```

### Part 2

#### Benchmark

```
[141.26 us 146.02 us 151.06 us]
```

After size optimizations
```
[136.67 us 137.11 us 137.62 us]
```

## Day 11

### Part 1

#### Benchmark

```
[200.92 us 201.79 us 202.78 us]
```

After size optimizations
```
[291.47 us 292.46 us 293.54 us]
```

### Part 2

#### Benchmark

```
[501.64 us 509.96 us 519.56 us]
```

After size optimizations
```
[673.40 us 675.50 us 677.97 us]
```

## Day 12

### Part 1

#### Benchmark

```
[2.0978 ms 2.1319 ms 2.1806 ms]
```

After size optimizations
```
[3.2680 ms 3.2784 ms 3.2898 ms]
```

### Part 2

#### Benchmark

```
[50.819 ms 51.288 ms 51.788 ms]
```

After size optimizations
```
[111.84 ms 112.33 ms 112.97 ms]
```

## Day 13

### Part 1

#### Benchmark

without mutating fold instructions:

```
[11.951 us 12.238 us 12.569 us]
```

with mutating fold instructions:

```
[12.419 us 12.470 us 12.525 us]
change: [+2.5203% +4.2624% +5.9012%] (p = 0.00 < 0.05)
Performance has regressed.
```

After refactoring fold execution to a seperate function, not a method on Data:

```
[10.885 us 10.931 us 10.981 us]
```

After size optimizations
```
[37.471 us 37.635 us 37.837 us]
```

### Part 2

#### Benchmark

without mutating fold instructions:

```
[83.771 us 85.367 us 87.226 us] 
```

with mutating fold instructions:

```
[82.032 us 82.971 us 84.093 us]
change: [-9.2115% -7.2411% -4.9832%] (p = 0.00 < 0.05)
Performance has improved.
```

After refactoring fold execution to a seperate function, not a method on Data:

```
[79.433 us 80.550 us 82.121 us]
```

After adding logic to recognize characters instead of printing out the ASCII

```
[83.661 us 84.447 us 85.291 us]
```

After size optimizations
```
[207.91 us 208.76 us 209.70 us]
```

## Day 14

### Part 1

#### Benchmark

```
[53.869 us 54.090 us 54.342 us]
```

After size optimizations
```
[157.03 us 157.64 us 158.40 us]
```

### Part 2

#### Benchmark

```
[248.48 us 250.10 us 251.92 us] 
```

After size optimizations
```
[729.82 us 731.85 us 734.01 us]
```

## Day 15

### Part 1

#### Benchmark

with a HashMap as datastructure to store shortest path to a certain point

```
[2.6342 ms 2.6547 ms 2.6761 ms]
```

with a 1D vector as datastructure to store shortest path to a certain point

```
[1.9476 ms 1.9558 ms 1.9643 ms]
change: [-26.981% -26.328% -25.664%] (p = 0.00 < 0.05)
```

After size optimizations
```
[2.4650 ms 2.4850 ms 2.5084 ms]
```

### Part 2

#### Benchmark

with a HashMap as datastructure to store shortest path to a certain point

```
[76.923 ms 78.024 ms 79.276 ms]
```

with a 1D vector as datastructure to store shortest path to a certain point

```
[53.251 ms 53.464 ms 53.690 ms]
change: [-32.613% -31.477% -30.468%] (p = 0.00 < 0.05)
```

After size optimizations
```
[65.226 ms 65.441 ms 65.681 ms]
```

## Day 16

### Part 1

#### Benchmark

Storing bits as `char`s and using `::from_str_radix()` to get numbers:

```
[87.264 us 88.040 us 88.992 us]
```

After storing bits as `bool`s and using math to get numbers:

```
[42.396 us 42.632 us 42.877 us]
change: [-52.498% -51.711% -50.886%] (p = 0.00 < 0.05)
```

After using bit operations to get the decimal number from bits:
(from `.fold(0, |acc, digit| acc * 2 + if digit { 1 } else { 0 }))` to `.fold(0, |acc, bit| (acc << 1) ^ if bit { 1 } else { 0 })`)

```
[42.018 us 42.161 us 42.310 us]
change: [-2.2989% -0.9042% +0.3506%] (p = 0.20 > 0.05)
```

After size optimizations
```
[48.239 us 48.461 us 48.721 us]
```

### Part 2

#### Benchmark

Storing bits as `char`s and using `::from_str_radix()` to get numbers:

```
[91.922 us 92.952 us 94.166 us]
```

After storing bits as `bool`s and using math to get numbers:

```
[48.997 us 50.510 us 52.263 us]
change: [-45.295% -43.619% -41.479%] (p = 0.00 < 0.05)
```

After using bit operations to get the decimal number from bits:
(from `.fold(0, |acc, digit| acc * 2 + if digit { 1 } else { 0 }))` to `.fold(0, |acc, bit| (acc << 1) | if bit { 1 } else { 0 })`)

```
[47.572 us 47.697 us 47.822 us]
change: [-10.960% -7.7181% -4.6726%] (p = 0.00 < 0.05)
```

Conclusion: Is the bit shifting thing more readable? Depends on who's reading.

After size optimizations
```
[48.817 us 48.978 us 49.189 us]
```

## Day 17

### Part 1

#### Benchmark

```
[35.678 ns 36.522 ns 37.610 ns]
```

After size optimizations
```
[41.424 ns 41.548 ns 41.691 ns]
```

### Part 2

#### Benchmark

```
[587.48 us 602.45 us 618.62 us]
```

After size optimizations
```
[538.67 us 540.32 us 542.18 us]
```

## Day 18

### Part 1

#### Benchmark

```
[300.43 us 302.66 us 305.30 us]
```

After size optimizations
```
[674.25 us 676.38 us 678.78 us]
```

### Part 2

#### Benchmark

```
[7.7538 ms 7.7788 ms 7.8057 ms]
```

After size optimizations
```
[15.751 ms 15.804 ms 15.864 ms]
```

## Day 19

This day takes a long time to run.
Decreased sample size to 10, because I'm not waiting for 100 measurements.

First day where I've implemented a custom `solve` method on the `AoCDay` trait to avoid duplicate work when asking for both part1 and 2 at once.
This will make the usage from WASM much faster, since that's the only method it calls currently.

### Part 1

#### Benchmark

```
[1.7463 s 1.7580 s 1.7725 s]
```

After size optimizations
```
[9.2029 s 11.494 s 14.106 s]
```
wow, this is incredible.
The effect of the speed optimizations the compiler did were huge here.

### Part 2

#### Benchmark

```
[1.7438 s 1.7702 s 1.8116 s]
```

After size optimizations
```
[8.4435 s 8.5749 s 8.7118 s]
```

## Day 20

### Part 1

#### Benchmark

With a 2D-VecDeque storing rows and cols

```
[547.49 us 548.93 us 550.44 us]
```

With a 2D-Vec storing rows and cols

```
[441.71 us 443.00 us 444.36 us]
```

After size optimizations
```
[1.1968 ms 1.2213 ms 1.2504 ms]
```

### Part 2

#### Benchmark

With a 2D-VecDeque storing rows and cols

```
[30.728 ms 31.506 ms 32.428 ms]
```

With a 2D-Vec storing rows and cols

```
[25.448 ms 25.821 ms 26.211 ms]]
```

After size optimizations
```
[65.185 ms 66.377 ms 67.696 ms]
```

## Day 21

### Part 1

#### Benchmark

```
[2.9967 us 3.0359 us 3.0783 us]
```

After size optimizations
```
[4.7205 us 4.8601 us 5.0192 us]
```

### Part 2

#### Benchmark

```
[5.5877 ms 5.6601 ms 5.7407 ms]
```

After size optimizations
```
[20.328 ms 20.488 ms 20.657 ms]
```

## Day 22

### Part 1

#### Benchmark

```
[988.51 us 1.0037 ms 1.0260 ms]
```

commented out approach

```
[1.3082 ms 1.3272 ms 1.3506 ms]
```

After size optimizations
```
[1.5193 ms 1.5497 ms 1.5819 ms]
```

### Part 2

#### Benchmark

```
[2.4163 ms 2.4292 ms 2.4437 ms]
```

commented out approach

```
[53.378 ms 55.485 ms 57.885 ms]
```

After size optimizations
```
[2.6511 ms 2.6884 ms 2.7272 ms]
```

## Day 23

### Part 1

#### Benchmark

```
[46.637 ms 46.829 ms 47.036 ms]
```

After size optimizations
```
[128.67 ms 129.66 ms 130.78 ms]
```

### Part 2

Part2 takes less time to complete somehow. Wow.

#### Benchmark

```
[40.298 ms 40.427 ms 40.561 ms]
```

After size optimizations
```
[115.58 ms 116.65 ms 118.02 ms]
```

## Day 24

### Part 1

#### Benchmark

```
[1.0734 s 1.0823 s 1.0937 s]
```

After size optimizations
```
[2.0489 s 2.0831 s 2.1218 s]
```

### Part 2

#### Benchmark

```
[144.04 ms 155.46 ms 162.40 ms]
```

After size optimizations
```
[274.41 ms 281.92 ms 289.90 ms]
```

## Day 25

### Part 1

#### Benchmark

```
[105.07 ms 105.55 ms 106.12 ms]
```

After size optimizations
```
[124.53 ms 126.48 ms 128.63 ms]
```

### Part 2

#### Benchmark

However quickly you can click a button to complete it.