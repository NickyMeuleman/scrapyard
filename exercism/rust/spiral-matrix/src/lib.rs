enum Direction {
    Right,
    Down,
    Left,
    Up,
}

pub fn spiral_matrix(size: usize) -> Vec<Vec<u32>> {
    let within_bounds = |row_idx, col_idx| {
        row_idx < (size as isize) && col_idx < (size as isize) && row_idx >= 0 && col_idx >= 0
    };

    let mut matrix = vec![vec![None; size]; size];
    let mut dir = Direction::Right;
    let mut row_idx = 0;
    let mut col_idx = 0;

    for num in 1..=size * size {
        // place a num
        matrix[row_idx][col_idx] = Some(num as u32);
        // get possible coords if move in current direction
        let (new_row, new_col) = match dir {
            Direction::Right => (row_idx as isize, col_idx as isize + 1),
            Direction::Down => (row_idx as isize + 1, col_idx as isize),
            Direction::Left => (row_idx as isize, col_idx as isize - 1),
            Direction::Up => (row_idx as isize - 1, col_idx as isize),
        };
        // if new location is out of bounds, or a Some() is there, turn
        // accessing an item with these coordinates works because the within_bounds change, and || short circuits
        if !within_bounds(new_row, new_col) || matrix[new_row as usize][new_col as usize].is_some()
        {
            dir = match dir {
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Up => Direction::Right,
            }
        }
        // do the move
        match dir {
            Direction::Right => col_idx += 1,
            Direction::Down => row_idx += 1,
            Direction::Left => col_idx -= 1,
            Direction::Up => row_idx -= 1,
        };
    }

    matrix
        .into_iter()
        .map(|row| row.into_iter().map(|item| item.unwrap()).collect())
        .collect()
}

// denenr's solution
// use std::iter;

// const VECTORS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

// pub fn spiral_matrix(size: usize) -> Vec<Vec<u32>> {
//     let mut matrix = vec![vec![0; size]; size];
//     let mut movement = VECTORS.iter().cycle();
//     let (mut x, mut y, mut n) = (-1, 0, 1..);

//     for (move_x, move_y) in iter::once(size)
//         .chain((1..size).rev().flat_map(|n| iter::repeat(n).take(2)))
//         .flat_map(|steps| iter::repeat(movement.next().unwrap()).take(steps))
//     {
//         x += move_x;
//         y += move_y;
//         matrix[y as usize][x as usize] = n.next().unwrap();
//     }

//     matrix
// }
