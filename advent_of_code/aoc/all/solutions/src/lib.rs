pub use aoc_core::{get_input, Answer, Part, Solution};

pub fn print_part(year: u32, day: u8, part: &Part) {
    let part_solver = get_solver(year).unwrap();
    aoc_core::print_part(year, day, part, part_solver)
}

pub fn solve_part(year: u32, day: u8, part: &Part, input: &str) -> Result<Answer, String> {
    let part_solver = get_solver(year)?;
    part_solver(day, part, input)
}

fn get_solver(year: u32) -> Result<impl Fn(u8, &Part, &str) -> Result<Answer, String>, String> {
    match year {
        2023 => Ok(aoc2023::solve_part),
        _ => Err(format!("Unsupported year")),
    }
}
