## Part 1

### Criterion benchmark

```
part1
time:   [445.33 ms 451.03 ms 457.35 ms]
Found 10 outliers among 100 measurements (10.00%)
4 (4.00%) high mild
6 (6.00%) high severe
```

after rewriting to use more mutation

```
part1
time:   [232.34 ms 235.17 ms 238.29 ms]
change: [-48.852% -47.858% -46.856%] (p = 0.00 < 0.05)
Performance has improved.
Found 15 outliers among 100 measurements (15.00%)
4 (4.00%) high mild
11 (11.00%) high severe
```

use u8 instead of usize for coordinates for a tiny speedup, but decent memory savings

```
part1
time:   [219.06 ms 219.72 ms 220.49 ms]
change: [-7.8274% -6.5708% -5.3977%] (p = 0.00 < 0.05)
Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
5 (5.00%) high mild
1 (1.00%) high severe
```