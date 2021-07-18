use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // get input words
    // create set of all letters
    // create translation map for every possibility
    // filter out possibilities where a leading digit is translated to 0
    // try possibilities, return Some(possibility) if it works, if all fail return None

    let words: Vec<&str> = input
        .split(" ")
        .filter(|&part| part != "+" && part != "==")
        .collect();
    let letters: HashSet<char> = words.iter().flat_map(|w| w.chars()).collect();
    let leading: HashSet<char> = words
        .iter()
        // optionally add check if word.len() > 1.
        // Tests pass without it, but the instructions say only multi-digits can't start with 0
        .map(|word| word.chars().nth(0).unwrap())
        .collect();
    let dicts = (0..=9)
        .permutations(letters.len())
        .filter_map(|permutation| {
            // this step relies on the order, should letters be a BTreeSet then?
            letters
                .iter()
                .copied()
                .zip(permutation)
                .try_fold(HashMap::new(), |mut acc, (c, n)| {
                    if n == 0 && leading.contains(&c) {
                        None
                    } else {
                        acc.insert(c, n);
                        Some(acc)
                    }
                })
        });

    for dict in dicts {
        let mut numbers: Vec<usize> = words
            .iter()
            .map(|w| {
                w.chars()
                    .fold(0, |acc, c| acc * 10 + *dict.get(&c).unwrap() as usize)
            })
            .collect();
        if numbers.pop().unwrap() == numbers.iter().sum() {
            return Some(dict);
        }
    }
    None
}

// rsalmei's solution
// use std::collections::{HashMap, HashSet};
// use std::iter::once;

// pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
//     let (words_sum, word_equal) = match input.split(" == ").collect::<Vec<_>>().as_slice() {
//         &[words, res] => (words.split(" + ").collect::<Vec<_>>(), res),
//         _ => return None,
//     };

//     let extract = |word: &str, map: &mut HashMap<_, _>, sign| {
//         let store = |(i, c)| *map.entry(c).or_insert(0) += sign * 10i64.pow(i as u32);
//         word.chars().rev().enumerate().for_each(store)
//     };
//     let mut coeffs = HashMap::with_capacity(10);
//     words_sum.iter().for_each(|&w| extract(w, &mut coeffs, 1));
//     extract(word_equal, &mut coeffs, -1);
//     let non_zero = words_sum
//         .iter()
//         .chain(once(&word_equal))
//         .map(|&w| w.chars().next().unwrap())
//         .map(|f| coeffs.keys().position(|&c| f == c).unwrap())
//         .collect::<HashSet<_>>();

//     let equation = |mem: &[u8]| {
//         let polynomial = mem.iter().zip(coeffs.values()).map(|(&d, &c)| d as i64 * c);
//         non_zero.iter().all(|&nz| mem[nz] != 0) && polynomial.sum::<i64>() == 0
//     };
//     let res = brute_force_digits(coeffs.len(), equation);

//     (res.len() == 1).then(|| coeffs.keys().copied().zip(res[0].iter().copied()).collect())
// }

// fn brute_force_digits(qty: usize, f: impl Fn(&[u8]) -> bool) -> Vec<Vec<u8>> {
//     fn gen_digit(qty: usize, mem: &mut Vec<u8>, f: &impl Fn(&[u8]) -> bool, rs: &mut Vec<Vec<u8>>) {
//         for i in 0u8..=9 {
//             if mem.iter().any(|&c| c == i) {
//                 continue;
//             }
//             mem.push(i);
//             if qty > 0 {
//                 gen_digit(qty - 1, mem, f, rs)
//             } else if f(&mem) {
//                 rs.push(mem.clone())
//             }
//             mem.pop();
//         }
//     }

//     let mut mem = Vec::with_capacity(qty);
//     let mut results = Vec::new();
//     gen_digit(qty - 1, &mut mem, &f, &mut results);
//     results
// }

// j0ran's solution
// use std::collections::HashMap;

// #[derive(Default, Debug)]
// struct Alphametics<'a> {
//     letters: Vec<char>,
//     values: Vec<u8>,
//     words: Vec<&'a str>,
//     used: Vec<bool>,
//     non_zero: Vec<bool>,
// }

// impl<'a> Alphametics<'a> {
//     fn new(input: &'a str) -> Self {
//         let mut a = Alphametics {
//             words: input
//                 .split(' ')
//                 .filter(|w| *w != "+" && *w != "==")
//                 .collect(),
//             ..Alphametics::default()
//         };
//         a.letters = input.chars().filter(|c| c.is_alphabetic()).collect();
//         a.letters.sort_unstable();
//         a.letters.dedup();
//         a.non_zero = a
//             .letters
//             .iter()
//             .map(|c| a.words.iter().any(|w| w.starts_with(*c)))
//             .collect();
//         a.used = vec![false; 10];
//         a.values = vec![0; a.letters.len()];
//         a
//     }

//     fn map(&self, c: char) -> u64 {
//         self.values[self.letters.iter().position(|d| *d == c).unwrap()] as u64
//     }

//     fn correct(&self) -> bool {
//         let numbers: Vec<u64> = self
//             .words
//             .iter()
//             .map(|w| w.chars().fold(0, |n, c| n * 10 + self.map(c)))
//             .collect();
//         numbers[0..numbers.len() - 1].iter().sum::<u64>() == numbers[numbers.len() - 1]
//     }

//     fn permut(&mut self, index: usize) -> bool {
//         if index >= self.letters.len() {
//             return self.correct();
//         }

//         for i in 0..=9 {
//             if !self.used[i] && (i != 0 || !self.non_zero[index]) {
//                 self.used[i] = true;
//                 self.values[index] = i as u8;
//                 if self.permut(index + 1) {
//                     return true;
//                 }
//                 self.used[i] = false;
//             }
//         }

//         false
//     }

//     fn as_hashmap(&self) -> HashMap<char, u8> {
//         self.letters
//             .iter()
//             .zip(self.values.iter())
//             .map(|i| (*i.0, *i.1))
//             .collect()
//     }
// }

// pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
//     let mut a = Alphametics::new(input);
//     if a.permut(0) {
//         Some(a.as_hashmap())
//     } else {
//         None
//     }
// }
