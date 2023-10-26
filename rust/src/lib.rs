mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn get_greeting() -> String {
    return String::from("Hello SvelteKit from Rust!!!!!! :)");
}
