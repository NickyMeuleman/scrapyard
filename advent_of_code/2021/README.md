## Day 01

### Part 1

#### Benchmark

```
time:   [1.0398 us 1.0458 us 1.0528 us]
```

### Part 2

#### Benchmark

```
time:   [4.0967 us 4.1735 us 4.2572 us]
```

## Day 02

### Part 1

#### Benchmark

```
time:   [1.5897 us 1.6265 us 1.6736 us]
```

### Part 2

#### Benchmark

```
time:   [1.2958 us 1.3273 us 1.3613 us]
```

## Day 03

### Part 1

#### Benchmark

```
time:   [64.042 us 66.476 us 69.339 us]
```

### Part 2

#### Benchmark

```
time:   [164.05 us 169.21 us 174.88 us]
```

## Day 04

### Part 1

#### Benchmark

```
time:   [207.48 us 212.46 us 218.10 us]
```

### Part 2

#### Benchmark

```
time:   [677.47 us 694.06 us 713.34 us]
```

## Day 05

### Part 1

#### Benchmark

```
time:   [4.2040 ms 4.3197 ms 4.4431 ms]
```

### Part 2

#### Benchmark

```
time:   [7.8825 ms 8.0998 ms 8.3261 ms]
```

## Day 06

### Part 1

#### Benchmark

```
time:   [5.6814 us 5.7887 us 5.9209 us]
```

### Part 2

#### Benchmark

```
time:   [6.3091 us 6.3808 us 6.4785 us]
```

## Day 07

### Part 1

#### Benchmark

```
time:   [1.1787 ms 1.1845 ms 1.1910 ms]
```

### Part 2

#### Benchmark

```
time:   [2.0990 ms 2.1065 ms 2.1141 ms]
```

## Day 08

### Part 1

#### Benchmark

```
time:   [921.17 ns 934.28 ns 949.27 ns]
```

### Part 2

#### Benchmark

```
time:   [145.11 us 147.86 us 150.99 us]
```

## Day 09

### Part 1

#### Benchmark

```
time:   [73.821 us 74.566 us 75.644 us
```

### Part 2

#### Benchmark

```
time:   [893.55 us 896.08 us 898.92 us]
```

## Day 10

### Part 1

#### Benchmark

```
time:   [74.966 us 76.239 us 77.746 us]
```

### Part 2

#### Benchmark

```
time:   [141.26 us 146.02 us 151.06 us]
```

## Day 11

### Part 1

#### Benchmark

```
time:   [200.92 us 201.79 us 202.78 us]
```

### Part 2

#### Benchmark

```
time:   [501.64 us 509.96 us 519.56 us]
```

## Day 12

### Part 1

#### Benchmark

```
time:   [2.0978 ms 2.1319 ms 2.1806 ms]
```

### Part 2

#### Benchmark

```
time:   [50.819 ms 51.288 ms 51.788 ms]
```

## Day 13

### Part 1

#### Benchmark

without mutating fold instructions:

```
time:   [11.951 us 12.238 us 12.569 us]
```

with mutating fold instructions:

```
time:   [12.419 us 12.470 us 12.525 us]
change: [+2.5203% +4.2624% +5.9012%] (p = 0.00 < 0.05)
Performance has regressed.
```

After refactoring fold execution to a seperate function, not a method on Data:

```
time:   [10.885 us 10.931 us 10.981 us]
```

### Part 2

#### Benchmark

without mutating fold instructions:

```
time:   [83.771 us 85.367 us 87.226 us] 
```

with mutating fold instructions:

```
time:   [82.032 us 82.971 us 84.093 us]
change: [-9.2115% -7.2411% -4.9832%] (p = 0.00 < 0.05)
Performance has improved.
```

After refactoring fold execution to a seperate function, not a method on Data:

```
time:   [79.433 us 80.550 us 82.121 us]
```

After adding logic to recognize characters instead of printing out the ASCII

```
time:   [83.661 us 84.447 us 85.291 us]
```

## Day 14

### Part 1

#### Benchmark

```
time:   [53.869 us 54.090 us 54.342 us]
```

### Part 2

#### Benchmark

```
time:   [248.48 us 250.10 us 251.92 us] 
```

## Day 15

### Part 1

#### Benchmark

with a HashMap as datastructure to store shortest path to a certain point

```
time:   [2.6342 ms 2.6547 ms 2.6761 ms]
```

with a 1D vector as datastructure to store shortest path to a certain point

```
time:   [1.9476 ms 1.9558 ms 1.9643 ms]
change: [-26.981% -26.328% -25.664%] (p = 0.00 < 0.05)
```

### Part 2

#### Benchmark

with a HashMap as datastructure to store shortest path to a certain point

```
time:   [76.923 ms 78.024 ms 79.276 ms]
```

with a 1D vector as datastructure to store shortest path to a certain point

```
time:   [53.251 ms 53.464 ms 53.690 ms]
change: [-32.613% -31.477% -30.468%] (p = 0.00 < 0.05)
```

## Day 16

### Part 1

#### Benchmark

Storing bits as `char`s and using `::from_str_radix()` to get numbers:

```
time:   [87.264 us 88.040 us 88.992 us]
```

After storing bits as `bool`s and using math to get numbers:

```
time:   [42.396 us 42.632 us 42.877 us]
change: [-52.498% -51.711% -50.886%] (p = 0.00 < 0.05)
```

After using bit operations to get the decimal number from bits:
(from `.fold(0, |acc, digit| acc * 2 + if digit { 1 } else { 0 }))` to `.fold(0, |acc, bit| (acc << 1) ^ if bit { 1 } else { 0 })`)

```
time:   [42.018 us 42.161 us 42.310 us]
change: [-2.2989% -0.9042% +0.3506%] (p = 0.20 > 0.05)
```

### Part 2

#### Benchmark

Storing bits as `char`s and using `::from_str_radix()` to get numbers:

```
time:   [91.922 us 92.952 us 94.166 us]
```

After storing bits as `bool`s and using math to get numbers:

```
time:   [48.997 us 50.510 us 52.263 us]
change: [-45.295% -43.619% -41.479%] (p = 0.00 < 0.05)
```

After using bit operations to get the decimal number from bits:
(from `.fold(0, |acc, digit| acc * 2 + if digit { 1 } else { 0 }))` to `.fold(0, |acc, bit| (acc << 1) | if bit { 1 } else { 0 })`)

```
time:   [47.572 us 47.697 us 47.822 us]
change: [-10.960% -7.7181% -4.6726%] (p = 0.00 < 0.05)
```

Conclusion: Is the bit shifting thing more readable? Depends on who's reading.

## Day 17

### Part 1

#### Benchmark

```
time:   [35.678 ns 36.522 ns 37.610 ns]
```

### Part 2

#### Benchmark

```
time:   [587.48 us 602.45 us 618.62 us]
```

## Day 18

### Part 1

#### Benchmark

```
time:   [300.43 us 302.66 us 305.30 us]
```

### Part 2

#### Benchmark

```
time:   [7.7538 ms 7.7788 ms 7.8057 ms]
```

## Day 19

This day takes a long time to run.
Decreased sample size to 10, because I'm not waiting for 100 measurements.

First day where I've implemented a custom `solve` method on the `AoCDay` trait to avoid duplicate work when asking for both part1 and 2 at once.
This will make the usage from WASM much faster, since that's the only method it calls currently.

### Part 1

#### Benchmark

```
time:   [1.7463 s 1.7580 s 1.7725 s]
```

### Part 2

#### Benchmark

```
time:   [1.7438 s 1.7702 s 1.8116 s]
```

## Day 20

### Part 1

#### Benchmark

With a 2D-VecDeque storing rows and cols

```
time:   [547.49 us 548.93 us 550.44 us]
```

With a 2D-Vec storing rows and cols

```
time:   [441.71 us 443.00 us 444.36 us]
```

### Part 2

#### Benchmark

With a 2D-VecDeque storing rows and cols

```
time:   [30.728 ms 31.506 ms 32.428 ms]
```

With a 2D-Vec storing rows and cols

```
time:   [25.448 ms 25.821 ms 26.211 ms]]
```

## Day 21

### Part 1

#### Benchmark

```
time:   [2.9967 us 3.0359 us 3.0783 us]
```

### Part 2

#### Benchmark

```
time:   [5.5877 ms 5.6601 ms 5.7407 ms]
```

## Day 22

### Part 1

#### Benchmark

```
time:   [988.51 us 1.0037 ms 1.0260 ms]
```

commented out approach

```
time:   [1.3082 ms 1.3272 ms 1.3506 ms]
```

### Part 2

#### Benchmark

```
time:   [2.4163 ms 2.4292 ms 2.4437 ms]
```

commented out approach

```
time:   [53.378 ms 55.485 ms 57.885 ms]
```

## Day 23

### Part 1

#### Benchmark

```
time:   [46.637 ms 46.829 ms 47.036 ms]
```

### Part 2

Part2 takes less time to complete somehow. Wow.

#### Benchmark

```
time:   [40.298 ms 40.427 ms 40.561 ms]
```

## Day 24

### Part 1

#### Benchmark

```
time:   [1.0734 s 1.0823 s 1.0937 s]
```

### Part 2

#### Benchmark

```
time:   [144.04 ms 155.46 ms 162.40 ms]
```

## Day 25

### Part 1

#### Benchmark

```
time:   [105.07 ms 105.55 ms 106.12 ms]
```

### Part 2

#### Benchmark

However quickly you can click a button to complete it.