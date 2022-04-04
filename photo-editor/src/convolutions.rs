use crate::Photo;
use crate::utils;

/// Convolves the image with the specified filter
/// @param filter [in] the filter to convolve the image with
/// @param [in/out] the photo to apply the convolution to
pub fn apply_convolution(filter: [f32; 9], photo: &mut Photo) {
    let mut filtered = utils::photo_to_imgrgb(photo).filter3x3(&filter);
    photo.pixels = filtered.as_bytes().to_vec();
}

/// Converts the photo to the edge representation according to the gradient Sobel filter
/// @param photo [in/out] the photo to convert
pub fn edge_detection(photo: &mut Photo) {
    apply_convolution([1.0, 2.0, 1.0, 0.0, 0.0, 0.0, -1.0, -2.0, -1.0], photo);
}