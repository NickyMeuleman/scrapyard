use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    factors: Vec<(u64, u64)>,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Self {
        Self {
            factors: vec![(a, b)],
        }
    }

    fn replace(&mut self, a: u64, b: u64) {
        // replace instance, set a and b as only factor pair
        self.factors = vec![(a, b)];
    }

    fn value(&self) -> u64 {
        let (a, b) = self.factors.first().unwrap();
        a * b
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        // adds a and b as factor pair
        self.factors.push((a, b));
    }
}

fn is_palindrome(num: u64) -> bool {
    let mut temp = num;
    let mut flipped = 0;
    while temp > 0 {
        flipped *= 10;
        flipped += temp % 10;
        temp /= 10;
    }
    flipped == num
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut result: Option<(Palindrome, Palindrome)> = None;
    let palindromes = (min..=max)
        .flat_map(|a| (a..=max).map(move |b| (a, b)))
        .filter(|(a, b)| is_palindrome(a * b));

    for (a, b) in palindromes {
        if let Some((min, max)) = &mut result {
            // for every encountered palindrome, compare it to the existing min and max
            match (a * b).cmp(&min.value()) {
                Ordering::Less => min.replace(a, b),
                Ordering::Equal => min.insert(a, b),
                Ordering::Greater => (),
            };
            match (a * b).cmp(&max.value()) {
                Ordering::Less => (),
                Ordering::Equal => max.insert(a, b),
                Ordering::Greater => max.replace(a, b),
            };
        } else {
            // set the first encountered palindrome. The min and max are the same.
            result = Some((Palindrome::new(a, b), Palindrome::new(a, b)));
        }
    }

    result
}

// slightly slower version with .fold()
// pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
//     (min..=max)
//         .flat_map(|a| (a..=max).map(move |b| (a, b)))
//         .filter(|(a, b)| is_palindrome(a * b))
//         .fold(None, |mut acc, (a, b)| {
//             match &mut acc {
//                 Some((min, max)) => {
//                     match (a * b).cmp(&min.value()) {
//                         Ordering::Less => min.replace(a, b),
//                         Ordering::Equal => min.insert(a, b),
//                         Ordering::Greater => (),
//                     };
//                     match (a * b).cmp(&max.value()) {
//                         Ordering::Less => (),
//                         Ordering::Equal => max.insert(a, b),
//                         Ordering::Greater => max.replace(a, b),
//                     };
//                     acc
//                 }
//                 None => Some((Palindrome::new(a, b), Palindrome::new(a, b))),
//             }
//         })
// }

// version that creates a Palindrome stuct for every palindrome and at the end picks off the min and the max.
// use std::collections::BTreeMap;

// #[derive(Debug, PartialEq, Eq)]
// pub struct Palindrome {
//     factors: Vec<(u64, u64)>,
// }

// impl Palindrome {
//     pub fn new(a: u64, b: u64) -> Palindrome {
//         Self {
//             factors: vec![(a, b)],
//         }
//     }

//     pub fn value(&self) -> u64 {
//         let (a, b) = self.factors.first().unwrap();
//         a * b
//     }

//     pub fn insert(&mut self, a: u64, b: u64) {
//         self.factors.push((a, b));
//     }
// }

// fn is_palindrome(num: u64) -> bool {
//     let mut other = num;
//     let mut opposite_num = 0;
//     while other > 0 {
//         opposite_num *= 10;
//         opposite_num += other.rem_euclid(10);
//         other /= 10;
//     }
//     opposite_num == num
// }

// pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
//     let mut palindromes = BTreeMap::new();
//     for a in min..=max {
//         for b in a..=max {
//             let product = a * b;
//             if is_palindrome(product) {
//                 palindromes
//                     .entry(product)
//                     .and_modify(|palindrome: &mut Palindrome| palindrome.insert(a, b))
//                     .or_insert(Palindrome::new(a, b));
//             }
//         }
//     }

//     let mut it = palindromes.into_iter();
//     match (it.next(), it.next_back()) {
//         (Some((_, min)), Some((_, max))) => Some((min, max)),
//         _ => None,
//     }
// }
