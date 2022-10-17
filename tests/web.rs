//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate game_of_life;
use game_of_life::Universe;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

#[cfg(test)]
pub fn input_spaceship() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells(&[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);
    universe
}

#[cfg(test)]
pub fn expected_spaceship() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells(&[(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);
    universe
}

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_tick() {
    let mut input_universe = input_spaceship();

    // 앞서 만든 우주의 한 틱 후 예상 모습
    let expected_universe = expected_spaceship();

    // `tick` 호출 후 `Universe`의 셀이 동일한지 확인
    input_universe.tick();
    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}
