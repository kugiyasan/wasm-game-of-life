//! Test suite for the Web and headless browsers.
//! Run the tests with `wasm-pack test --firefox --headless`
//! Don't use `--headless` when running on wsl2

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
extern crate wasm_game_of_life;
use wasm_game_of_life::Universe;

wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
pub fn input_spaceship() -> Universe {
    let mut universe = Universe::new(6, 6);
    universe.set_cells(&[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);
    universe
}

#[cfg(test)]
pub fn expected_spaceship() -> Universe {
    let mut universe = Universe::new(6, 6);
    universe.set_cells(&[(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);
    universe
}

#[wasm_bindgen_test]
pub fn test_tick() {
    // Let's create a smaller Universe with a small spaceship to test!
    let mut input_universe = input_spaceship();

    // This is what our spaceship should look like
    // after one tick in our universe.
    let expected_universe = expected_spaceship();

    // Call `tick` and then see if the cells in the `Universe`s are the same.
    input_universe.tick();
    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}

#[cfg(test)]
pub fn input_pulsar() -> Universe {
    let mut universe = Universe::new(13, 13);

    universe.spawn_pulsar(6, 6);

    universe
}

#[cfg(test)]
pub fn expected_pulsar() -> Universe {
    let mut universe = Universe::new(13, 13);

    universe.set_cells(&[(0, 2), (0, 3), (0, 4), (0, 8), (0, 9), (0, 10)]);
    universe.set_cells(&[(2, 0), (2, 5), (2, 7), (2, 12)]);
    universe.set_cells(&[(3, 0), (3, 5), (3, 7), (3, 12)]);
    universe.set_cells(&[(4, 0), (4, 5), (4, 7), (4, 12)]);
    universe.set_cells(&[(5, 2), (5, 3), (5, 4), (5, 8), (5, 9), (5, 10)]);
    universe.set_cells(&[(7, 2), (7, 3), (7, 4), (7, 8), (7, 9), (7, 10)]);
    universe.set_cells(&[(8, 0), (8, 5), (8, 7), (8, 12)]);
    universe.set_cells(&[(9, 0), (9, 5), (9, 7), (9, 12)]);
    universe.set_cells(&[(10, 0), (10, 5), (10, 7), (10, 12)]);
    universe.set_cells(&[(12, 2), (12, 3), (12, 4), (12, 8), (12, 9), (12, 10)]);

    universe
}

#[wasm_bindgen_test]
pub fn test_pulsar() {
    let input_universe = input_pulsar();
    let expected_universe = expected_pulsar();

    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}
