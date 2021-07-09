pub fn primes_up_to(upper_bound: usize) -> Vec<usize> {
    // initialise sieve that considers all numbers as prime
    let mut sieve = vec![true; upper_bound + 1];

    // 0 and 1 are not prime
    sieve[0] = false;
    sieve[1] = false;

    // iterate up to square root of upper_bound
    // reason: if num is not prime and one factor of num is bigger than sqrt(upper_bound),
    // an other factor _must_ be smaller than sqrt(upper_bound)
    // detecting one factor is enough to say a number is not prime
    for num in 2..=(upper_bound as f64).sqrt() as usize + 1 {
        // if sieve[num] is true, then num is prime
        if sieve[num] {
            // mark all multiples as not prime
            // start marking at num squared
            // every num below has already been marked in previous iterations
            for multiple in (num * num..=upper_bound).step_by(num) {
                sieve[multiple] = false;
            }
        }
    }

    // sieve is done, turn `true` into numbers
    sieve
        .iter()
        .enumerate()
        .filter_map(|(idx, flag)| match flag {
            true => Some(idx),
            false => None,
        })
        .collect()
}
