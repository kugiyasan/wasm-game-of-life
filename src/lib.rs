mod utils;

extern crate fixedbitset;

use fixedbitset::FixedBitSet;
use js_sys::Math;
use utils::Timer;
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

    /// Returns the index of a cell at a certain row and column
    ///
    /// If the index is out of range, the result will be wrapped
    fn get_index(&self, row: u32, column: u32) -> usize {
        ((row * self.width + column) % (self.width * self.height)) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 { self.height - 1 } else { row - 1 };
        let south = if row == self.height - 1 { 0 } else { row + 1 };
        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };
        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        let n = self.get_index(north, column);
        count += self.cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells[w] as u8;

        let e = self.get_index(row, east);
        count += self.cells[e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;

        let s = self.get_index(south, column);
        count += self.cells[s] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;

        count
    }
}

#[wasm_bindgen]
impl Universe {
    /// When creating an Universe, this function must be called
    /// to enable the `console_error_panic_hook` feature
    fn _new<F: Fn(usize) -> bool>(width: u32, height: u32, f: F) -> Self {
        // Enable the `console_error_panic_hook` feature
        utils::set_panic_hook();

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
    pub fn new(width: u32, height: u32) -> Self {
        Self::_new(width, height, |i| i % 2 == 0 || i % 7 == 0)
    }

    /// Create a new Universe with a glider in the top left corner
    pub fn new_with_glider(width: u32, height: u32) -> Self {
        let mut universe = Self::_new(width, height, |_| false);

        Self::spawn_glider(&mut universe, 1, 1);

        universe
    }

    /// Create a new Universe and makes half the cells alive randomly
    pub fn new_random(width: u32, height: u32) -> Self {
        Self::_new(width, height, |_| Math::random() > 0.5)
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

    /// Toggle a single cell from dead to alive or vice-versa
    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells.toggle(idx);
    }

    /// Spawn a glider at location (row, col)
    ///
    /// The glider is centered on the coordinates
    ///
    /// Below is a representation of a glider
    ///
    /// ◻◼◻\
    /// ◻◻◼\
    /// ◼◼◼
    pub fn spawn_glider(&mut self, row: u32, col: u32) {
        let glider: [(u32, u32); 5] = [(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)];

        for &(delta_row, delta_col) in glider.iter() {
            // Adding self.width - 1 and modulo self.width
            // is roughly the same as -1 but without underflowing
            let row = (row + delta_row + self.width - 1) % self.width;
            let col = (col + delta_col + self.height - 1) % self.height;

            let i = self.get_index(row, col);
            self.cells.set(i, true);
        }
    }

    /// Spawn a pulsar at location (row, col)
    ///
    /// The pulsar is centered on the coordinates
    ///
    /// Below is a representation of a pulsar (13x13)
    ///
    /// ◻◻◼◼◼◻◻◻◼◼◼◻◻\
    /// ◻◻◻◻◻◻◻◻◻◻◻◻◻\
    /// ◼◻◻◻◻◼◻◼◻◻◻◻◼\
    /// ◼◻◻◻◻◼◻◼◻◻◻◻◼\
    /// ◼◻◻◻◻◼◻◼◻◻◻◻◼\
    /// ◻◻◼◼◼◻◻◻◼◼◼◻◻\
    /// ◻◻◻◻◻◻◻◻◻◻◻◻◻\
    /// ◻◻◼◼◼◻◻◻◼◼◼◻◻\
    /// ◼◻◻◻◻◼◻◼◻◻◻◻◼\
    /// ◼◻◻◻◻◼◻◼◻◻◻◻◼\
    /// ◼◻◻◻◻◼◻◼◻◻◻◻◼\
    /// ◻◻◻◻◻◻◻◻◻◻◻◻◻\
    /// ◻◻◼◼◼◻◻◻◼◼◼◻◻
    pub fn spawn_pulsar(&mut self, row: u32, col: u32) {
        let two_lines = 0b0011100011100;
        let four_dots = 0b1000010100001;
        let dead_line = 0;
        let pulsar: [u32; 13] = [
            two_lines, dead_line, four_dots, four_dots, four_dots, two_lines, dead_line, two_lines,
            four_dots, four_dots, four_dots, dead_line, two_lines,
        ];

        for (delta_col, line) in pulsar.iter().enumerate() {
            for delta_row in 0..13 {
                // Adding self.width - 1 and modulo self.width
                // is roughly the same as -1 but without underflowing
                let row = (row + delta_row + self.width - 6) % self.width;
                let col = (col + delta_col as u32 + self.height - 6) % self.height;

                let bit = ((line >> delta_row) & 1) == 1;

                let i = self.get_index(row, col);
                self.cells.set(i, bit);
            }
        }
    }

    /// Randomize all the cells
    pub fn randomize(&mut self) {
        for i in 0..(self.width * self.height) as usize {
            self.cells.set(i, Math::random() > 0.5);
        }
    }

    /// Reset all the cells to dead
    pub fn reset(&mut self) {
        self.cells.set_range(.., false);
    }

    /// Update the game of life map by multiples iterations
    pub fn ticks(&mut self, ticks_per_animation_frame: u32) {
        for _ in 0..ticks_per_animation_frame {
            self.tick();
        }
    }

    /// Update the game of life map by one iteration
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

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

                next.set(idx, next_cell);
            }
        }

        self.cells = next;
    }
}
