mod utils;
mod demo_scene;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, simulator!");
}

#[wasm_bindgen]
pub fn run_models_demo_app() {
    utils::set_panic_hook();
    demo_scene::run_models_demo_app();
}