// Mutex version bench results
// test bench_large_parallel   ... bench:     305,535 ns/iter (+/- 141,551)
// test bench_large_sequential ... bench:     486,663 ns/iter (+/- 42,248)
// test bench_small_parallel   ... bench:     153,638 ns/iter (+/- 19,008)
// test bench_small_sequential ... bench:      16,852 ns/iter (+/- 1,514)
// test bench_tiny_parallel    ... bench:      78,343 ns/iter (+/- 15,128)
// test bench_tiny_sequential  ... bench:          59 ns/iter (+/- 16)
// use std::collections::HashMap;
// use std::sync::{Arc, Mutex};
// use std::thread;

// pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
//     let result = Arc::new(Mutex::new(HashMap::new()));
//     let mut handles: Vec<_> = Vec::new();

//     for chunk in input.chunks((input.len() / worker_count).max(1)) {
//         let string = chunk.join("");
//         let cloned_result = Arc::clone(&result);
//         let handle = thread::spawn(move || {
//             let mut map: HashMap<char, usize> = HashMap::new();
//             let iter = string
//                 .chars()
//                 .filter(|c| c.is_alphabetic())
//                 .map(|c| c.to_ascii_lowercase());
//             for c in iter {
//                 *map.entry(c).or_default() += 1;
//             }
//             let mut result = cloned_result.lock().unwrap();
//             for (k, v) in map {
//                 *result.entry(k).or_default() += v;
//             }
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap()
//     }

//     // get the HashMap from the Arc<Mutex<HashMap>>
//     Arc::try_unwrap(result).unwrap().into_inner().unwrap()
// }

// This version blocks while a thread is trying to get the Mutex, effectively making it sequential with extra steps
// all threads do their individual work before aquiring the mutex
// use std::collections::HashMap;
// use std::sync::{Arc, Mutex};
// use std::thread;

// pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
//     let result = Arc::new(Mutex::new(HashMap::new()));
//     let mut handles: Vec<_> = Vec::new();

//     for chunk in input.chunks((input.len() / worker_count).max(1)) {
//         let string = chunk.join("");
//         let cloned_result = Arc::clone(&result);
//         let handle = thread::spawn(move || {
//             let mut result = cloned_result.lock().unwrap();
//             let iter = string
//                 .chars()
//                 .filter(|c| c.is_alphabetic())
//                 .map(|c| c.to_ascii_lowercase());
//             for c in iter {
//                 *result.entry(c).or_default() += 1;
//             }
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap()
//     }

//     // get the HashMap from the Arc<Mutex<HashMap>>
//     Arc::try_unwrap(result).unwrap().into_inner().unwrap()
// }
// 
// Channel version bench results
// test bench_large_parallel   ... bench:     317,746 ns/iter (+/- 18,042)
// test bench_large_sequential ... bench:     474,631 ns/iter (+/- 28,118)
// test bench_small_parallel   ... bench:     149,051 ns/iter (+/- 16,816)
// test bench_small_sequential ... bench:      16,439 ns/iter (+/- 909)
// test bench_tiny_parallel    ... bench:      72,586 ns/iter (+/- 18,529)
// test bench_tiny_sequential  ... bench:          53 ns/iter (+/- 7)
// use std::collections::HashMap;
// use std::mem;
// use std::sync::mpsc;
// use std::thread;

// pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
//     let (sender, receiver) = mpsc::channel();

//     for chunk in input.chunks((input.len() / worker_count).max(1)) {
//         let sender = sender.clone();
//         let string = chunk.join("");
//         thread::spawn(move || {
//             let map = string
//                 .chars()
//                 .filter(|c| c.is_alphabetic())
//                 .map(|c| c.to_ascii_lowercase())
//                 .fold(HashMap::<char, usize>::new(), |mut acc, c| {
//                     *acc.entry(c).or_default() += 1;
//                     acc
//                 });
//             sender.send(map).unwrap();
//         });
//     }

//     // drop the original sender, else the channel will remain open, causing the receiver to infinitely wait
//     mem::drop(sender);

//     receiver
//         .iter()
//         .reduce(|mut acc, map| {
//             for (key, value) in map {
//                 *acc.entry(key).or_default() += value;
//             }
//             acc
//         })
//         .unwrap_or(HashMap::new())
// }

// Iterator version that uses the internal type of the joinhandle bench results:
// test bench_large_parallel   ... bench:     316,377 ns/iter (+/- 26,063)
// test bench_large_sequential ... bench:     477,191 ns/iter (+/- 106,096)
// test bench_small_parallel   ... bench:     153,522 ns/iter (+/- 26,236)
// test bench_small_sequential ... bench:      16,581 ns/iter (+/- 1,343)
// test bench_tiny_parallel    ... bench:      77,122 ns/iter (+/- 12,552)
// test bench_tiny_sequential  ... bench:          55 ns/iter (+/- 4)
// use std::collections::HashMap;
// use std::thread;

// pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
//     if input.len() == 0 {
//         return HashMap::new();
//     }

//     let handles: Vec<_> = input
//         .chunks((input.len() / worker_count).max(1))
//         .map(|chunk| {
//             let string = chunk.join("");
//             thread::spawn(move || {
//                 string
//                     .chars()
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

// Iterator version before fix bench results
// test bench_large_parallel   ... bench:     819,007 ns/iter (+/- 204,413)
// test bench_large_sequential ... bench:     474,953 ns/iter (+/- 40,738)
// test bench_small_parallel   ... bench:     260,249 ns/iter (+/- 125,860)
// test bench_small_sequential ... bench:      16,563 ns/iter (+/- 1,761)
// test bench_tiny_parallel    ... bench:      74,991 ns/iter (+/- 10,403)
// test bench_tiny_sequential  ... bench:          56 ns/iter (+/- 4)
// The fix in iterative version mentioned:
// before, I was using one big iterator chain and wasn't waiting to join the handles until all threads had been spawned
// By collecting all joinHandles into a vector first,
// the code only starts joining once all threads have been spawned.

// Imperative version that uses the internal type of the joinhandle bench results
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

// A version that spreads equal amounts of chars across threads is slightly slower,
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

// Code from blogpost https://nickymeuleman.netlify.app/blog/multithreading-rust

// JOINHANDLES
// use std::collections::HashMap;
// use std::thread;

// pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
//     let mut result: HashMap<char, usize> = HashMap::new();
//     let chunks = input.chunks((input.len() / worker_count).max(1));
//     let mut handles = Vec::new();

//     for chunk in chunks {
//         let string = chunk.join("");
//         // return a HashMap from each thread, the JoinHandle wraps this hashmap
//         let handle = thread::spawn(move || {
//             let mut map: HashMap<char, usize> = HashMap::new();
//             for c in string.chars().filter(|c| c.is_alphabetic()) {
//                 *map.entry(c.to_ascii_lowercase()).or_default() += 1;
//             }
//             map
//         });
//         handles.push(handle);
//     }

//     // wait for each thread to finish and combine every HashMap into the final result
//     for handle in handles {
//         let map = handle.join().unwrap();
//         for (key, value) in map {
//             *result.entry(key).or_default() += value;
//         }
//     }
//     result
// }

// CHANNEL
// use std::collections::HashMap;
// use std::mem;
// use std::sync::mpsc;
// use std::thread;

// pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
//     let mut result: HashMap<char, usize> = HashMap::new();
//     let chunks = input.chunks((input.len() / worker_count).max(1));
//     let (sender, receiver) = mpsc::channel();

//     for chunk in chunks {
//         let sender = sender.clone();
//         let string = chunk.join("");
//         thread::spawn(move || {
//             // Solve each chunk and send the resulting HashMap down the channel
//             let mut map: HashMap<char, usize> = HashMap::new();
//             for c in string.chars().filter(|c| c.is_alphabetic()) {
//                 *map.entry(c.to_ascii_lowercase()).or_default() += 1;
//             }
//             sender.send(map).unwrap();
//         });
//     }

//     // drop the original sender, else the channel will remain open, causing the receiver to infinitely wait
//     mem::drop(sender);

//     // combine every received HashMap
//     for received in receiver {
//         for (key, value) in received {
//             *result.entry(key).or_default() += value;
//         }
//     }

//     result
// }

// MUTEX
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let result = Arc::new(Mutex::new(HashMap::new()));
    let chunks = input.chunks((input.len() / worker_count).max(1));
    let mut handles: Vec<_> = Vec::new();

    for chunk in chunks {
        let string = chunk.join("");
        let result = Arc::clone(&result);
        let handle = thread::spawn(move || {
            let mut map: HashMap<char, usize> = HashMap::new();
            // create a HashMap for this chunk
            for c in string.chars().filter(|c| c.is_alphabetic()) {
                *map.entry(c.to_ascii_lowercase()).or_default() += 1;
            }
            // add the HashMap of this chunk to the HashMap that is wrapped by the Mutex
            let mut result = result.lock().unwrap();
            for (key, value) in map {
                *result.entry(key).or_default() += value;
            }
        });
        handles.push(handle);
    }

    // wait for each thread to finish
    for handle in handles {
        handle.join().unwrap()
    }

    // get the HashMap from the Arc<Mutex<HashMap>>
    Arc::try_unwrap(result).unwrap().into_inner().unwrap()
}