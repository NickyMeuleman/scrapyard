// I decided not to resolve a solved problem, so CustomSet is a HashSet
// Maybe the question prohibits this, so the commented out solution uses a Vec instead
// The vec method requires T to be Ord, because that's the chosen method of deduplication
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, PartialEq)]
pub struct CustomSet<T: PartialEq + Eq + Hash> {
    set: HashSet<T>,
}

impl<T> CustomSet<T>
where
    T: PartialEq + Eq + Hash + Clone,
{
    pub fn new(input: &[T]) -> Self {
        Self {
            set: input.into_iter().cloned().collect(),
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.set.contains(element)
    }

    pub fn add(&mut self, element: T) {
        self.set.insert(element);
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.set.is_subset(&other.set)
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.set.is_disjoint(&other.set)
    }

    pub fn intersection(&self, other: &Self) -> Self {
        Self {
            set: self.set.intersection(&other.set).cloned().collect(),
        }
    }

    pub fn difference(&self, other: &Self) -> Self {
        Self {
            set: self.set.difference(&other.set).cloned().collect(),
        }
    }

    pub fn union(&self, other: &Self) -> Self {
        Self {
            set: self.set.union(&other.set).cloned().collect(),
        }
    }
}

// #[derive(Debug, PartialEq)]
// pub struct CustomSet<T> {
//     set: Vec<T>,
// }

// impl<T> CustomSet<T>
// where
//     // dedup relies on PartialEq. Tests pass without it, but it should be there
//     T: PartialEq + Clone + Ord,
// {
//     pub fn new(input: &[T]) -> Self {
//         let mut vec: Vec<T> = input.into_iter().cloned().collect();
//         vec.sort_unstable();
//         vec.dedup();

//         Self { set: vec }
//     }

//     pub fn contains(&self, element: &T) -> bool {
//         self.set.contains(element)
//     }

//     pub fn add(&mut self, element: T) {
//         self.set.push(element);
//         self.set.sort_unstable();
//         self.set.dedup();
//     }

//     pub fn is_subset(&self, other: &Self) -> bool {
//         self.set.iter().all(|item| other.contains(item))
//     }

//     pub fn is_empty(&self) -> bool {
//         self.set.is_empty()
//     }

//     pub fn is_disjoint(&self, other: &Self) -> bool {
//         self.set.iter().all(|item| !other.contains(item))
//     }

//     pub fn intersection(&self, other: &Self) -> Self {
//         let vec = self
//             .set
//             .iter()
//             .filter(|item| other.contains(item))
//             .cloned()
//             .collect();

//         Self { set: vec }
//     }

//     pub fn difference(&self, other: &Self) -> Self {
//         let vec = self
//             .set
//             .iter()
//             .filter(|item| !other.contains(item))
//             .cloned()
//             .collect();

//         Self { set: vec }
//     }

//     pub fn union(&self, other: &Self) -> Self {
//         let mut vec = self.set.clone();
//         vec.extend_from_slice(&other.set);
//         vec.sort_unstable();
//         vec.dedup();

//         Self { set: vec }
//     }
// }