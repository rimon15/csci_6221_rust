use std::cmp;
use crate::Photo;
use web_sys::console;

/// Converts the photo to grayscale using the Desaturation algorithm
/// @param photo [in/out] the photo to convert
pub fn grayscale(photo: &mut Photo) {
    for i in (0..(photo.pixels.len() - 4)).step_by(4) {
        let r = photo.pixels[i];
        let g = photo.pixels[i + 1];
        let b = photo.pixels[i + 2];

        let gray = (cmp::min(cmp::min(r, g), b) + cmp::max(cmp::max(r, g), b)) / 2;

        photo.pixels[i] = gray;
        photo.pixels[i + 1] = gray;
        photo.pixels[i + 2] = gray;
    }
}