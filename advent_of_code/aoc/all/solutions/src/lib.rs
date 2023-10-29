pub use aoc_core::{get_input, Answer, AoCError, Day, Part, Solution, Year};

pub fn print_part(year: &Year, day: &Day, part: &Part) {
    let part_solver = get_solver(year).unwrap();
    aoc_core::print_part(year, day, part, part_solver)
}

pub fn solve_part(year: &Year, day: &Day, part: &Part, input: &str) -> Result<Answer, AoCError> {
    let part_solver = get_solver(year)?;
    part_solver(day, part, input)
}

fn get_solver(
    year: &Year,
) -> Result<impl Fn(&Day, &Part, &str) -> Result<Answer, AoCError>, AoCError> {
    match year.value() {
        2023 => Ok(aoc2023::solve_part),
        _ => Err(AoCError::new("Unsupported year")),
    }
}
