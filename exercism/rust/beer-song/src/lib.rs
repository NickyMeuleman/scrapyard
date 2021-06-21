pub fn verse(n: u32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_owned(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_owned(),
        _ => format!(
            "{num} bottles of beer on the wall, {num} bottles of beer.\nTake one down and pass it around, {new_num} {new_bottles} of beer on the wall.\n",
            num = n,
            new_num = n - 1,
            new_bottles = if n - 1 == 1 { "bottle" } else { "bottles" },
        )
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<_>>()
        .join("\n")
}
