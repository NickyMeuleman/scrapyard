pub fn build_proverb1(list: &[&str]) -> String {
    match list.is_empty() {
        true => String::new(),
        false => list
            .windows(2)
            .map(|window| format!("For want of a {} the {} was lost.", window[0], window[1]))
            .chain(std::iter::once(format!(
                "And all for the want of a {}.",
                list[0]
            )))
            .collect::<Vec<_>>()
            .join("\n"),
    }
}

pub fn build_proverb(list: &[&str]) -> String {
    match list.is_empty() {
        true => String::new(),
        false => list
            .windows(2)
            .map(|window| format!("For want of a {} the {} was lost.", window[0], window[1]))
            .chain(
                // first() returns an Option which implements IntoIterator and can be .chain()ed to an other iterator
                list.first()
                    .map(|item| format!("And all for the want of a {}.", item)),
            )
            .collect::<Vec<_>>()
            .join("\n"),
    }
}