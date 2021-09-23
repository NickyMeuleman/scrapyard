use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let chunks = input.chunks((input.len() / worker_count).max(1));
    let mut threads = Vec::new();

    for chunk in chunks {
        let string = chunk.join("");
        let handle = thread::spawn(move || {
            string
                .chars()
                .fold(HashMap::<char, usize>::new(), |mut acc, c| {
                    if c.is_alphabetic() {
                        *acc.entry(c.to_ascii_lowercase()).or_default() += 1;
                    }
                    acc
                })
        });
        threads.push(handle);
    }

    threads.into_iter().fold(HashMap::new(), |mut acc, handle| {
        for (key, value) in handle.join().unwrap() {
            *acc.entry(key).or_default() += value;
        }
        acc
    })
}
