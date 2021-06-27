struct Point {
    row: usize,
    col: usize,
    val: u64,
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    // loop through rows
    // find max of a row
    // push max to a temp vector. The index in the temp vector will be equal to the row coordinate of that max

    // loop through cols
    // find all minima in a col
    // if the minimum is in the temp array at that minimum's row coordinate, it's a ~~yeehaw~~ saddle point

    if input.is_empty() || input[0].is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();
    let temp: Vec<u64> = input
        .iter()
        .map(|row| row.iter().max().copied().unwrap())
        .collect();

    // make vec of cols. I know, very inefficient, I want to easily iterate through the cols 🤷‍♂️
    let mut cols = Vec::new();
    for col_idx in 0..input[0].len() {
        let col: Vec<u64> = input.iter().map(|row| row[col_idx]).collect();
        cols.push(col);
    }

    for (col_idx, col) in cols.iter().enumerate() {
        let min = col.iter().min().unwrap();
        let all_mins: Vec<Point> = col
            .iter()
            .enumerate()
            .filter(|&(_, num)| num == min)
            .map(|(row_idx, &num)| Point {
                row: row_idx,
                col: col_idx,
                val: num,
            })
            .collect();
        for point in all_mins {
            if point.val == temp[point.row] {
                result.push((point.row, point.col));
            }
        }
    }

    result
}
