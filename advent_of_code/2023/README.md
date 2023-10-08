Benchmarking with `hyperfine`:

Using 5 warmup runs before measuring.
The first one of those warmup runs will build the project in release mode.
Every subsequent run will use the cached built project.

Example to benchmark day 1:
```sh
hyperfine --warmup 5 "cargo run --release 1"
```