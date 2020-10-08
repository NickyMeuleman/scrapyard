// Given a list of integers, use a vector and return the mean (the average value),
// median (when sorted, the value in the middle position),
// and mode (the value that occurs most often;
// a hash map will be helpful here) of the list.

use std::collections::HashMap;
use std::convert::TryInto;

fn mean(vec: &Vec<i32>) -> f64 {
    let len: i32 = vec.len().try_into().unwrap();
    let len: f64 = len.into();
    let sum: i32 = vec.iter().sum();
    let sum: f64 = sum.into();

    sum / len
}

fn median(vec: &mut Vec<i32>) -> f64 {
    vec.sort();

    let len = vec.len();
    let mid_idx = len / 2;

    if len % 2 == 0 {
        mean(&vec![vec[mid_idx - 1], vec[mid_idx]])
    } else {
        vec[mid_idx].into()
    }
}

fn mode(vec: &Vec<i32>) -> i32 {
    let mut counts = HashMap::new();

    for number in vec {
        let count = counts.entry(number).or_insert(0);
        *count = *count + 1
    }

    // can I do this without initializing these variables to 0?
    let mut highest_count = 0;
    let mut most_common_int = 0;
    for (int, count) in counts {
        if count > highest_count {
            highest_count = count;
            most_common_int = *int;
        }
    }
    most_common_int

    // alternative for the variable declarations and for loop:
    // counts
    //     .into_iter()
    //     .max_by_key(|&(_, count)| count)
    //     .map(|(value, _)| *value)
    //     .unwrap()
}

fn main() {
    let mut numbers: Vec<i32> = vec![1, 3, 4, 5, 6, 2, 7, 8, 8];

    println!("list: {:?}", &numbers);
    println!("mean: {}", mean(&numbers));
    println!("median: {}", median(&mut numbers));
    println!("mode: {}", mode(&numbers));
}
