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
pub fn crunch_numbers(iterations: u32) -> String {
    let mut state = 0x1234_5678_9abc_def0u64;

    for i in 0..iterations {
        let step = u64::from(i).wrapping_mul(0x9e37_79b9_7f4a_7c15u64);
        state ^= step;
        state = state.rotate_left(27);
        state = state
            .wrapping_mul(0xbf58_476d_1ce4_e5b9u64)
            .wrapping_add(0x94d0_49bb_1331_11ebu64);
        state ^= state >> 31;
    }

    state.to_string()
}
