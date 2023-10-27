use aoc_all::{solve_part, Answer, Part, Solution};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
pub struct WasmSolution {
    pub part1: Option<String>,
    pub part2: Option<String>,
}

impl From<Solution> for WasmSolution {
    fn from(value: Solution) -> Self {
        WasmSolution {
            part1: Some(value.part1.to_string()),
            part2: Some(value.part2.to_string()),
        }
    }
}

#[wasm_bindgen]
pub async fn solve(year: u32, day: u8, input: String, part: u8) -> Result<WasmSolution, JsError> {
    let part = match part {
        1 => Part::One,
        2 => Part::Two,
        _ => Part::Both,
    };
    // wasm bindgen can't handle enums with values yet
    // see: https://github.com/rustwasm/wasm-bindgen/issues/2407
    // I'd like to return an enum for the solved part that holds a string, but we can't.
    // That's why we return a WasmSolution for everything, even single parts,
    // A WasmSolution has Option<String> fields, None values turn into undefined in JS
    match solve_part(year, day, &part, &input) {
        Ok(answer) => match answer {
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
        },
        Err(error) => Err(JsError::new(&error)),
    }
}
