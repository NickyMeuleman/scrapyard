pub use aoc_core::{get_input, Answer, AoCError, AoCResult, Day, Part, Solution, Year};

pub fn print_part(year: &Year, day: &Day, part: &Part) {
    let part_solver = get_solver(year).unwrap();
    aoc_core::print_part(year, day, part, part_solver)
}

pub fn solve_part(year: &Year, day: &Day, part: &Part, input: &str) -> AoCResult<Answer> {
    let part_solver = get_solver(year)?;
    part_solver(day, part, input)
}

fn get_solver(year: &Year) -> AoCResult<fn(&Day, &Part, &str) -> AoCResult<Answer>> {
    match year.value() {
        2015 => Ok(aoc2015::solve_part),
        2016 => Ok(aoc2016::solve_part),
        2017 => Ok(aoc2017::solve_part),
        2018 => Ok(aoc2018::solve_part),
        2019 => Ok(aoc2019::solve_part),
        2020 => Ok(aoc2020::solve_part),
        2021 => Ok(aoc2021::solve_part),
        2022 => Ok(aoc2022::solve_part),
        2023 => Ok(aoc2023::solve_part),
        _ => Err(AoCError::new("Unsupported year")),
    }
}
