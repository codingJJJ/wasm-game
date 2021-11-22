mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct  Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new () -> Universe {
        let w = 400;
        let h = 250;

        let cells = (0..w * h).map(|i| {
            if i % 2 ==0 || i % 11 ==0 {
                Cell::Alive
            }else {
                Cell::Dead
            }
        }).collect();

        Universe {
            width: w,
            height: h,
            cells: cells
        }
    }

    pub fn render (&self) -> String {
        self.to_string()
    }

    // 通过行列获取下标
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }
    // 获取活着的邻居细胞个数
    fn get_alive_neibor_count (&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height-1, 0, 1].iter().cloned() {
            for delta_col in [self.width -1, 0, 1].iter().cloned() {
                if delta_col == 0 && delta_row == 0 {
                    continue;
                }
                let neibor_row = (row + delta_row)%self.height;
                let neibor_col = (col + delta_col)%self.width;
                let idx = self.get_index(neibor_row, neibor_col);
                count += self.cells[idx] as u8
            }
        }
        count
    }
    // 下个时间宇宙的变化
    pub fn tick (&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let alive_neibors = self.get_alive_neibor_count(row, col);
                
                let next_cell = match (cell, alive_neibors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }
        self.cells = next
    }

    // 下一个
}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead {
                    '□'
                } else  {
                    '■'
                };
                write!(f, "{}",  symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}