mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        // 이전 row, col 부터 자신 뒤의 row, col까지 만약 4, 4라면 [3, 0, 1]이 된다.
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                // 자기 자신은 건너뛴다.
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                // 0  1  2  3
                // 0 [0, 0, 0, 0,
                // 1  0, 0, 0, 0,
                // 2  0, 0, 0, 0,
                // 3  0, x, 0, 0]
                //
                // (row: 3, column: 1) 이라면
                // delta_row = 3
                // (3 + 3) % 4 = 2
                // (1 + 3) % 4 = 0
                //
                // (3 + 3) % 4 = 2
                // (1 + 0) % 4 = 1
                //
                // (3 + 3) % 4 = 2
                // (1 + 1) % 4 = 2
                //
                // delta_row = 0
                // (3 + 0) % 4 = 3
                // (1 + 3) % 4 = 0
                //
                // ...
                //
                // delta_row = 1
                // (3 + 1) % 4 = 0
                // (1 + 3) % 4 = 0
                //
                // (3 + 1) % 4 = 0
                // (1 + 0) % 4 = 1
                //
                // (3 + 1) % 4 = 0
                // (1 + 1) % 4 = 2
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}
