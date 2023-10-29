use aoc_all::{solve_part, Answer, Day, Part, Year};
use aoc_core_wasm::WasmSolution;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn solve(year: u16, day: u8, part: u8, input: String) -> Result<WasmSolution, JsError> {
    let year = Year::try_new(year)?;
    let day = Day::try_new(day)?;
    let part = Part::new(part);
    let answer = solve_part(&year, &day, &part, &input)?;
    // wasm bindgen can't handle enums with values yet
    // see: https://github.com/rustwasm/wasm-bindgen/issues/2407
    // I'd like to return an enum for the solved part that holds a string, but we can't.
    // That's why we return a WasmSolution for everything, even single parts,
    // A WasmSolution has Option<String> fields, None values turn into undefined in JS
    match answer {
        Answer::Part(result) => match part {
            Part::One => Ok(WasmSolution {
                part1: Some(result),
                part2: None,
            }),
            Part::Two => Ok(WasmSolution {
                part1: None,
                part2: Some(result),
            }),
            _ => unreachable!(),
        },
        Answer::Both(solution) => Ok(solution.into()),
    }
}
