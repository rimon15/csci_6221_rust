/*
 * This file will contain all of our function interactions with the JS code
 *
 * i.e. this means that say, for example, if we have an 'Upload Image' button in index.js,
 * we will upload the image to a byte stream via JS, then call wasm.load_image or something of the like,
 * and load_image will be defined here, where we will load the image into rust and manipulate it here,
 * then send it back to index.js
 *
 */

mod utils;
mod filters;

use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Define our functions which will be visible to the JS code here using the wasm_bindgen attribute
#[wasm_bindgen]
pub fn console_log() -> u8 {
    console::log_1(&"Hello".into());
    return 10;
}