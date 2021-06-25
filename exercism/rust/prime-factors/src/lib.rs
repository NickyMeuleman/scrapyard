const LARGEST_WANTED_PRIME: usize = 1_000_000;

// with precalculated primes
pub fn factors(mut n: usize) -> Vec<usize> {
    get_primes(LARGEST_WANTED_PRIME)
        .fold(Vec::new(), |mut acc, prime| {
            while n % prime == 0 {
                acc.push(prime);
                n /= prime;
            }
            acc
        })
}

// no precalculated primes.
// pub fn factors(mut n: usize) -> Vec<usize> {
//     let mut res = Vec::new();
//     let mut divisor = 2;
//     while n > 1 {
//         if n % divisor == 0 {
//             res.push(divisor);
//             n /= divisor;
//         } else {
//             divisor += 1;
//         }
//     }
//     res
// }

// no precalculated primes.
// pub fn factors(n: usize) -> Vec<usize> {
//     let mut curr = n;
//     let mut result = (2..)
//         .take_while(|i| i * i <= n)
//         .fold(Vec::new(), |mut acc, num| {
//             while curr % num == 0 {
//                 acc.push(num);
//                 curr /= num;
//             }
//             acc
//         });
//     if curr > 1 {
//         result.push(curr)
//     }
//     result
// }

// with precalculated primes
// pub fn factors(n: usize) -> Vec<usize> {
//     let primes: Vec<usize> = get_primes(LARGEST_WANTED_PRIME).collect();
//     let mut curr = n;
//     let mut prime_idx = 0;
//     let mut factors = Vec::new();
//     while curr > 1 {
//         if curr % primes[prime_idx] == 0 {
//             curr = curr / primes[prime_idx];
//             factors.push(primes[prime_idx]);
//         } else {
//             // only move on to next prime if curr is not divisible
//             prime_idx += 1;
//         }
//     }
//     factors
// }

/// Uses https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes to calculate all prime numbers up to the max
fn get_primes(max: usize) -> impl Iterator<Item = usize> {
    let mut sieve = vec![true; max + 1];
    sieve[0] = false;
    sieve[1] = false;
    // upper bound is sqrt of max
    for num in 2..=(max as f64).sqrt() as usize {
        if sieve[num] {
            // mark all multiples as not prime
            // start marking at num squared (everything below will already be marked).
            let mut multiple = num * num;
            while multiple <= max {
                sieve[multiple] = false;
                multiple += num;
            }
        }
    }
    // reached? sieve is done, put all unmarked numbers into an iterator
    sieve
        .into_iter()
        .enumerate()
        .filter_map(|(idx, bool)| if bool { Some(idx) } else { None })
}