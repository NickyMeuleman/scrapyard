// Question suggested BTreeSet, why that instead of a HashSet?
use std::collections::BTreeSet;

pub struct Triangle(usize);

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        // https://en.wikipedia.org/wiki/Triangle_inequality
        // In a valid triangle: 2 * max(a, b, c) < a + b + c
        // bonus: if any side is 0, triangle_inequality will be false
        let triangle_inequality = 2 * sides.iter().max().unwrap() < sides.iter().sum();
        match triangle_inequality {
            true => {
                let set: BTreeSet<&u64> = sides.iter().collect();
                Some(Self(set.len()))
            }
            false => None,
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.0 == 1
    }

    pub fn is_scalene(&self) -> bool {
        self.0 == 3
    }

    pub fn is_isosceles(&self) -> bool {
        self.0 == 2
    }
}
