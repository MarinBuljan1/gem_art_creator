use wasm_bindgen::prelude::*;

pub mod dmc_colors;
pub mod models;
pub mod utils;
pub mod image_processing;
pub mod components;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<components::App>::new().render();
}