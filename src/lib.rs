use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn add(left: i32, right: i32) -> i32;
}

#[wasm_bindgen]
pub fn add_n(left: i32, right: i32) -> i32 {
    return add(left, right)
}
