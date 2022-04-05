use tract_tensorflow::prelude::*;
use crate::utils;
use crate::Photo;
use std::io::Cursor;
use web_sys::console;

/*/// Converts the photo to the cartoonized representation
/// @param photo [in/out] the photo to convert
pub fn cartoon(photo: &mut Photo) {
    let tf = tensorflow();
    let mut model = tf.model_for_path("../models/cartoon_gan.pb").unwrap();

    model.set_input_fact(0, InferenceFact::dt_shape(f32::datum_type(), tvec!(1, photo.height, photo.width, 3))).unwrap();



}*/


/// Uses MobileNet CNN for image recognition on the photo
/// @param photo [in] the photo to label
/// @param model_dat [in] the model data (since opening files is currently unsupported by rust-wasm)
pub fn recognition(photo: & Photo, model_dat: &[u8]) -> String {
    let mut model_dat_mut = Cursor::new(model_dat);
    let mut model = tract_tensorflow::tensorflow().model_for_read(&mut model_dat_mut).unwrap();

    console::log_1(&photo.height.to_string().into());
    // set the shape and type of input
    model.set_input_fact(0, InferenceFact::dt_shape(f32::datum_type(), 
        tvec!(1, 224, 224, 3))).unwrap();

    // get the model execution plan
    let model = model.into_optimized().unwrap();
    let plan = SimplePlan::new(&model).unwrap();

    // resize image and transform into tensor
    let image = utils::photo_to_imgrgb(photo);
    let resized = image::imageops::resize(
        &image, 224, 224, ::image::imageops::FilterType::Triangle);
    
    let image: Tensor = tract_ndarray::Array4::from_shape_fn((1, 224, 224, 3), |(_, y, x, c)| {
        resized[(x as _, y as _)][c] as f32 / 255.0
    }).into();

    // get the result from the model
    let result = plan.run(tvec!(image)).unwrap();

    let best = result[0]
        .to_array_view::<f32>().unwrap()
        .iter()
        .cloned()
        .zip(1..)
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    serde_json::to_string(&best.unwrap()).unwrap()
}