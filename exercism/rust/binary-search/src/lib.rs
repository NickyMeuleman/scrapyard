use std::cmp::Ordering;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = array.len();
    while left < right {
        let mid = left + (right - left) / 2;
        match key.cmp(&array[mid]) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => right = mid,
            Ordering::Greater => left = mid + 1,
        }
    }
    None
}
