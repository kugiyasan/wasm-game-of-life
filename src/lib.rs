mod utils;

extern crate fixedbitset;

use fixedbitset::FixedBitSet;
use js_sys::Math;
use wasm_bindgen::prelude::*;

/// A Universe is a game of life map
///
/// The borders of the map wraps around
///
/// A cell is considered alive if its bit is set to true in FixedBitSet
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

impl Universe {
    /// Get the dead and alive values of the entire universe
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

#[wasm_bindgen]
impl Universe {
    /// When creating an Universe, this function must be called
    /// to enable the `console_error_panic_hook` feature
    fn _new<F: Fn(usize) -> bool>(f: F) -> Self {
        // Enable the `console_error_panic_hook` feature
        utils::set_panic_hook();

        let width = 64;
        let height = 64;

        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, f(i));
        }

        Self {
            width,
            height,
            cells,
        }
    }

    /// Create a new Universe where every multiple of 2 or 7 is alive
    pub fn new() -> Self {
        Self::_new(|i| i % 2 == 0 || i % 7 == 0)
    }

    /// Create a new Universe with a glider in the top left corner
    pub fn new_with_glider() -> Self {
        let mut universe = Self::_new(|_| false);

        let glider: [(u32, u32); 5] = [(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)];
        for &(row, col) in glider.iter() {
            let i = universe.get_index(row, col);
            universe.cells.set(i, true);
        }

        universe
    }

    /// Create a new Universe and makes half the cells alive randomly
    pub fn new_random() -> Self {
        Self::_new(|_| Math::random() > 0.5)
    }
}

#[wasm_bindgen]
impl Universe {
    /// Get the width of the Universe
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get the height of the Universe
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Set the width of the universe
    ///
    /// Resets all cells to the dead state
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = FixedBitSet::with_capacity((self.width * self.height) as usize);
    }

    /// Set the height of the universe
    ///
    /// Resets all cells to the dead state
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = FixedBitSet::with_capacity((self.width * self.height) as usize);
    }

    /// Get a raw pointer to the cells
    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn toggle_cells(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells.toggle(idx);
    }

    /// Updates the game of life map by one iteration
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                log!(
                    "cell[{}, {}] is initially {:?} and has {} live neighbors",
                    row,
                    col,
                    cell,
                    live_neighbors,
                );

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (true, x) if x < 2 => false,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (true, 2) | (true, 3) => true,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (true, x) if x > 3 => false,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (false, 3) => true,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                log!("    it becomes {:?}", next_cell);

                if next[idx] != next_cell {
                    log!(
                        "    The state of the cell changed from {} to {}",
                        next[idx],
                        next_cell
                    );
                }

                next.set(idx, next_cell);
            }
        }

        self.cells = next;
    }
}
