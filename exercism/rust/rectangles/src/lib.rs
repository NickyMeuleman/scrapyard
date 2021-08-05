pub fn count(input: &[&str]) -> usize {
    // loop through all corner (row, col) indexes
    // treat them as the top_left corner of potential rectangles
    // loop through all row_offset and col_offset so the potential rectangle is 1 + row_offset high and 1 + col_offset wide
    // check if a potential rectangle has correct corners and correct edges, if it does, that's a bingo

    let corners = input.iter().enumerate().flat_map(|(row, line)| {
        line.chars()
            .enumerate()
            .filter_map(move |(col, c)| if c == '+' { Some((row, col)) } else { None })
    });

    let rows = input.len();
    let cols = input.get(0).map(|line| line.len()).unwrap_or(0);

    corners
        .flat_map(|(row, col)| {
            (1..)
                .take_while(move |i| i + col < cols)
                .flat_map(move |col_offset| {
                    (1..)
                        .take_while(move |i| i + row < rows)
                        .filter(move |&row_offset| {
                            has_corners(input, row, col, col_offset, row_offset).is_some()
                                && has_edges(input, row, col, col_offset, row_offset).is_some()
                        })
                })
        })
        .count()
}

fn has_corners(
    input: &[&str],
    row: usize,
    col: usize,
    col_offset: usize,
    row_offset: usize,
) -> Option<()> {
    // returning an Option to get early returns, how do you ideomatically do this with booleans?
    let top_right = input[row].chars().nth(col + col_offset)?;
    let bottom_left = input[row + row_offset].chars().nth(col)?;
    let bottom_right = input[row + row_offset].chars().nth(col + col_offset)?;

    (top_right == '+' && bottom_left == '+' && bottom_right == '+').then(|| ())
}

fn has_edges(
    input: &[&str],
    row: usize,
    col: usize,
    col_offset: usize,
    row_offset: usize,
) -> Option<()> {
    // returning an Option to get early returns, how do you ideomatically do this with booleans?

    // top
    input[row]
        .chars()
        .skip(col)
        .take(col_offset + 1)
        .all(|c| c == '+' || c == '-')
        .then(|| ())?;

    // bottom
    input[row + row_offset]
        .chars()
        .skip(col)
        .take(col_offset + 1)
        .all(|c| c == '+' || c == '-')
        .then(|| ())?;

    // left
    input
        .iter()
        .filter_map(|line| line.chars().nth(col))
        .skip(row)
        .take(row_offset + 1)
        .all(|c| c == '+' || c == '|')
        .then(|| ())?;

    // right
    input
        .iter()
        .filter_map(|line| line.chars().nth(col + col_offset))
        .skip(row)
        .take(row_offset + 1)
        .all(|c| c == '+' || c == '|')
        .then(|| ())?;

    Some(())
}

// with regular for loops and without early returns:
// pub fn count(lines: &[&str]) -> u32 {
//     let corners = lines.iter().enumerate().flat_map(|(row, line)| {
//         line.chars()
//             .enumerate()
//             .filter_map(move |(col, c)| if c == '+' { Some((row, col)) } else { None })
//     });
//     let rows = lines.len();
//     let cols = lines.get(0).map(|line| line.len()).unwrap_or(0);
//     let mut result = 0;
//     for (row, col) in corners {
//         for col_offset in (1..cols).take_while(|i| i + col < cols) {
//             for row_offset in (1..rows).take_while(|i| i + row < rows) {
//                 let right_top = lines[row].chars().nth(col + col_offset);
//                 let left_bottom = lines[row + row_offset].chars().nth(col);
//                 let right_bottom = lines[row + row_offset].chars().nth(col + col_offset);
//                 if right_top == Some('+') && left_bottom == Some('+') && right_bottom == Some('+') {
//                     // corners are valid, now check the edges
//                     let top_edge = lines[row]
//                         .chars()
//                         .skip(col)
//                         .take(col_offset + 1)
//                         .all(|c| c == '+' || c == '-');
//                     let bottom_edge = lines[row + row_offset]
//                         .chars()
//                         .skip(col)
//                         .take(col_offset + 1)
//                         .all(|c| c == '+' || c == '-');
//                     let left_edge = lines
//                         .iter()
//                         .filter_map(|line| line.chars().nth(col))
//                         .skip(row)
//                         .take(row_offset + 1)
//                         .all(|c| c == '+' || c == '|');
//                     let right_edge = lines
//                         .iter()
//                         .filter_map(|line| line.chars().nth(col + col_offset))
//                         .skip(row)
//                         .take(row_offset + 1)
//                         .all(|c| c == '+' || c == '|');
//                     if top_edge && bottom_edge && left_edge && right_edge {
//                         result += 1;
//                     }
//                 }
//             }
//         }
//     }
//     result
// }
