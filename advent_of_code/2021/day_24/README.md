## Part 1

### Criterion benchmark

```
part1
time:   [1.0652 s 1.0787 s 1.0928 s]
```

after using [hashbrown](https://crates.io/crates/hashbrown) as `HashMap` instead of `std::collections::HashMap`:

```
part1
time:   [832.79 ms 833.96 ms 835.21 ms]
change: [-23.696% -22.690% -21.693%] (p = 0.00 < 0.05)
Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
2 (2.00%) high mild
```

## Part 2

### Criterion benchmark

```
part2
time:   [129.81 ms 131.72 ms 133.74 ms]
Found 1 outliers among 100 measurements (1.00%)
1 (1.00%) high mild
```

after using [hashbrown](https://crates.io/crates/hashbrown) as `HashMap` instead of `std::collections::HashMap`:

```
part2
time:   [94.425 ms 94.793 ms 95.208 ms]
change: [-29.156% -28.036% -26.934%] (p = 0.00 < 0.05)
Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
5 (5.00%) high mild
1 (1.00%) high severe
```
