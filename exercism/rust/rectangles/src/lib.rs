#![feature(option_result_contains)]

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
                            let top_left = (row, col);
                            let top_right = (row, col + col_offset);
                            let bottom_left = (row + row_offset, col);
                            let bottom_right = (row + row_offset, col + col_offset);
                            let corners = [top_left, top_right, bottom_left, bottom_right];
                            has_corners(input, corners) && has_edges(input, corners)
                        })
                })
        })
        .count()
}

fn has_corners(input: &[&str], corners: [(usize, usize); 4]) -> bool {
    corners
        .iter()
        .all(|&(row, col)| input[row].chars().nth(col).contains(&'+'))
}

fn has_edges(input: &[&str], corners: [(usize, usize); 4]) -> bool {
    // first line is between top_left and top_right, second is between bottom_left and bottom_right
    let horizontal_points = [(corners[0], corners[1]), (corners[2], corners[3])];
    // first line is between top_left and bottom_left, second is between top_right and bottom_right
    let vertical_points = [(corners[0], corners[2]), (corners[1], corners[3])];

    horizontal_points
        .iter()
        .map(|&((row, col_left), (_, col_right))| {
            input[row]
                .chars()
                .skip(col_left)
                .take((col_right - col_left) + 1)
        })
        .all(|mut edge| edge.all(|c| c == '+' || c == '-'))
        .then(|| {
            vertical_points
                .iter()
                .map(|&((row_top, col), (row_bottom, _))| {
                    input
                        .iter()
                        .filter_map(move |line| line.chars().nth(col))
                        .skip(row_top)
                        .take((row_bottom - row_top) + 1)
                })
                .all(|mut edge| edge.all(|c| c == '+' || c == '|'))
        })
        .contains(&true)
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
