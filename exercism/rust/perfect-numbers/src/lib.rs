use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    match num.cmp(&aliquot_sum(num)) {
        _ if num == 0 => None,
        Ordering::Equal => Some(Classification::Perfect),
        Ordering::Less => Some(Classification::Abundant),
        Ordering::Greater => Some(Classification::Deficient),
    }
}

fn aliquot_sum(num: u64) -> u64 {
    if num == 1 {
        return 0;
    }
    (2..=(num as f64).sqrt().floor() as u64).fold(1, |mut acc, n| {
        if num % n == 0 {
            acc += n;
            if n != num / n {
                acc += num / n;
            }
        }
        acc
    })
}
