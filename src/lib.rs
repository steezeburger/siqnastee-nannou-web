use async_std::task::block_on;
use wasm_bindgen::prelude::wasm_bindgen;

use sketch::{Model, run_app};

mod sketch;
// struct BrowserWindow {
//     /// Width of the browser window
//     width: u64,
//
//     /// Height of the browser window
//     height: u64
// }

// web app entry_point
#[wasm_bindgen]
pub async fn main_web() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let model = Model {};
    block_on(async {
        run_app(model).await;
    });
}

#[wasm_bindgen]
pub async fn print_wh(width: u32, height: u32) {
    web_sys::console::log_1(&format!("w: {}, h: {}", width, height).into());
}