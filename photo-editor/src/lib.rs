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
use base64::decode;
use image::DynamicImage::ImageRgba8;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// The photo struct which represents the photo to edit
#[wasm_bindgen]
pub struct Photo {
    pixels: Vec<u8>,
    width: u32,
    height: u32,
}

/// Called when a new image is uploaded, this function sets the current image as a pixel array
/// @return a Photo variable pointer to JS
#[wasm_bindgen]
pub fn set_img_bytes(base64: &str) -> Photo {
    utils::set_panic_hook(); // Sets the error message to actually be useful
    let vec64: Vec<u8> = decode(base64).unwrap();
    let img = image::load_from_memory(vec64.as_slice()).unwrap();

    Photo {
        pixels: ImageRgba8(img.to_rgba8()).as_bytes().to_vec(),
        width: img.width(),
        height: img.height()
    }
}

/// Applies the grayscale filter to the image
/// @param photo [in] the photo to convert to grayscale
/// @return the base64 encoded image with grayscale applied
#[wasm_bindgen]
pub fn to_grayscale(photo: &Photo) -> String {
    let mut tmp = Photo {
        pixels: photo.pixels.clone(),
        width: photo.width,
        height: photo.height
    };
    filters::grayscale(&mut tmp);
    utils::photo_to_base64(&tmp)
}