mod utils;
use js_sys;
use wasm_bindgen::prelude::*;

extern crate web_sys;

// `console.log` 로깅을 위한 `println!(..)` 스타일 구문을 제공
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }

    pub fn cells(&self) -> *const Cell {
        // 슬라이스 버퍼에 있는 포인터 정보를 리턴한다.
        self.cells.as_ptr()
    }

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

    // 자바스크립트로 내보낸 퍼블릭 메소드
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: 살아있는 세포 주변에 살아있는 세포가 2개 미만이라면 마치 인구 부족처럼 죽는다.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: 살아있는 세포 주변에 살아있는 세포가 2~3개라면 다음 세대까지 산다.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: 살아있는 세포 주변에 살아있는 세포가 3개 초과라면 마치 인구 과잉처럼 죽는다.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: 죽은 세포 주변에 살아있는 세포가 3개라면 마치 번식처럼 산다.
                    (Cell::Dead, 3) => Cell::Alive,
                    // 이외 다른 모든 세포는 동일하게 유지된다.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn new() -> Universe {
        utils::set_panic_hook();
        // panic!("패닉 테스트");

        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|_| {
                if js_sys::Math::random() < 0.5 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        log!("create universe!");

        Universe {
            width,
            height,
            cells,
        }
    }
}

// #[wasm_bindgen] 속성이 없는, js에 노출하고 싶지 않은, 테스트에만 필요한 함수들
impl Universe {
    // 러스트가 생성하는 WebAssembly 함수는 빌린 참조를 반환할 수 없다.
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    // 각 셀의 row, column을 배열로 전달하여 셀이 살아있도록 설정
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }
}
