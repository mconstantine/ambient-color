use core_logic::generate_color;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_color_web() -> Result<JsValue, JsValue> {
    let payload = generate_color();

    Ok(serde_wasm_bindgen::to_value(&payload)?)
}
