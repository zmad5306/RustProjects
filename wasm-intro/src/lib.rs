use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {name}, from Rust + WASM!")
}

#[wasm_bindgen]
pub fn crunch_numbers(iterations: u32) -> u32 {
    let mut state = 0x1234_5678u32;

    for i in 0..iterations {
        state ^= i.wrapping_mul(0x45d9_f3bu32);
        state = state.rotate_left(7);
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
    }

    state
}
