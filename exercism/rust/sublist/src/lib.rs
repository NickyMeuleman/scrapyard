use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist1<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let len_first = first_list.len();
    let len_second = second_list.len();
    if len_first == len_second {
        if first_list == second_list {
            Comparison::Equal
        } else {
            Comparison::Unequal
        }
    } else {
        // figure out which is the bigger list
        // iterate over windows the size of the smaller list
        // check if the window is equal to the smaller list
        if len_first > len_second {
            if len_second == 0
                || first_list
                    .windows(len_second)
                    .any(|window| window == second_list)
            {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        } else {
            if len_first == 0
                || second_list
                    .windows(len_first)
                    .any(|window| window == first_list)
            {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
    }
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    fn is_superlist<T: PartialEq>(longer: &[T], shorter: &[T]) -> bool {
        shorter.is_empty() || longer.windows(shorter.len()).any(|window| window == shorter)
    }

    match first_list.len().cmp(&second_list.len()) {
        Ordering::Equal if first_list == second_list => Comparison::Equal,
        Ordering::Greater if is_superlist(first_list, second_list) => Comparison::Superlist,
        Ordering::Less if is_superlist(second_list, first_list) => Comparison::Sublist,
        _ => Comparison::Unequal,
    }
}