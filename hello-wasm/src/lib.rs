use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
    let result:f32 = 0.0/0.0;
    alert(&format!("{:?}", result.to_be_bytes()))
}
