use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::console::log_1;


#[wasm_bindgen(start)]
pub fn main() {
    let nums = vec![1,2,3,4,5];
    for num in nums {
        log_1(&JsValue::from(num.to_string()));
    }
}
