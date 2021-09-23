use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.len() == 0 {
        return HashMap::new()
    }
    input
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
        .map(|handle| handle.join().unwrap())
        .reduce(|mut acc, map| {
            for (key, value) in map {
                *acc.entry(key).or_default() += value;
            }
            acc
        })
        .expect("thread maps are not empty")
}

// I'm a bit sad the iterator version is so much slower.
// They are supposed to be zero cost abstractions 😢
// Iterator version bench results
// test bench_large_parallel   ... bench:     819,007 ns/iter (+/- 204,413)
// test bench_large_sequential ... bench:     474,953 ns/iter (+/- 40,738)
// test bench_small_parallel   ... bench:     260,249 ns/iter (+/- 125,860)
// test bench_small_sequential ... bench:      16,563 ns/iter (+/- 1,761)
// test bench_tiny_parallel    ... bench:      74,991 ns/iter (+/- 10,403)
// test bench_tiny_sequential  ... bench:          56 ns/iter (+/- 4)
// Imperative version bench results
// test bench_large_parallel   ... bench:     304,838 ns/iter (+/- 16,081)
// test bench_large_sequential ... bench:     476,711 ns/iter (+/- 53,785)
// test bench_small_parallel   ... bench:     151,602 ns/iter (+/- 26,897)
// test bench_small_sequential ... bench:      16,706 ns/iter (+/- 825)
// test bench_tiny_parallel    ... bench:      74,920 ns/iter (+/- 7,201)
// test bench_tiny_sequential  ... bench:          55 ns/iter (+/- 3)

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
