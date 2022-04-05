use std::cmp;
use crate::Photo;

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

pub fn monochrome(photo: &mut Photo, r_offset: u32, g_offset: u32, b_offset: u32) {
    for i in (0..(photo.pixels.len() - 4)).step_by(4) {
        let r = photo.pixels[i];
        let g = photo.pixels[i + 1];
        let b = photo.pixels[i + 2];
        
        let mut avg: u32 = ((r + g + b) / 3).into();
        if avg >= 255 {
            avg = 255
        }
        let new_r = if avg as u32 + r_offset < 255 {
            avg as u8 + r_offset as u8
        } else {
            255
        };

        let new_g = if avg as u32 + g_offset < 255 {
            avg as u8 + g_offset as u8
        } else {
            255
        };

        let new_b = if avg as u32 + b_offset < 255 {
            avg as u8 + b_offset as u8
        } else {
            255
        };

        photo.pixels[i] = new_r;
        photo.pixels[i + 1] = new_g;
        photo.pixels[i + 1] = new_b;
    }
}