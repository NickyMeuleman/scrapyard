use std::collections::HashMap;
use std::fs;

const DIRECTIONS: [Direction; 4] = [
    Direction::Top,
    Direction::Right,
    Direction::Bottom,
    Direction::Left,
];

const MONSTER_OFFSETS: [(usize, usize); 15] = [
    (0, 18),
    (1, 0),
    (1, 5),
    (1, 6),
    (1, 11),
    (1, 12),
    (1, 17),
    (1, 18),
    (1, 19),
    (2, 1),
    (2, 4),
    (2, 7),
    (2, 10),
    (2, 13),
    (2, 16),
];

#[derive(Debug, PartialEq, Copy, Clone)]
enum Cell {
    Occupied,
    Empty,
}

#[derive(Debug, PartialEq)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    grid: Vec<Vec<Cell>>
}

// tileID, tile
type Data = HashMap<usize, Vec<Vec<Cell>>>;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let data: Data = parse(&input);
    println!("Part one answer: {}", part_one(&data));
    println!("Part two answer: {}", part_two(&data));
}

fn parse(input: &str) -> Data {
    // a tile is 10x10
    let sections: Vec<&str> = input.split("\n\n").collect();
    // 144 sections
    let mut result: Data = HashMap::new();
    for section in sections {
        let parts: Vec<&str> = section.split(":\n").collect();
        let id: String = parts[0].chars().filter(|c| c.is_ascii_digit()).collect();
        let id: usize = id.parse().unwrap();
        let mut tile = Vec::new();
        for line in parts[1].lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                let cell = match c {
                    '#' => Cell::Occupied,
                    '.' => Cell::Empty,
                    _ => panic!("invalid cell value found"),
                };
                row.push(cell);
            }
            tile.push(row);
        }
        result.insert(id, tile);
    }
    result
}

fn part_one(data: &Data) -> usize {
    // question states: the outermost edges won't line up with any other tiles.
    // taking the shortcut where you look for tiles that only match on 2 side as you need to build the entire image for part2
    let mut side_ids: Vec<usize> = Vec::new();
    for (id, tile) in data {
        let borders = get_all_possible_borders(tile);
        let mut count = 0;
        for (_, other_tile) in data {
            let other_borders = get_all_possible_borders(other_tile);
            for border in &borders {
                // check if any of the other_borders match
                for other_border in &other_borders {
                    if border.iter().zip(other_border).all(|(a, b)| a == b) {
                        count += 1;
                    }
                }
            }
        }
        // we count every match twice (a matches b, but we also count that b matches a), so /2
        // and didn't bother to filter the tile we're searching out first, so -4
        if count / 2 - 4 == 2 {
            side_ids.push(*id);
        }
    }
    side_ids.iter().product()
}

fn part_two(data: &Data) -> usize {
    let mut picture: HashMap<(i32, i32), Tile> = HashMap::new();
    let mut available: Vec<Tile> = to_tile_vec(data);
    // get random tile and place it, pop removes it from the available pool
    let tile = available.pop().unwrap();
    picture.insert((0, 0), tile.clone());
    let mut to_search: Vec<((i32, i32), Tile)> = Vec::new();
    to_search.push(((0, 0), tile));
    let picture = make_picture(available, picture, to_search);
    // stitch together and remove edges
    // from q: The borders of each tile are not part of the actual image; start by removing them.
    let picture = stitch(picture);
    // look for number of monsters
    let total = picture
        .iter()
        .flatten()
        .filter(|&item| *item == Cell::Occupied)
        .count();
    let sea_monster_size = 15;
    let amount_of_monsters = count_monsters(picture);

    total - (amount_of_monsters * sea_monster_size)
}

fn get_all_possible_borders(tile: &Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    // return a vec of length 8, each inner vec is a border of length 10
    let top = tile[0].to_vec();
    let bottom = tile[9].to_vec();
    let mut left = Vec::new();
    let mut right = Vec::new();
    for row in tile {
        for (col_idx, cell) in row.iter().enumerate() {
            if col_idx == 0 {
                left.push(*cell);
            }
            if col_idx == 9 {
                right.push(*cell);
            }
        }
    }
    let mut top_reverse = top.clone();
    top_reverse.reverse();
    let mut bottom_reverse = bottom.clone();
    bottom_reverse.reverse();
    let mut left_reverse = left.clone();
    left_reverse.reverse();
    let mut right_reverse = right.clone();
    right_reverse.reverse();
    vec![
        top,
        top_reverse,
        right,
        right_reverse,
        bottom,
        bottom_reverse,
        left,
        left_reverse,
    ]
}

fn make_picture(
    mut available: Vec<Tile>,
    mut picture: HashMap<(i32, i32), Tile>,
    mut to_search: Vec<((i32, i32), Tile)>,
) -> HashMap<(i32, i32), Tile> {
    if available.len() == 0 {
        return picture;
    }
    // IT'S THE SEASON OF TREEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE
    // pop tile off of to_search
    let searching = to_search.pop();
    if let Some((coords, tile)) = searching {
        // for every direction, if there is no tile in that location already, get the border to search for
        for direction in DIRECTIONS.iter() {
            let new_coords = get_coords(&coords, &direction);
            // check if those coords are already filled before continuing
            if !picture.contains_key(&new_coords) {
                let border = get_border(&tile.grid, &direction);
                // search all available tiles for that border
                // to find a match, might need to rotate+flip the searched tile
                let complement_direction = get_complement_direction(&direction);
                // search for a tile that has a certain border in a certain direction
                let search_result = search_tiles(&border, complement_direction, &available);
                // if no search result is found, that means we are at the edge and only out of bounds places are available
                if let Some(found_tile) = search_result {
                    // place tile in coords+offset for that direction
                    picture.insert(new_coords, found_tile.clone());
                    // remove tile from available and add tile to to_search
                    let found_idx = available.iter().position(|tile| tile.id == found_tile.id);
                    if let Some(idx) = found_idx {
                        available.remove(idx);
                    } else {
                        panic!("index not found in available")
                    }
                    // optimization: remove the placed tile's complement direction from the directions vector
                    to_search.push((new_coords, found_tile));
                } else {
                    continue;
                }
            }
        }
    } else {
        panic!("Trying to search for a non extistent tile");
    }

    make_picture(available, picture, to_search)
}

fn to_tile_vec(data: &Data) -> Vec<Tile> {
    let mut result = Vec::new();
    // transform input to a vector of tiles
    for item in data {
        let tile = Tile {
            id: *item.0,
            grid: item.1.to_vec(),
        };
        result.push(tile);
    }
    result
}

fn get_coords(coords: &(i32, i32), direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::Top => (coords.0, coords.1 - 1),
        Direction::Right => (coords.0 + 1, coords.1),
        Direction::Bottom => (coords.0, coords.1 + 1),
        Direction::Left => (coords.0 - 1, coords.1),
    }
}

fn get_border(grid: &Vec<Vec<Cell>>, direction: &Direction) -> Vec<Cell> {
    match direction {
        Direction::Top => grid[0].to_vec(),
        Direction::Right => {
            let mut right = Vec::new();
            for row in grid {
                for (col_idx, cell) in row.iter().enumerate() {
                    if col_idx == grid.len() - 1 {
                        right.push(*cell);
                    }
                }
            }
            right
        }
        Direction::Bottom => grid[grid.len() - 1].to_vec(),
        Direction::Left => {
            let mut left = Vec::new();
            for row in grid {
                for (col_idx, cell) in row.iter().enumerate() {
                    if col_idx == 0 {
                        left.push(*cell);
                    }
                }
            }
            left
        }
    }
}

fn search_tiles(border: &Vec<Cell>, direction: Direction, available: &Vec<Tile>) -> Option<Tile> {
    // placeholder code to get all border, mixing part1 and part2
    for item in available {
        let borders = get_all_possible_borders(&item.grid);
        // if ANY border matches, return which one
        for side in borders.iter() {
            if side.iter().zip(border).all(|(a, b)| a == b) {
                // the tile has a matching border
                // transform tile until the border matches the direction passed in
                let rotated_grid = rotate_tile(&item.grid, border, &direction);
                return Some(Tile { grid: rotated_grid, ..*item});
            }
        }
    }

    None
}

fn get_complement_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::Top => Direction::Bottom,
        Direction::Right => Direction::Left,
        Direction::Bottom => Direction::Top,
        Direction::Left => Direction::Right,
    }
}

fn rotate_tile(grid: &Vec<Vec<Cell>>, target_border: &Vec<Cell>, target_direction: &Direction) -> Vec<Vec<Cell>> {
    for num in 0..4 {
        let grid = turn_clockwise(grid, num);
        let border = get_border(&grid, target_direction);
        if target_border.iter().zip(border).all(|(a, b)| *a == b) {
            return grid.clone();
        }
    }
    let grid = flip_vertical(grid);
    for num in 0..4 {
        let grid = turn_clockwise(&grid, num);
        let border = get_border(&grid, target_direction);
        if target_border.iter().zip(border).all(|(a, b)| *a == b) {
            return grid.clone();
        }
    }
    panic!("could not rotate tile to have a matching border");
}

fn stitch(picture: HashMap<(i32, i32), Tile>) -> Vec<Vec<Cell>> {
    // removes edges from each tile before stitching them all together
    // takes a hashmap of (x_coord, y_coord), Tile and returns a 2D Vec
    let min_x = picture.iter().map(|((x, _), _)| x).min().unwrap();
    let min_y = picture.iter().map(|((_, y), _)| y).min().unwrap();
    let max_x = picture.iter().map(|((x, _), _)| x).max().unwrap();
    let max_y = picture.iter().map(|((_, y), _)| y).max().unwrap();
    // delta between min and max should be 12 for each (11 because 0 is also counted)
    // since the picture is square and 12x12 = 144 (the amount of given tiles)
    // end grid should be 96x96, 12 pieces of length 8 in both directions
    let mut result = Vec::new();
    for y in *min_y..=*max_y {
        // each item has 10 rows of its own
        for inner_row in 1..9 {
            let mut row = Vec::new();
            for x in *min_x..=*max_x {
                let grid = &picture.get(&(x, y)).unwrap().grid;
                let to_add = grid[inner_row].clone();
                row.append(&mut to_add[1..9].to_vec());
            }
            result.push(row);
        }
    }
    result.clone()
}

fn count_monsters(grid: Vec<Vec<Cell>>) -> usize {
    let orientations = get_all_orientations(&grid);
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let mut count = 0;

    for mut grid in orientations {
        for row in 0..num_rows {
            for col in 0..num_cols {
                // check if monster
                // replace original Occupied tiles with Empty ones
                // todo if needed: replace with Monster tiles to support overlap and later check that not all tiles are Monster to avoid double counting
                if is_monster((row, col), &grid) {
                    count += 1;
                    for offset in &MONSTER_OFFSETS {
                        grid[row + offset.0][col + offset.1] = Cell::Empty;
                    }
                }
            }
        }
        if count > 0 {
            return count;
        }
    }
    count
}

fn is_monster(coord: (usize, usize), grid: &Vec<Vec<Cell>>) -> bool {
    MONSTER_OFFSETS
    .iter()
    .all(|(row_offset, col_offset)| {
       let option = grid
        .get(coord.0 + *row_offset)
        .and_then(|row| row.get(coord.1 + *col_offset));
        match option {
            Some(cell) => {
                match cell {
                    Cell::Empty => false,
                    Cell::Occupied => true,
                }
            },
            None => false
        }
    })
}

fn get_all_orientations(grid: &Vec<Vec<Cell>>) -> Vec<Vec<Vec<Cell>>> {
    let mut result = Vec::new();
    for num in 0..4 {
        let grid = turn_clockwise(grid, num);
        result.push(grid);
    }
    let grid = flip_vertical(grid);
    for num in 0..4 {
        let grid = turn_clockwise(&grid, num);
        result.push(grid);
    }
    result
}

fn turn_clockwise(grid: &Vec<Vec<Cell>>, amount: usize) -> Vec<Vec<Cell>> {
    if amount == 0 {
        return grid.clone();
    }
    let n = grid.len();
    let mut result = grid.clone();
    for i in 0..n {
        for j in i..n - i - 1 {
            let temp = grid[i][j];
            result[i][j] = grid[n - 1 - j][i];
            result[n - 1 - j][i] = grid[n - 1 - i][n - 1 - j];
            result[n - 1 - i][n - 1 - j] = grid[j][n - 1 - i];
            result[j][n - 1 - i] = temp;
        }
    }
    turn_clockwise(&result, amount - 1)
}

fn flip_vertical(grid: &Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    grid.clone().into_iter().rev().collect()
}

// Debugging helper functions:

fn print_grid(grid: &Vec<Vec<Cell>>) {
    let n = grid.len();
    for i in 0..n {
        let result: String = grid[i]
            .iter()
            .map(|x| match x {
                Cell::Empty => '.',
                Cell::Occupied => '#',
            })
            .collect();
        println!("{:?}", result);
    }
}

fn print_tile(tile: &Tile) {
    let n = tile.grid.len();
    for i in 0..n {
        let result: Vec<char> = tile.grid[i]
            .iter()
            .map(|x| match x {
                Cell::Empty => '.',
                Cell::Occupied => '#',
            })
            .collect();
        println!("{:?}", result);
    }
}
