#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Bucket {
    One,
    Two,
}

impl Bucket {
    fn other(&self) -> Self {
        match self {
            Bucket::One => Bucket::Two,
            Bucket::Two => Bucket::One
        }
    }
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    // local variable bucket_1 is the start bucket, bucket_2 is the other bucket
    let (bucket_1_capacity, bucket_2_capacity) = match start_bucket {
        Bucket::One => (capacity_1, capacity_2),
        Bucket::Two => (capacity_2, capacity_1),
    };

    // initial fill
    let mut bucket_1 = bucket_1_capacity;
    let mut bucket_2 = 0;
    let mut moves = 1;

    // start bucket has goal size, return stats
    if goal == bucket_1_capacity {
        return Some(BucketStats {
            moves,
            goal_bucket: start_bucket.clone(),
            other_bucket: bucket_2,
        });
    }

    // other bucket has goal size, fill it and return stats
    if goal == bucket_2_capacity {
        return Some(BucketStats {
            moves: moves + 1,
            goal_bucket: start_bucket.other(),
            other_bucket: bucket_1,
        });
    }

    loop {
        // pour start bucket into other bucket.
        let pouring = bucket_1.min(bucket_2_capacity - bucket_2);
        bucket_1 = bucket_1 - pouring;
        bucket_2 = bucket_2 + pouring;
        moves += 1;

        // check if the goal is met
        if bucket_1 == goal {
            return Some(BucketStats {
                moves,
                goal_bucket: start_bucket.clone(),
                other_bucket: bucket_2,
            });
        }
        if bucket_2 == goal {
            return Some(BucketStats {
                moves,
                goal_bucket: start_bucket.other(),
                other_bucket: bucket_1,
            });
        }

        // Whenever the start bucket becomes empty fill it.
        if bucket_1 == 0 {
            bucket_1 = bucket_1_capacity;
            moves += 1;
        };

        // Whenever the other becomes full empty it.
        if bucket_2 == bucket_2_capacity {
            bucket_2 = 0;
            moves += 1;
        };

        // if initial levels are reached, we made a loop and goal wasn't found
        if bucket_1 == bucket_1_capacity && bucket_2 == 0 {
            return None;
        }
    }
}

// rsalmei's solution
// use std::collections::HashSet;
// use std::iter::FromIterator;
// use std::mem;

// #[derive(PartialEq, Eq, Debug)]
// pub enum Bucket {
//     One,
//     Two,
// }

// #[derive(PartialEq, Eq, Debug)]
// pub struct BucketStats {
//     pub moves: u8,
//     pub goal_bucket: Bucket,
//     pub other_bucket: u8,
// }

// pub fn solve(bucket_1: u8, bucket_2: u8, goal: u8, start_bucket: &Bucket) -> Option<BucketStats> {
//     let mut seen = HashSet::<_>::from_iter(vec![(0, 0), (bucket_1, 0), (0, bucket_2)]);
//     let mut states = vec![match start_bucket {
//         Bucket::One => (bucket_1, 0),
//         Bucket::Two => (0, bucket_2),
//     }];

//     let (mut moves, mut swap) = (1, vec![]);
//     loop {
//         if let Some((goal_bucket, other_bucket)) = states.iter().find_map(|&(a, b)| {
//             (a == goal)
//                 .then(|| (Bucket::One, b))
//                 .or_else(|| (b == goal).then(|| (Bucket::Two, a)))
//         }) {
//             break Some(BucketStats {
//                 moves,
//                 goal_bucket,
//                 other_bucket,
//             });
//         } else if states.is_empty() {
//             break None;
//         }

//         moves += 1;
//         states.drain(..).for_each(|(a, b)| {
//             let (ab, ba) = ((a + b).min(bucket_2), (a + b).min(bucket_1));
//             let (ma, mb) = ((a + b - ab, ab), (ba, a + b - ba));
//             let maybe_states = [(a, 0), (a, bucket_2), (0, b), (bucket_1, b), ma, mb];
//             swap.extend(maybe_states.iter().filter(|&&state| seen.insert(state)));
//         });

//         mem::swap(&mut states, &mut swap);
//     }
// }
