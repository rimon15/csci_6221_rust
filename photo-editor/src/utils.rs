use base64::encode;
use image::ImageBuffer;
use image::DynamicImage::ImageRgba8;
use image::DynamicImage;
use crate::Photo;
use std::io::Cursor;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Converts a Photo to ImageRgba8
/// @param photo [in] the photo to convert
/// @return the ImageRgba8 representation of the photo
pub fn photo_to_imgrgb(photo: &Photo) -> DynamicImage {
    ImageRgba8(ImageBuffer::from_vec(photo.width, photo.height, photo.pixels.clone()).unwrap())
}

/// Converts a Photo to base64 for display in JS
/// @param photo [in] the photo to convert
/// @return string containing the base64 representation of the photo
pub fn photo_to_base64(photo: &Photo) -> String {
    let img = photo_to_imgrgb(photo);
    let mut buf = Cursor::new(Vec::new());

    img.write_to(&mut buf, image::ImageOutputFormat::Png).unwrap();
    let str = encode(&buf.get_ref());

    format!("data:image/png;base64,{}", str.replace("\r\n", ""))
}