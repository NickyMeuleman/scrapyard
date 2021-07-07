use std::iter;
// Pascal triangle to 5 rows
// 1
// 1 1
// 1 2 1
// 1 3 3 1
// 1 4 6 4 1

// numbers are the sum of two numbers on the row above
// the one at index-1 plus the one at index (so left+right)
// if there is nothing at the same index, the number is 0
// if there is nothing at index-1, the number is 0
// Result of those last 2 rules: the start and end of a line is 1
// 1
// 0+1=1 1+0=1
// 0+1=1 1+1=2 1+0=1
// 0+1=1 1+2=3 2+1=3 1+0=1
// 0+1=1 1+3=4 3+3=6 3+1=4 1+0=1

pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        match self.row_count {
            0 => Vec::new(),
            1 => vec![vec![1]],
            num => {
                let mut rows = Self::new(num - 1).rows();
                let prev_row = rows.last().unwrap();
                let prepend_0 = iter::once(&0).chain(prev_row.iter());
                let append_0 = prev_row.iter().chain(iter::once(&0));
                let new_row = prepend_0.zip(append_0).map(|(n1, n2)| n1 + n2).collect();
                rows.push(new_row);
                rows
            }
        }
    }
}