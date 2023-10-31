use aoc_core::Solution;
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
