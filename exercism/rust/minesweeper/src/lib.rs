pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.chars()
                .enumerate()
                .map(|(col_idx, c)| match c {
                    '*' => '*'.to_string(),
                    _ => match count_neighbours(minefield, row_idx, col_idx) {
                        0 => ' '.to_string(),
                        count => count.to_string(),
                    },
                })
                .collect()
        })
        .collect()
}

fn count_neighbours(input: &[&str], row: usize, col: usize) -> usize {
    input
        .iter()
        // skip one less than row, ensure valid number
        .skip((row as isize - 1).max(0) as usize)
        // take one more than row, remember row is 0 indexed
        .take((row + 2).min(3))
        // end up with at most 3 rows, 1 before idx to 1 after idx
        // now do the same for cols to end up with at most a 3 by 3 section
        .flat_map(|line| {
            line.chars()
                .skip((col as isize - 1).max(0) as usize)
                .take((col + 2).min(3))
        })
        .filter(|&c| c == '*')
        .count()
}
