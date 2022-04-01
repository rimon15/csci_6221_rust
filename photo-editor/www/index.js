/**
 * This file will contain our bare-bones JS frontend. We will write the HTML directly to the window
 * and have basic methods for our filters, file uploads, etc. Once these functions are called in this file, they will
 * reference the rust wasm-bindgen implementations to actually perform the operations on the images. All we really need
 * here is a basic layout, image display, buttons for filters, and an upload file option.
 */

import * as wasm from "photo_editor";

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


</style>
`);

/** 
 * Now we just render the HTML of the frontend container for our page
 */
document.write(`
                <html>
                    <body>
                        <h2>Welcome to the WASM Photo Editor!</h2>
                    </body>
                    <div class="row">
                        <div class="first-column" style="background-color:#aaa;">
                            <h2>Image Functions</h2>
                            <p><button type="button" id="test_btn">Add rust functions!</button></p>
                            <p>
                                Upload an image
                                <input type='file' />
                            </p>
                        </div>
                        <div class="second-column" style="background-color:#bbb;">
                            <h2>Column 2</h2>
                            <p>Some text..</p>
                        </div>
                    </div>
                </html>`);

/** 
 * In this section, we add all of our event listeners for the various buttons on the page (janky, but easiest due to the way our setup is...)
 */
document.getElementById("test_btn").addEventListener("click", print_console);


/**
 * In this section, we add our interfacing functions which will be called by the buttons on the frontend, and will subsequently call
 * the rust wasm code to execute the commands (and perform any other JS actions such as image rendering, etc...)
 */

/**
 * Just an example function. Runs a wasm function, and also gets a return value from it, this is basically what we'll be doing
 * throughout the project I think
 */
function print_console() {
    var a = wasm.console_log();
    console.log(a);
}

/**
 * 
 */
window.addEventListener('load', function() {
    document.querySelector('input[type="file"]').addEventListener('change', function() {
        if (this.files && this.files[0]) {
            console.log('here');
            var img = document.querySelector('img');
            img.onload = () => {
                URL.revokeObjectURL(img.src);  // no longer needed, free memory
            }
  
            img.src = URL.createObjectURL(this.files[0]); // set src to blob url
            console.log(img);
        }
    });
  });