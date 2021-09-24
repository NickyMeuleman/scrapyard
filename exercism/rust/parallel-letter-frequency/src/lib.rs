use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.len() == 0 {
        return HashMap::new();
    }

    let handles: Vec<_> = input
        .chunks((input.len() / worker_count).max(1))
        .map(|chunk| {
            let string = chunk.join("");
            thread::spawn(move || {
                string
                    .chars()
                    .filter(|c| c.is_alphabetic())
                    .map(|c| c.to_ascii_lowercase())
                    .fold(HashMap::<char, usize>::new(), |mut acc, c| {
                        *acc.entry(c).or_default() += 1;
                        acc
                    })
            })
        })
        .collect();

    handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .reduce(|mut acc, map| {
            for (key, value) in map {
                *acc.entry(key).or_default() += value;
            }
            acc
        })
        .expect("thread maps are not empty")
}

// Iterator version before fix bench results
// test bench_large_parallel   ... bench:     819,007 ns/iter (+/- 204,413)
// test bench_large_sequential ... bench:     474,953 ns/iter (+/- 40,738)
// test bench_small_parallel   ... bench:     260,249 ns/iter (+/- 125,860)
// test bench_small_sequential ... bench:      16,563 ns/iter (+/- 1,761)
// test bench_tiny_parallel    ... bench:      74,991 ns/iter (+/- 10,403)
// test bench_tiny_sequential  ... bench:          56 ns/iter (+/- 4)
// Iterator version above, after fix results:
// test bench_large_parallel   ... bench:     316,377 ns/iter (+/- 26,063)
// test bench_large_sequential ... bench:     477,191 ns/iter (+/- 106,096)
// test bench_small_parallel   ... bench:     153,522 ns/iter (+/- 26,236)
// test bench_small_sequential ... bench:      16,581 ns/iter (+/- 1,343)
// test bench_tiny_parallel    ... bench:      77,122 ns/iter (+/- 12,552)
// test bench_tiny_sequential  ... bench:          55 ns/iter (+/- 4)
// Imperative version below bench results
// test bench_large_parallel   ... bench:     304,838 ns/iter (+/- 16,081)
// test bench_large_sequential ... bench:     476,711 ns/iter (+/- 53,785)
// test bench_small_parallel   ... bench:     151,602 ns/iter (+/- 26,897)
// test bench_small_sequential ... bench:      16,706 ns/iter (+/- 825)
// test bench_tiny_parallel    ... bench:      74,920 ns/iter (+/- 7,201)
// test bench_tiny_sequential  ... bench:          55 ns/iter (+/- 3)

// The fix in iterative version mentioned:
// before, I wasn't waiting to join the handles until all threads had been spawned
// By collecting all joinHandles into a vector first,
// the code only starts joining once all threads have been spawned.
// It's still slightly slower than the version below, but I'll take it

// use std::collections::HashMap;
// use std::thread;

// pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
//     let chunks = input.chunks((input.len() / worker_count).max(1));
//     let mut threads = Vec::new();

//     for chunk in chunks {
//         let string = chunk.join("");
//         let handle = thread::spawn(move || {
//             string
//                 .chars()
//                 .fold(HashMap::<char, usize>::new(), |mut acc, c| {
//                     if c.is_alphabetic() {
//                         *acc.entry(c.to_ascii_lowercase()).or_default() += 1;
//                     }
//                     acc
//                 })
//         });
//         threads.push(handle);
//     }

//     threads.into_iter().fold(HashMap::new(), |mut acc, handle| {
//         for (key, value) in handle.join().unwrap() {
//             *acc.entry(key).or_default() += value;
//         }
//         acc
//     })
// }

// The version that spreads equal amounts of chars across threads is slightly slower,
// than the one that spreads equal lengths of the given input array-slice
// test bench_large_parallel   ... bench:     384,731 ns/iter (+/- 31,526)
// test bench_large_sequential ... bench:     481,675 ns/iter (+/- 47,494)
// test bench_small_parallel   ... bench:     153,132 ns/iter (+/- 14,640)
// test bench_small_sequential ... bench:      16,618 ns/iter (+/- 1,167)
// test bench_tiny_parallel    ... bench:      75,048 ns/iter (+/- 7,415)
// test bench_tiny_sequential  ... bench:          55 ns/iter (+/- 5)
// use std::collections::HashMap;
// use std::thread;

// pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
//     let input = input.join("");
//     if input.is_empty() {
//         return HashMap::new();
//     }
//     let mut chars = input.chars();
//     let handles: Vec<_> = (0..)
//         .map(|_| {
//             chars
//                 .by_ref()
//                 .take((input.len() / worker_count).max(1))
//                 .collect()
//         })
//         .take_while(|s: &String| !s.is_empty())
//         .into_iter()
//         .map(|s| {
//             thread::spawn(move || {
//                 s.chars()
//                     .filter(|c| c.is_alphabetic())
//                     .map(|c| c.to_ascii_lowercase())
//                     .fold(HashMap::<char, usize>::new(), |mut acc, c| {
//                         *acc.entry(c).or_default() += 1;
//                         acc
//                     })
//             })
//         })
//         .collect();

//     handles
//         .into_iter()
//         .map(|handle| handle.join().unwrap())
//         .reduce(|mut acc, map| {
//             for (key, value) in map {
//                 *acc.entry(key).or_default() += value;
//             }
//             acc
//         })
//         .expect("thread maps are not empty")
// }