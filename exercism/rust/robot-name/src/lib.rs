use std::collections::HashSet;
use std::cell::RefCell;
use rand::prelude::*;

// jgilray's solution with comments and an alternative way to gen the random name
thread_local! {
    // RefCell to provide interior mutability since HASHET is immutable
    pub static HASHSET: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
}

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            name: Robot::generate_unique_random_name(),
        }
    }

    fn generate_unique_random_name() -> String {
        let mut name: String = String::new();
        HASHSET.with(|cell|  // performs lazy initialization of the thread local static
            loop {
                let mut rng = rand::thread_rng();
                // let chars = rand::distributions::Uniform::new_inclusive('A', 'Z');
                // let nums = rand::distributions::Uniform::new_inclusive(0, 9);
                // let picked_chars: String = (&mut rng).sample_iter(chars).take(2).collect();
                // let picked_nums: String = (&mut rng).sample_iter(nums).take(3).map(|n| n.to_string()).collect();
                // name = picked_chars + &picked_nums;
                name = format!(
                    "{}{}{:03}",
                    rng.gen_range(b'A'..=b'Z') as char,
                    rng.gen_range(b'A'..=b'Z') as char,
                    rng.gen_range(0..=999)
                );
                if !cell.borrow().contains(&name) {
                    cell.borrow_mut().insert(name.clone());
                    break;
                }
            }
        );
        name
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = Robot::generate_unique_random_name();
    }
}

// Gabirel's solution
// use std::collections::HashSet;

// use lazy_static::lazy_static;
// use rand::{thread_rng, Rng};

// lazy_static! {
//     // no need to use mutex
//     static ref USED_ROBOT_NAMES: HashSet<String> = HashSet::new();
// }

// pub struct Robot {
//     name: String,
// }

// impl Default for Robot {
//     fn default() -> Self {
//         Self {
//             name: generate_unique_name(),
//         }
//     }
// }

// impl Robot {
//     pub fn new() -> Self {
//         Default::default()
//     }

//     pub fn name(&self) -> &str {
//         self.name.as_ref()
//     }

//     pub fn reset_name(&mut self) {
//         self.name = generate_unique_name();
//     }
// }

// fn generate_unique_name() -> String {
//     const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
//     let mut rng = rand::thread_rng();
//     loop {
//         let letter: String = (0..2)
//             .map(|_| {
//                 let idx = rng.gen_range(0..=CHARSET.len());
//                 CHARSET[idx] as char
//             })
//             .collect();
//         let number: u32 = thread_rng().gen_range(0..=999);
//         let name = format!("{}{}", letter, number);
//         if !USED_ROBOT_NAMES.contains(&name) {
//             return name;
//         }
//     }
// }