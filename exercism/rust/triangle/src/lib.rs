use std::collections::HashSet;

pub struct Triangle {
    different_side_count: usize,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        // https://en.wikipedia.org/wiki/Triangle_inequality
        // In a valid triangle: 2 * max(a, b, c) < a + b + c
        // bonus: if any side is 0, triangle_inequality will be false
        let triangle_inequality = 2 * sides.iter().max().unwrap() < sides.iter().sum();
        if triangle_inequality {
            let set: HashSet<u64> = sides.iter().copied().collect();
            let triangle = Self {
                different_side_count: set.len(),
            };
            Some(triangle)
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.different_side_count == 1
    }

    pub fn is_scalene(&self) -> bool {
        self.different_side_count == 3
    }

    pub fn is_isosceles(&self) -> bool {
        self.different_side_count == 2
    }
}
