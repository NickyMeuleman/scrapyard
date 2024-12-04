use crate::{AoCData, AoCResult};
use std::fmt::Display;

// up, up-right, right, down-right, down, down-left, left, up-left
const DIRS: [(i32, i32); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

#[derive(Debug, Clone)]
pub struct Data<'a>(Vec<&'a [u8]>);

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(
            input
                .lines()
                .map(|line| line.as_bytes())
                .collect(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let grid = &self.0;
        let mut count = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] != b'X' {
                    continue;
                }
                let r = r as i32;
                let c = c as i32;
                for (dr, dc) in DIRS {
                    // bounds check (last char has to be inside the grid)
                    let final_r = r + dr * 3;
                    let final_c = c + dc * 3;
                    if final_r < 0
                        || final_c < 0
                        || final_r >= grid.len() as i32
                        || final_c >= grid[0].len() as i32
                    {
                        continue;
                    }

                    if &[
                        grid[(r + dr) as usize][(c + dc) as usize],
                        grid[(r + dr * 2) as usize][(c + dc * 2) as usize],
                        grid[(r + dr * 3) as usize][(c + dc * 3) as usize],
                    ] == b"MAS"
                    {
                        count += 1;
                    }
                }
            }
        }

        Ok(count)

        // (0..grid.len())
        //     // turn into iterator over (row_idx, col_idx)
        //     .flat_map(|r| (0..grid[0].len()).map(move |c| (r, c)))
        //     // first char must be X
        //     .filter(|(r, c)| grid[*r][*c] == b'X')
        //     // map into indexes in one direction
        //     .flat_map(|(r, c)| {
        //         DIRS.iter().map(move |(dr, dc)| {
        //             [
        //                 (r as i32 + dr, c as i32 + dc),
        //                 (r as i32 + 2 * dr, c as i32 + 2 * dc),
        //                 (r as i32 + 3 * dr, c as i32 + 3 * dc),
        //             ]
        //         })
        //     })
        //     // bounds check the last letter
        //     .filter(|&[_, _, (r, c)]| {
        //         (r >= 0 && r < grid.len() as i32) && (c >= 0 && c < grid[0].len() as i32)
        //     })
        //     // turn indexes back into usizes
        //     .map(|[(r1, c1), (r2, c2), (r3, c3)]| {
        //         [
        //             (r1 as usize, c1 as usize),
        //             (r2 as usize, c2 as usize),
        //             (r3 as usize, c3 as usize),
        //         ]
        //     })
        //     // turn idxes into letters
        //     .map(|[(r1, c1), (r2, c2), (r3, c3)]| [grid[r1][c1], grid[r2][c2], grid[r3][c3]])
        //     // check if word spells MAS
        //     .filter(|word| word == b"MAS")
        //     .count()
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let grid = &self.0;
        let mut count = 0;
        for r in 1..grid.len() - 1 {
            for c in 1..grid[0].len() - 1 {
                if grid[r][c] != b'A' {
                    continue;
                }
                let circle = [
                    grid[r - 1][c - 1],
                    grid[r - 1][c + 1],
                    grid[r + 1][c + 1],
                    grid[r + 1][c - 1],
                ];
                if &circle == b"MSSM"
                    || &circle == b"MMSS"
                    || &circle == b"SMMS"
                    || &circle == b"SSMM"
                {
                    count += 1;
                }
            }
        }
        Ok(count)

        // (1..grid.len() - 1)
        //     // turn into iterator over (row_idx, col_idx)
        //     .flat_map(|r| (1..grid[0].len() - 1).map(move |c| (r, c)))
        //     // middle char must be A
        //     .filter(|(r, c)| grid[*r][*c] == b'A')
        //     // turn into up-left, up-right, down-right, down-left
        //     .map(|(r, c)| {
        //         [
        //             grid[r - 1][c - 1],
        //             grid[r - 1][c + 1],
        //             grid[r + 1][c + 1],
        //             grid[r + 1][c - 1],
        //         ]
        //     })
        //     // only accept valid circles
        //     .filter(|circle| {
        //         circle == b"MSSM" || circle == b"MMSS" || circle == b"SMMS" || circle == b"SSMM"
        //     })
        //     .count()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "18");
    }

    #[test]
    fn part_2() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "9");
    }
}
