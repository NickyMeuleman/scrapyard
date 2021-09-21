use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut map: HashMap<&str, u32> =
        magazine
            .iter()
            .copied()
            .fold(HashMap::new(), |mut acc, item| {
                *acc.entry(item).or_default() += 1;
                acc
            });
    note.iter()
        .copied()
        .try_for_each(|part| {
            if let Some(count) = map.get_mut(part) {
                *count -= 1;
                if *count == 0 {
                    map.remove(part);
                }
                Ok(())
            } else {
                Err(())
            }
        })
        .is_ok()
}