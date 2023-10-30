use aoc2022::{solve_part, Answer, Day, Part};
use aoc_core_wasm::WasmSolution;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn solve(day: u8, part: u8, input: String) -> Result<WasmSolution, JsError> {
    let day = Day::try_new(day)?;
    let part = Part::new(part);

    // wasm bindgen can't handle enums with values yet
    // see: https://github.com/rustwasm/wasm-bindgen/issues/2407
    // I'd like to return an enum for the solved part that holds a string, but we can't.
    // That's why we return a WasmSolution for everything, even single parts,
    // A WasmSolution has Option<String> fields, None values turn into undefined in JS
    match solve_part(&day, &part, &input)? {
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
