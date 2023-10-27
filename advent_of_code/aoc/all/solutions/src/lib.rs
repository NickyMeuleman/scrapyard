pub use aoc_core::{get_input, Part};

pub fn print_part(year: u32, day: u8, part: &Part) {
    let part_solver = match year {
        2023 => Ok(aoc2023::solve_part),
        _ => Err(format!("Unsupported year")),
    };
    aoc_core::print_part(year, day, part, part_solver.unwrap())
}
