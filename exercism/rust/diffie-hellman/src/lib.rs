use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    mod_pow(b_pub, a, p)
}

// doesn't work for large numbers because of overflow
// this would be too straight forward, on to writing custom algorithms!
// fn mod_pow(base: u64, exp: u32, modulus: u64) -> u64 {
//     base.pow(exp) % modulus
// }

// https://rob.co.bb/posts/2019-02-10-modular-exponentiation-in-rust/
// https://en.wikipedia.org/wiki/Modular_exponentiation#Right-to-left_binary_method
// fn mod_pow(base: u64, mut exp: u64, modulus: u64) -> u64 {
//     if modulus == 1 {
//         return 0;
//     }
//     let mut result: u128 = 1;
//     let mut base: u128 = (base % modulus).into();
//     while exp > 0 {
//         if exp % 2 == 1 {
//             result = result * base % u128::from(modulus);
//         }
//         exp = exp >> 1;
//         base = base * base % u128::from(modulus)
//     }
//     result as u64
// }

// slightly modified
// https://docs.rs/mod_exp/1.0.1/src/mod_exp/lib.rs.html#37-63
pub fn mod_pow(base: u64, exponent: u64, modulus: u64) -> u64 {
    let mut result: u128 = 1;
    let mut base: u128 = (base % modulus).into();
    let mut exponent: u128 = exponent.into();
    let modulus: u128 = modulus.into();

    loop {
        if exponent <= 0 {
            break;
        }

        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }

        exponent = exponent >> 1;
        base = (base * base) % modulus;
    }

    // result never overflows, it is u128 in the algorithm to be able to use arithmetic with the other variables
    result as u64
}
