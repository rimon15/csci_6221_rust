/**
 * This file will contain our bare-bones JS frontend. We will write the HTML directly to the window
 * and have basic methods for our filters, file uploads, etc. Once these functions are called in this file, they will
 * reference the rust wasm-bindgen implementations to actually perform the operations on the images. All we really need
 * here is a basic layout, image display, buttons for filters, and an upload file option.
 */

import * as wasm from "photo_editor";

// The variable holding the rust Photo struct image after uploaded
var photo_ptr = null;
// The ML model for ML functions
var model = null;
// The labels for the model predictions
var labels = [];

/**
 * First, we write the CSS style for our page
 */
document.write(`
<style>
 /* Column layout style */
* {
    box-sizing: border-box;
  }
  
  .first-column {
    float: left;
    width: 20%;
    padding: 10px;
    height: 100%;
  }
  .second-column {
    float: left;
    width: 80%;
    padding: 10px;
    height: 100%; 
  }
  
  /* Clear floats after the columns */
  .row:after {
    content: "";
    display: table;
    clear: both;
  }

  #display_image {
      width: 100%;
      height: 100%;
      object-fit: contain;
      background-repeat-x: no-repeat;
      background-repeat-y: no-repeat;
  }

</style>
`);

/** 
 * Now we just render the HTML of the frontend container for our page
 */
document.write(`
                <html>
                    <body>
                        <h2>Welcome to the Rust-WASM Photo Editor!</h2>
                    </body>
                    <div class="row">
                        <div class="first-column" style="background-color:#aaa;">
                            <h3>General Transformations</h3>
                            <p><button type="button" id="grayscale_btn">Grayscale (Desaturation)</button></p>
                            <p><button type="button" id="monochrome_btn">Monochrome</button></p>
                            <p><button type="button" id="ocean_blue_btn">Ocean Blue</button></p>
                            <p><button type="button" id="purple_btn">Purple</button></p>
                            <p><button type="button" id="edges_btn">Detect Edges</button></p>
                            <p><button type="button" id="sharp_btn">Sharpening Filter</button></p>
                            <p><button type="button" id="thresh_btn">Thresholding</button></p>
                            <p><button type="button" id="emboss_btn">Emboss</button></p>
                            <h3>Machine Learning</h3>
                            Upload Model<input type="file" id="model_input">
                            Upload Labels<input type="file" id="label_input">
                            <p><button type="button" id="recognition_btn">Image Recognition</button></p>
                        </div>
                        <div class="second-column">
                            <input type="file" id="image_input" accept="image/jpeg, image/png">
                            <div id="display_image" />
                        </div>
                    </div>
                </html>
`);

/** 
 * In this section, we add all of our event listeners for the various buttons on the page (janky, but easiest due to the way our setup is...)
 */
document.getElementById("grayscale_btn").addEventListener("click", conv_gray);
document.getElementById("monochrome_btn").addEventListener("click", conv_mono);
document.getElementById("edges_btn").addEventListener("click", detect_edges);
document.getElementById("sharp_btn").addEventListener("click", sharpening);
//document.getElementById("cartoon_btn").addEventListener("click", cartoonize);
document.getElementById("recognition_btn").addEventListener("click", ml_recog);
document.getElementById("ocean_blue_btn").addEventListener("click", conv_ocean_blue);
document.getElementById("purple_btn").addEventListener("click", conv_purple);
document.getElementById("thresh_btn").addEventListener("click", filter_thresh);
document.getElementById("emboss_btn").addEventListener("click", conv_emboss);
/**
 * In this section, we add our interfacing functions which will be called by the buttons on the frontend, and will subsequently call
 * the rust wasm code to execute the commands (and perform any other JS actions such as image rendering, etc...)
 */

function conv_gray() {
    let new_img = wasm.to_grayscale(photo_ptr);
    document.querySelector("#display_image").style.backgroundImage = `url(${new_img})`;
}

function conv_mono() {
    let new_img = wasm.to_monochrome(photo_ptr, 50, 50, 50);
    document.querySelector("#display_image").style.backgroundImage = `url(${new_img})`;
}

function detect_edges() {
    let new_img = wasm.get_edges(photo_ptr);
    document.querySelector("#display_image").style.backgroundImage = `url(${new_img})`;
}

function sharpening() {
    let new_img = wasm.sharpen_image(photo_ptr);
    document.querySelector("#display_image").style.backgroundImage = `url(${new_img})`;
}

function ml_recog() {
    let img_txt = wasm.cnn_recognition(photo_ptr, model);
    img_txt = JSON.parse(img_txt);
    alert('This is a ' + labels[img_txt[1] - 1] + ' with confidence: ' + img_txt[0]);
    document.querySelector('#img_txt').innerHTML = img_txt;
}
function conv_ocean_blue() {
    let new_img = wasm.to_oceanblue(photo_ptr, 50, 50, 50);
    document.querySelector("#display_image").style.backgroundImage = `url(${new_img})`;
}

function conv_purple() {
    let new_img = wasm.to_purple(photo_ptr, 50, 50, 50);
    document.querySelector("#display_image").style.backgroundImage = `url(${new_img})`;
}

function filter_thresh() {
    let new_img = wasm.to_threshold(photo_ptr);
    document.querySelector("#display_image").style.backgroundImage = `url(${new_img})`;
}

function conv_emboss() {
    let new_img = wasm.to_emboss(photo_ptr);
    document.querySelector("#display_image").style.backgroundImage = `url(${new_img})`;
}
/**
 * This section is for the image/model/labels upload
 */
const image_input = document.querySelector("#image_input");

image_input.addEventListener("change", function () {
    const reader = new FileReader();
    reader.addEventListener("load", () => {
        const uploaded_image = reader.result;
        // Get the uploaded image as a pointer in the rust-wasm so we can use it
        if (uploaded_image) {
            const needle = "base64,";
            console.log(uploaded_image);
            var a = uploaded_image.substring(uploaded_image.toString().indexOf(needle) + needle.length);
            photo_ptr = wasm.set_img_bytes(a);
        }
        document.querySelector("#display_image").style.backgroundImage = `url(${uploaded_image})`;
    });
    reader.readAsDataURL(this.files[0]);
});

const model_input = document.querySelector("#model_input");

model_input.addEventListener("change", function () {
    const reader = new FileReader();
    reader.addEventListener("load", () => {
        const uploaded_model = reader.result;
        // Get the uploaded model as a pointer in the rust-wasm so we can use it
        if (uploaded_model) {
            const needle = "data:application/octet-stream;base64,";
            model = uploaded_model.substring(uploaded_model.toString().indexOf(needle) + needle.length);
        }
    });
    reader.readAsDataURL(this.files[0]);
});

const label_input = document.querySelector("#label_input");

label_input.addEventListener("change", function () {
    const reader = new FileReader();
    reader.addEventListener("load", () => {
        const uploaded_label = reader.result;
        // Get the uploaded model as a pointer in the rust-wasm so we can use it
        if (uploaded_label) {
            var labels_str = Buffer.from(uploaded_label, 'base64').toString('ascii');
            labels_str = labels_str.substring(labels_str.indexOf('vZ1n8') + 'vZ1n8'.length);
            labels = labels_str.split('\n');
        }
    });
    reader.readAsDataURL(this.files[0]);
});