#![feature(test)]
extern crate test;
extern crate wasm_game_of_life;

// We also have to comment out all the #[wasm_bindgen] annotations, and the "cdylib" bits from Cargo.toml or else building native code will fail and have link errors.

#[bench]
fn universe_ticks(b: &mut test::Bencher) {
    let mut universe = wasm_game_of_life::Universe::new(64, 64);

    b.iter(|| {
        universe.tick();
    });
}