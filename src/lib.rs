mod utils;

use wasm_bindgen::prelude::*;
use js_sys;
extern crate fixedbitset;
use fixedbitset::FixedBitSet;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


/*
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}*/


extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct Universe{
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

#[wasm_bindgen]
impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        return (row * self.width + column) as usize;
    }
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count: u8 = 0;
        // [left, center, right] when iterating over the 8 neighboring cells
        // left is calculated by adding self.width - 1 or self.height - 1 then taking the mod
        for delt_row in [self.height - 1, 0, 1].iter().cloned() {
            for delt_col in [self.width -1 , 0, 1].iter().cloned() {
                if delt_row == 0 && delt_col == 0 {
                    continue;
                }
                let neighbor_row = (row + delt_row) % self.height;
                let neighbor_col = (column + delt_col) % self.width; 

                let index = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[index] as u8;
            }
        }
        return count;
    }

    pub fn tick(&mut self) {
        let mut next_state = FixedBitSet::with_capacity((self.width * self.height) as usize);
        for row in 0..self.height {
            for col in 0..self.width {

                let curr_index = self.get_index(row, col);
                let curr_cell = self.cells[curr_index]; 
                
                let alive_neighbors = self.live_neighbor_count(row, col);
                /*log!(
                    "cell[{}, {}] is initially {:?} and has {} live neighbors",
                    row,
                    col,
                    curr_cell ,
                    alive_neighbors
                );*/

                // true == Cell::Alive
                // false == Cell::Dead
                next_state.set(curr_index, match (curr_cell, alive_neighbors) {
                    (true, x) if x < 2 => false,
                    (true, 2) | (true, 3) => true,
                    (true, x) if x > 3 => false,
                    (false, 3) => true,
                    (otherwise, _) => otherwise
                });
                //log!("    it becomes {:?}", next_state[curr_index]);
                //next_state[curr_index] = next_cell;
                
            }
        }
        self.cells = next_state;
    }
    pub fn new() -> Universe {
        utils::set_panic_hook();
        let width: u32 = 64;
        let height: u32 = 64;

        let size: usize = (width * height) as usize; 
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, i % 2 == 0 || i % 7 == 0);
        }

        Universe {
            width,
            height,
            cells,
        }
    }
    pub fn new_random() -> Universe {
        utils::set_panic_hook();
        let width: u32 = 64;
        let height: u32 = 64;

        let size: usize = (width * height) as usize; 
        let mut cells = FixedBitSet::with_capacity(size);

        /* original implentation using Vec<Cell>
        let cells = (0..width * height)
            .map(|i| {
                if js_sys::Math::random() < 0.5 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();*/
        for i in 0..size {
            cells.set(i, unsafe{js_sys::Math::random()} < 0.5);
        }
    
        

        Universe {
            width,
            height,
            cells,
        }
    }
    pub fn render(&self) -> String {
        self.to_string()
    }
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        return self.cells.as_slice().as_ptr();
    }
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = FixedBitSet::with_capacity((self.height * width) as usize); 
    }
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = FixedBitSet::with_capacity((height * self.width) as usize); 
    }
    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells.toggle(idx);

    }

    pub fn reset(&mut self) {
        self.cells = FixedBitSet::with_capacity((self.height * self.width) as usize);
    }
    pub fn reinitialize(&mut self) {
        for i in 0..self.height * self.width() {
            self.cells.set(i as usize, unsafe{js_sys::Math::random()} < 0.5);
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
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }

}

use std::fmt;
impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == 0 { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
