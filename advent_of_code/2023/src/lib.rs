pub mod day_01;
pub mod utils;

use utils::{run, Solution};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn solve(day: u8, input: String) -> Result<Solution, JsError> {
    run(day, input)
}
