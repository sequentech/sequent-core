use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen]
    fn postMessage(s: &str);
}

#[wasm_bindgen]
pub fn message(s: &str) {
    log(s);
    postMessage(s);
}

#[wasm_bindgen]
pub fn test() {
    message("--- wasm::test.rs")
}
