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
mod convolutions;
mod machine_learning;

use wasm_bindgen::prelude::*;
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

/// Applies the monochrome filter to the image
/// @param photo [in] the photo to convert to monochrome
/// @return the base64 encoded image with monochrome applied
#[wasm_bindgen]
pub fn to_monochrome(photo: &Photo, r_offset: u32, g_offset: u32, b_offset: u32) -> String {
    let mut tmp = Photo {
        pixels: photo.pixels.clone(),
        width: photo.width,
        height: photo.height
    };
    filters::monochrome(&mut tmp, r_offset, g_offset, b_offset);
    utils::photo_to_base64(&tmp)
}

/// Applies the sobel filter to the image
/// @param photo [in] the photo to get edges for
/// @return the base64 encoded image with edges
#[wasm_bindgen]
pub fn get_edges(photo: &Photo) -> String {
    let mut tmp = Photo {
        pixels: photo.pixels.clone(),
        width: photo.width,
        height: photo.height
    };
    convolutions::edge_detection(&mut tmp);
    utils::photo_to_base64(&tmp)
}

/// Applies the laplacian to the image for sharpening
/// @param photo [in] the photo to sharpen
/// @return the base64 encoded sharpened image
#[wasm_bindgen]
pub fn sharpen_image(photo: &Photo) -> String {
    let mut tmp = Photo {
        pixels: photo.pixels.clone(),
        width: photo.width,
        height: photo.height
    };
    convolutions::sharpen(&mut tmp);
    utils::photo_to_base64(&tmp)
}

/// Applies a Generative Adversarial Network (GAN) with a pretrained model to cartoonize the image
/// @param photo [in] the photo to cartoonize
/// @return the base64 encoded cartoon image
#[wasm_bindgen]
pub fn cartoon_gan(photo: &Photo) -> String {
    let tmp = Photo {
        pixels: photo.pixels.clone(),
        width: photo.width,
        height: photo.height
    };
    //machine_learning::sharpen(&mut tmp);
    utils::photo_to_base64(&tmp)
}

/// Applies a Convolutional Neural Network (CNN) with a pretrained model to recognize the image
/// @param photo [in] the photo to cartoonize
/// @return the base64 encoded cartoon image
#[wasm_bindgen]
pub fn cnn_recognition(photo: &Photo, base64: &str) -> String {
    let model_dat: Vec<u8> = decode(base64).unwrap();
    machine_learning::recognition(&photo, &model_dat)
}


/// Applies the oceanBlue filter to the image
/// @param photo [in] the photo to convert to ocean blue
/// @return the base64 encoded image with ocean blue applied
#[wasm_bindgen]
pub fn to_oceanblue(photo: &Photo) -> String {
    let mut tmp = Photo {
        pixels: photo.pixels.clone(),
        width: photo.width,
        height: photo.height
    };
    filters::ocean_blue(&mut tmp);
    utils::photo_to_base64(&tmp)
}

/// Applies the Purple filter to the image
/// @param photo [in] the photo to convert to purple
/// @return the base64 encoded image with purple applied
#[wasm_bindgen]
pub fn to_purple(photo: &Photo) -> String {
    let mut tmp = Photo {
        pixels: photo.pixels.clone(),
        width: photo.width,
        height: photo.height
    };
    filters::purple(&mut tmp);
    utils::photo_to_base64(&tmp)
}

/// Applies thresholding to the image
/// @param photo [in] the photo to convert to threshold
/// @return the base64 encoded image with thresholding applied
#[wasm_bindgen]
pub fn to_threshold(photo: &Photo) -> String {
    let mut tmp = Photo {
        pixels: photo.pixels.clone(),
        width: photo.width,
        height: photo.height
    };
    filters::bw_threshold(&mut tmp);
    utils::photo_to_base64(&tmp)
}

/// Applies embossing filter
/// @param photo [in] the photo to convert to emboss
/// @return the base64 encoded image with embossing applied
#[wasm_bindgen]
pub fn to_emboss(photo: &Photo) -> String {
    let mut tmp = Photo {
        pixels: photo.pixels.clone(),
        width: photo.width,
        height: photo.height
    };
    convolutions::emboss(&mut tmp);
    utils::photo_to_base64(&tmp)
}