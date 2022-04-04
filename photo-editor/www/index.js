/**
 * This file will contain our bare-bones JS frontend. We will write the HTML directly to the window
 * and have basic methods for our filters, file uploads, etc. Once these functions are called in this file, they will
 * reference the rust wasm-bindgen implementations to actually perform the operations on the images. All we really need
 * here is a basic layout, image display, buttons for filters, and an upload file option.
 */

import * as wasm from "photo_editor";

// The variable holding the rust Photo struct image after uploaded
var photo_ptr = null;

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
      max-width: inherit;
      max-height: inherit;
      background-position: center;
      background-size: 80% 80%;
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
                    <div id="display_image" />
                    <div class="row">
                        <div class="first-column" style="background-color:#aaa;">
                            <h2>Image Functions</h2>
                            <p><button type="button" id="grayscale_btn">Grayscale (Desaturation)</button></p>
                        </div>
                        <div class="second-column">
                            <input type="file" id="image_input" accept="image/jpg, image/png">
                            <div id="display_image">
                        </div>
                    </div>
                </html>
`);

/** 
 * In this section, we add all of our event listeners for the various buttons on the page (janky, but easiest due to the way our setup is...)
 */
document.getElementById("grayscale_btn").addEventListener("click", conv_gray);


/**
 * In this section, we add our interfacing functions which will be called by the buttons on the frontend, and will subsequently call
 * the rust wasm code to execute the commands (and perform any other JS actions such as image rendering, etc...)
 */

function conv_gray() {
    let new_img = wasm.to_grayscale(photo_ptr);
    console.log(new_img);
    document.querySelector("#display_image").style.backgroundImage = `url(${new_img})`;
}


/**
 * This section is for the image upload
 */
const image_input = document.querySelector("#image_input");

image_input.addEventListener("change", function () {
    const reader = new FileReader();
    //wasm.set_img_bytes();
    reader.addEventListener("load", () => {
        const uploaded_image = reader.result;
        // Get the uploaded image as a pointer in the rust-wasm so we can use it
        if (uploaded_image) {
            const needle = "image/png;base64,";
            var a = uploaded_image.substring(uploaded_image.toString().indexOf(needle) + needle.length);
            photo_ptr = wasm.set_img_bytes(a);
        }
        document.querySelector("#display_image").style.backgroundImage = `url(${uploaded_image})`;
    });
    reader.readAsDataURL(this.files[0]);
});