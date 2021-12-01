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
    prev_cells: FixedBitSet,
    // hot cells should be checked next tick
    hot_cells: FixedBitSet,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        let mut rng = rand::thread_rng();
        for i in 0..size {
            cells.set(i, rng.gen_bool(0.5));
        }
        let prev_cells = cells.clone();
        // mark every cell as hot
        let mut hot_cells = FixedBitSet::with_capacity(size);
        hot_cells.set_range(.., true);

        Self {
            width,
            height,
            cells,
            prev_cells,
            hot_cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    /// Marks all cells as hot
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        let size = (width * self.height) as usize;
        self.cells.clear();
        self.cells.grow(size);
        self.hot_cells.grow(size);
        self.hot_cells.set_range(.., true);
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    /// Marks all cells as hot
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        let size = (self.width * height) as usize;
        self.cells.clear();
        self.cells.grow(size);
        self.hot_cells.grow(size);
        self.hot_cells.set_range(.., true);
    }

    // returning a raw pointer to the vector's buffer
    pub fn cells(&self) -> *const u32 {
        // convert fixedbitset to slice, then to a pointer
        self.cells.as_slice().as_ptr()
    }

    fn coords_to_idx(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn idx_to_coords(&self, idx: usize) -> (u32, u32) {
        (idx as u32 / self.width, idx as u32 % self.width)
    }

    fn neighbours(&self, row: u32, column: u32) -> impl Iterator<Item = usize> {
        let mut indexes = Vec::with_capacity(8);
        for row_offset in [self.height - 1, 0, 1].iter().cloned() {
            for col_offset in [self.width - 1, 0, 1].iter().cloned() {
                if row_offset == 0 && col_offset == 0 {
                    continue;
                }
                let neighbor_row = (row + row_offset) % self.height;
                let neighbor_col = (column + col_offset) % self.width;
                let idx = self.coords_to_idx(neighbor_row, neighbor_col);
                indexes.push(idx);
            }
        }
        indexes.into_iter()
    }

    pub fn tick(&mut self) {
        let cloned_hot_ones = self.hot_cells.clone();
        self.hot_cells.clear();

        // using prev_cells as starting point for calculation
        // all outdated cells will be overwritten during this loop
        for idx in cloned_hot_ones.ones() {
            let cell = self.cells[idx];
            let (row, col) = self.idx_to_coords(idx);
            let live_neighbours = self
                .neighbours(row, col)
                .map(|idx| self.cells[idx])
                .filter(|&bit| bit)
                .count();
            let next_cell = match (cell, live_neighbours) {
                (true, x) if x < 2 => false,
                (true, 2) | (true, 3) => true,
                (true, x) if x > 3 => false,
                (false, 3) => true,
                (cell, _) => cell,
            };
            self.prev_cells.set(idx, next_cell);
            if cell != next_cell {
                // set self as hot
                self.hot_cells.set(idx, true);
                // set neighbours as hot
                for neighbour_idx in self.neighbours(row, col) {
                    self.hot_cells.set(neighbour_idx, true);
                }
            }
        }

        std::mem::swap(&mut self.cells, &mut self.prev_cells);
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.coords_to_idx(row, col);
        let curr_bit = self.cells[idx];
        self.cells.set(idx, !curr_bit);
        // mark curr idx and neigbours as hot
        self.hot_cells.set(idx, true);
        let neighbours = self.neighbours(row, col);
        for neighbour_idx in neighbours {
            self.hot_cells.set(neighbour_idx, true);
        }
    }
}

impl Universe {
    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.coords_to_idx(row, col);
            self.cells.set(idx, true);
        }
    }
}
