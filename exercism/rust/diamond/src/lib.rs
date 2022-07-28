pub fn get_diamond(c: char) -> Vec<String> {
    let depth = (c as u8 - b'A') as i8;
    (-depth..=depth)
        .map(|row| {
            (-depth..=depth)
                .map(|col| {
                    if col.abs() + row.abs() == depth {
                        (b'A' + col.unsigned_abs()) as char
                    } else {
                        ' '
                    }
                })
                .collect()
        })
        .collect()
}
