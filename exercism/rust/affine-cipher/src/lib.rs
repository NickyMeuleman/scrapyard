// Wrote about this at https://nickymeuleman.netlify.app/blog/affine-cipher
const LOWERCASE_ASCII_A: u8 = 97;
const ALPHABET_LENGTH: u8 = 26;

/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    mmi(a, ALPHABET_LENGTH as i32).ok_or(AffineCipherError::NotCoprime(a))?;

    Ok(plaintext
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| {
            if c.is_numeric() {
                c
            } else {
                let x = c as u8 - LOWERCASE_ASCII_A;
                let mut num = a * x as i32 + b;
                num = num.rem_euclid(ALPHABET_LENGTH as i32);
                (num as u8 + LOWERCASE_ASCII_A) as char
            }
        })
        .enumerate()
        .fold(String::new(), |mut acc, (i, c)| {
            if i % 5 == 0 && i != 0 {
                acc.push(' ');
            }
            acc.push(c);
            acc
        }))
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let inverse = mmi(a, ALPHABET_LENGTH as i32).ok_or(AffineCipherError::NotCoprime(a))?;

    Ok(ciphertext
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| {
            if c.is_numeric() {
                c
            } else {
                let y = c as u8 - LOWERCASE_ASCII_A;
                let mut num = inverse * (y as i32 - b);
                num = num.rem_euclid(ALPHABET_LENGTH as i32);
                (num as u8 + LOWERCASE_ASCII_A) as char
            }
        })
        .collect())
}

// from bucatini-coder's solution
// Extended Euclidian Algorithm for greatest common divisor
// see https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
// for equation a*x + b*y == gcd(a,b)
// accepts (a, b)
// returns (g, x, y) where
// g == greatest common divisor of a, b
// x, y == Bézout coefficients satisfying gcd(a,b) == g == a*x + b*y
fn egcd(a: i32, b: i32) -> (i32, i32, i32) {
    match (a, b) {
        (0, _) => (b, 0, 1),
        (_, 0) => (a, 1, 0),
        _ => {
            let quotient = b / a;
            let remainder = b % a;
            let (g, x, y) = egcd(remainder, a);
            (g, y - quotient * x, x)
        }
    }
}
// Modular multiplicative inverse
// see https://en.wikipedia.org/wiki/Modular_multiplicative_inverse
// for a*x mod m == 1
// accepts (a, m)
// returns None where gcd(a, m) != 1 or Some(x) where gcd(a, m) == 1
fn mmi(a: i32, m: i32) -> Option<i32> {
    let (g, x, _) = egcd(a, m);
    match g {
        1 => Some(x.rem_euclid(m)),
        _ => None,
    }
}