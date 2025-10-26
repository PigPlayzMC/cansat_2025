// This file probably needs a better name
const post_button = document.getElementById("post_button");

const file_type_regex = /\.jpeg|\.png|\.jpg|\.svg/;
// ^^ There's server side support for xml so it may as well be a valid option.
const image_tag_regex = /\[.+\.(jpeg|png|jpg|svg)]/;
// ^^ [ + any+1 length of any characters + . + any valid file extension + ]
// Requring all of this to be fulfilled reduces the chance of false positives
// I have just learnt what an XML is and no. Not an image; not invited.
const jpeg_jpg_regex = /\.jpeg\.jpg/;
const png_regex = /\.png/;
const svg_regex = /\.svg/;

// Attributes
//TODO constant document references
const title = document.getElementById("title_box");
const content = document.getElementById("content_box");
const description = document.getElementById("description_box");
const image_folder = document.getElementById("image_file");

let title_text;
let content_text;
let description_text;

const protocol = "http://";
const url1 = "127.0.0.1:5500";

post_button.addEventListener("click", function(e) {
    getPostAttributes();
});

function getPostAttributes() {
    // get values from document references
    title_text = title.value;
    content_text = content.value;
    description_text = description.value;

    if (validateTexts() && validateImages()) {
        let json_text = jsonifyText();

        postNewPost(json_text, images_required);
    } else {
        console.error("ERROR: Text and/or image checks failed - Upload cancelled");
        alert("Text and/or image criteria unfulfilled. Please try again.");
    };
};

function validateTexts() { // Returns bool
    // Check all fields contain text and check description is shorter than 255 chars
    if (title_text.length > 0) {
        console.log("Title length check passed!")
    } else {
        console.warn("Title length check failed!");
        return false
    };

    if (content_text.length > 0) {
        console.log("Content length check passed!")
    } else {
        console.warn("Content length check failed!");
        return false
    };

    if (description_text.length > 0 && description_text.length < 256) {
        console.log("Description length check passed!");
    } else {
        console.warn("Description length check failed!");
        return false
    };

    return true
};

function validateImages() { // Returns bool
    ////console.log(image_folder.files);

    let files = [];

    let valid = true;
    let iter = 0;
    const max = image_folder.files.length;

    while (iter < max) { // Remake the below functionality but allowing for other files to also be in the folder
        if (file_type_regex.test(image_folder.files[iter].name)) {
            files.push(image_folder.files[iter].name);
        }; // else do nothing -> homework.mp4 or actual_homework.docx are both allowed to be submitted, just not uploaded.

        iter += 1;
    };

    // Check if every file has one of the correct file extensions (.jpg, .jpeg, .png, .svg)
    /* // This is unneeded as only required files are posted and these files must have the correct tags
    while (iter < max) {
        if (file_type_regex.test(image_folder.files[iter].name)) {
            console.log("File type check passed: " + (iter+1) + "/" + max);
            files.push(image_folder.files[iter].name); // For image collecting tags, assuming this is a valid set of files
        } else if (image_folder.files[iter].name == ".DS_Store") {
            /// Invisible Apple file from looking at folders in Finder [thanks apple.]
            console.log(".DS_Store encountered - Disregarding: " + (iter+1) + "/" + max);
        } else {
            console.warn("File type check failed: " + (iter+1) + "/" + max);
            valid = false;
        };

        iter += 1;
    };

    if (!valid) {
        return false
    } else {
        console.log("File type checking passed!");
    };
    */

    // Find image tags
    images_required = collectImageTags(); // This is an array [ should have used Typescript :( ]

    // CRITICAL: Check all images are actually images. Without, anything could be uploaded! (Like CPP or Rust)
    // TODO Also have server side verifcation for this
    iter = 0;
    while (iter < images_required.length) {
        if (file_type_regex.test(images_required[iter])) {
            console.log("File type check passed: " + (iter+1) + "/" + images_required.length);
        } else {
            console.warn("File type check failed: " + (iter+1) + "/" + max);
            valid = false;
        };

        iter += 1; // STOP FORGETTING TO INCREMENT!!!
    };

    if (!valid) {
        return false
    } else {
        console.log("File type checking passed!");
    };

    // Check named files are present
    iter = 0;
    let images_present = true;
    ////console.log(files);
    while(iter < images_required.length) {
        if (files.includes(images_required[iter])) {
            console.log("File " + images_required[iter] + " found!");
        } else {
            console.warn("File " + images_required[iter] + " not found!");
            images_present = false;
        };

        iter += 1;
    };

    if (!images_present) {
        return false
    } else {
        console.log("All required files found!");
    };

    // Check thumbnail exists
    if( // There's a better way to do this... [now case sensitive]
        files.includes("thumbnail.jpeg")
        | files.includes("thumbnail.png")
        | files.includes("thumbnail.jpg")
        | files.includes("thumbnail.svg")
        ////| files.includes("thumbnail.xml")
        ////| files.includes("Thumbnail.jpeg")
        ////| files.includes("Thumbnail.png")
        ////| files.includes("Thumbnail.jpg")
        ////| files.includes("Thumbnail.svg")
        ////| files.includes("Thumbnail.xml")
    ) {
        console.log("Thumbnail found!");
    } else {
        console.warn("Thumbnail not found!");
        return false
    };

    return true
};

function collectImageTags() { // Returns files required for the document
    // Called during verification and also during posting to ensure only required files are posted

    // Find all content tags then convert to a real array
    let tag_array = []
    try {
        const tag_exec_array = image_tag_regex.exec(content_text); // Really weird object so we will convert
        tag_array = tag_exec_array[0].split(" "); // Whitespace will never occur in a file (PLEASE don't shatter my illusions)
    } catch {
        // tag_exec_array is null -> no tags were used
        return [];
    }
    
    let iter = 0;
    while (iter < tag_array.length) {
        ////console.log("Running...");
        tag_array[iter] = tag_array[iter].substring(1, tag_array[iter].length - 1); // Remove square brackets
        iter += 1;
    };

    ////console.log(tag_array);

    return tag_array
};

function jsonifyText() { // Returns JSON object
    //NOTE: JSON.stringify() automatically adds new lines where enter characters are found

    // Convert text into a JSON structure
    ////console.log(content_text);

    let text_json = JSON.stringify({ // This is only the text fields, for transmission
        "title": title_text,
        "description": description_text,
        "content": content_text
    });

    ////console.log(text_json);

    return(text_json);
};

async function postNewPost(json_text, images_required) {
    const token = localStorage.getItem("Token"); // Either stringified UUIDv4 or Null

    // Post request for json of text
    try {
        const text_response = await fetch(protocol + url1, { 
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                "Authorization": `Bearer ${token}`
            },
            body: json_text,
        });

        if (!text_response.ok) {
            console.error("Server responded with status:", text_response.status);
            if (text_response.status == 403) {
                alert("Invalid token. Please copy work then reload to log in again.");
            };
            return false
        } else {
            console.log("Text content posted!");
        };
    } catch {
        console.error("Failed to post text contents");
        return false
    };

    // Not compressing images. Good luck backend!

    // TODO Post requests for each image
    iter = 0;
    let content_type;
    let index = 0;
    while (iter < images_required.length) {
        const form_data = new FormData; // Because I didn't use a form element

        let iter2 = 0;
        while (iter2 < image_folder.files.length) {
            if (image_folder.files[iter2].name == images_required[iter]) {
                form_data.append("image", image_folder.files[iter2]);
                break
            };
        };

        const image_response = await fetch(protocol + url1, {
            method: "POST",
            headers: {
                "Authorization": `Bearer ${token}`
            },
            body: form_data
        });

        if (!image_response.ok) {
            console.error("Server responded with status:", image_response.status);
            if (image_response.status == 403) {
                alert("Invalid token. Please copy work then reload to log in again.");
            };
            return false
        } else {
            console.log("Image content posted: " + (iter+1) + "/" + images_required.length);
        };

        iter += 1; // Very important. My apologies to the countless Firefox instances killed due to these issues.
    };

    // TODO Return response
};