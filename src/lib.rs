use async_std::task::block_on;
use wasm_bindgen::prelude::wasm_bindgen;

use sketch::{Model, run_app};

mod sketch;

// web app entry_point
#[wasm_bindgen]
pub async fn main_web(width: f32, height: f32) {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    block_on(async {
        run_app().await;
    });
}
