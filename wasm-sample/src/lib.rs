use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test(x: i32) -> i32 {
    if x <= 0 {
        x
    } else {
        test(x - 1) + x
    }
}

#[wasm_bindgen]
pub fn square(x: i32) -> i32 {
    x * x
}
