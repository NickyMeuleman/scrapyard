const LARGEST_WANTED_PRIME: usize = 1_000_000;

/// Uses https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes to calculate all prime numbers up to the max
/// stops executing the algorithm when the wanted n-th prime is discovered before completion of the algorithm
fn get_primes(max: usize, nth: usize) -> Vec<usize> {
    let mut primes_found = 0;
    let mut sieve = vec![true; max + 1];
    sieve[0] = false;
    sieve[1] = false;
    // upper bound is sqrt of max
    for num in 2..=(max as f64).sqrt() as usize {
        if sieve[num] {
            // num is prime
            primes_found += 1;
            if primes_found == nth + 1 {
                break;
            }
            // mark all multiples as not prime
            // start marking at num squared (everything below will already be marked).
            let mut multiple = num * num;
            while multiple <= max {
                sieve[multiple] = false;
                multiple += num;
            }
        }
    }
    // reached? sieve is done, collect all unmarked numbers
    sieve
        .iter()
        .enumerate()
        .filter_map(|(idx, &bool)| if bool { Some(idx) } else { None })
        .collect()
}

pub fn nth(n: u32) -> u32 {
    *get_primes(LARGEST_WANTED_PRIME, n as usize)
        .iter()
        .nth(n as usize)
        .unwrap() as u32
}