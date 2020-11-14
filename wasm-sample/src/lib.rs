use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[allow(unused_unsafe)]
#[wasm_bindgen]
pub fn greet(name: &str) {
    unsafe {
        log(&format!("Hello, {}!", name));
    }
}

#[wasm_bindgen]
pub fn square(x: i32) -> i32 {
    x * x
}
