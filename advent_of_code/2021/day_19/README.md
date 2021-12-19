## Part 1

Incredibly long runtimes today.
Reduced sample size to 10 for part2 because the benchmark took too long and I needed to go.

### Criterion benchmark

```
part1
time:   [4.0263 s 4.0818 s 4.1532 s]
Found 6 outliers among 100 measurements (6.00%)
5 (5.00%) high mild
1 (1.00%) high severe
```

after swapping the standard `HashSet` to the [hashbrown](https://crates.io/crates/hashbrown) drop-in replacement:

```
reduces-sample-size/part1
time:   [1.7021 s 1.7344 s 1.7766 s]
change: [-59.038% -58.113% -57.054%] (p = 0.00 < 0.05)
Performance has improved.
Found 2 outliers among 10 measurements (20.00%)
2 (20.00%) high severe
```

After changing `rotations()` to return an iterator instead of a vector:

```
reduces-sample-size/part1
time:   [1.6855 s 1.6929 s 1.7032 s]
change: [-15.451% -11.019% -7.3658%] (p = 0.00 < 0.05)
Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
1 (10.00%) high severe
```

## Part 2

### Criterion benchmark

```
reduces-sample-size/part2
time:   [4.1362 s 4.2487 s 4.3838 s]
Found 2 outliers among 10 measurements (20.00%)
2 (20.00%) high mild
```

after swapping the standard `HashSet` to the [hashbrown](https://crates.io/crates/hashbrown) drop-in replacement:

```
reduces-sample-size/part2
time:   [1.7095 s 1.7528 s 1.8180 s]
change: [-60.462% -58.745% -56.763%] (p = 0.00 < 0.05)
Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
1 (10.00%) high severe
```

After changing `rotations()` to return an iterator instead of a vector:

```
Benchmarking reduces-sample-size/part2: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 16.9s.
reduces-sample-size/part2
time:   [1.6990 s 1.7316 s 1.7676 s]
change: [-26.363% -21.918% -17.337%] (p = 0.00 < 0.05)
Performance has improved.
```