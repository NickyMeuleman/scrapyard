mod utils;

use fixedbitset::FixedBitSet;
use rand::Rng;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Self {
        let width = 64;
        let height = 64;
        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        let mut rng = rand::thread_rng();
        for i in 0..size {
            cells.set(i, rng.gen_bool(0.5));
        }
        Self {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    // returning a raw pointer to the vector's buffer
    pub fn cells(&self) -> *const u32 {
        // convert fixedbitset to slice, then to a pointer
        self.cells.as_slice().as_ptr()
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbour_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for row_offset in [self.height - 1, 0, 1].iter().cloned() {
            for col_offset in [self.width - 1, 0, 1].iter().cloned() {
                if row_offset == 0 && col_offset == 0 {
                    continue;
                }
                let neighbour_row = (row + row_offset) % self.height;
                let neighbour_col = (col + col_offset) % self.width;
                let idx = self.get_index(neighbour_row, neighbour_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbours = self.live_neighbour_count(row, col);
                let next_cell = match (cell, live_neighbours) {
                    (true, x) if x < 2 => false,
                    (true, 2) | (true, 3) => true,
                    (true, x) if x > 3 => false,
                    (false, 3) => true,
                    (cell, _) => cell,
                };
                next.set(idx, next_cell);
            }
        }
        self.cells = next;
    }
}
