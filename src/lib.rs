mod utils;
use js_sys;
use wasm_bindgen::prelude::*;
use web_sys::console;

extern crate web_sys;

// `console.log` 로깅을 위한 `println!(..)` 스타일 구문을 제공
macro_rules! log {
    ( $( $t:tt )* ) => {
        console::log_1(&format!( $( $t )* ).into());
    }
}

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        }
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    back_cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    fn get_random_cells(width: u32, height: u32) -> Vec<Cell> {
        (0..width * height)
            .map(|_| {
                if js_sys::Math::random() < 0.5 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        // 슬라이스 버퍼에 있는 포인터 정보를 리턴한다.
        self.cells.as_ptr()
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize % self.cells.len()
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 { self.height - 1 } else { row - 1 };

        let south = if row == self.height - 1 { 0 } else { row + 1 };

        let west = if row == 0 { self.width - 1 } else { column - 1 };

        let east = if row == self.width - 1 { 0 } else { column + 1 };

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

    // 자바스크립트로 내보낸 퍼블릭 메소드
    pub fn tick(&mut self) {
        let _timer = Timer::new("Universe::tick");

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

                self.back_cells[idx] = next_cell;
            }
        }

        std::mem::swap(&mut self.cells, &mut self.back_cells);
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }

    pub fn glider(&mut self, row: u32, column: u32) {
        vec![
            (self.get_index(row - 1, column - 1), Cell::Dead),
            (self.get_index(row - 1, column), Cell::Alive),
            (self.get_index(row - 1, column + 1), Cell::Dead),
            (self.get_index(row, column - 1), Cell::Dead),
            (self.get_index(row, column), Cell::Dead),
            (self.get_index(row, column + 1), Cell::Alive),
            (self.get_index(row + 1, column - 1), Cell::Alive),
            (self.get_index(row + 1, column), Cell::Alive),
            (self.get_index(row + 1, column + 1), Cell::Alive),
        ]
        .iter()
        .for_each(|(idx, status)| {
            self.cells[*idx] = *status;
        });
    }

    pub fn pulsar(&mut self, row: u32, column: u32) {
        for current_row in [row - 6, row - 1, row + 1, row + 6] {
            for idx in [
                self.get_index(current_row, column - 4),
                self.get_index(current_row, column - 3),
                self.get_index(current_row, column - 2),
                self.get_index(current_row, column + 2),
                self.get_index(current_row, column + 3),
                self.get_index(current_row, column + 4),
            ] {
                self.cells[idx] = Cell::Alive;
            }

            for idx in [
                self.get_index(current_row, column - 6),
                self.get_index(current_row, column - 5),
                self.get_index(current_row, column - 1),
                self.get_index(current_row, column),
                self.get_index(current_row, column + 1),
                self.get_index(current_row, column + 5),
                self.get_index(current_row, column + 6),
            ] {
                self.cells[idx] = Cell::Dead;
            }
        }

        for current_row in [row - 5, row, row + 5] {
            for idx in [
                self.get_index(current_row, column - 6),
                self.get_index(current_row, column - 5),
                self.get_index(current_row, column - 4),
                self.get_index(current_row, column - 3),
                self.get_index(current_row, column - 2),
                self.get_index(current_row, column - 1),
                self.get_index(current_row, column),
                self.get_index(current_row, column + 1),
                self.get_index(current_row, column + 2),
                self.get_index(current_row, column + 3),
                self.get_index(current_row, column + 4),
                self.get_index(current_row, column + 5),
                self.get_index(current_row, column + 6),
            ] {
                self.cells[idx] = Cell::Dead;
            }
        }

        for current_row in [row - 4, row - 3, row - 2, row + 2, row + 3, row + 4] {
            for idx in [
                self.get_index(current_row, column - 6),
                self.get_index(current_row, column - 1),
                self.get_index(current_row, column + 1),
                self.get_index(current_row, column + 6),
            ] {
                self.cells[idx] = Cell::Alive;
            }

            for idx in [
                self.get_index(current_row, column - 5),
                self.get_index(current_row, column - 4),
                self.get_index(current_row, column - 3),
                self.get_index(current_row, column - 2),
                self.get_index(current_row, column),
                self.get_index(current_row, column + 2),
                self.get_index(current_row, column + 3),
                self.get_index(current_row, column + 4),
                self.get_index(current_row, column + 5),
            ] {
                self.cells[idx] = Cell::Dead;
            }
        }
    }

    pub fn new() -> Universe {
        utils::set_panic_hook();
        // panic!("패닉 테스트");

        let width = 64;
        let height = 64;

        let cells = Universe::get_random_cells(width, height);
        let back_cells = cells.clone();

        log!("create universe!");

        Universe {
            width,
            height,
            cells,
            back_cells,
        }
    }

    pub fn random(&mut self) {
        self.cells = Universe::get_random_cells(self.width, self.height);
    }

    pub fn clear(&mut self) {
        self.cells = vec![Cell::Dead; (self.width * self.height) as usize];
    }
}

// #[wasm_bindgen] 속성이 없는, js에 노출하고 싶지 않은, 테스트에만 필요한 함수들
impl Universe {
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }

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
