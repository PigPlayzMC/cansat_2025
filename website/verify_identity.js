//* Don't put passwords in this file (or any...)
const submit_button = document.getElementById("submit_button");
const username_text = document.getElementById("username_box"); // This is actually an email, for simplicity
const password_text = document.getElementById("password_box");

const security = document.getElementById("security");
const post_maker = document.getElementById("post_maker");

// ! Deprecated, do not use
////const description_box = document.getElementById("description_box");
////const content_box = document.getElementById("content_box");

const auth = true; // DEBUG ONLY PLEASE!!!!

// regexs
// VV Valid for RFC 5322 VV
const email_regex = /(?:[a-z0-9!#$%&'*+\x2f=?^_`\x7b-\x7d~\x2d]+(?:\.[a-z0-9!#$%&'*+\x2f=?^_`\x7b-\x7d~\x2d]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9\x2d]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9\x2d]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9\x2d]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])/
const letter_regex = /[a-zA-Z]/;
const capital_regex = /[A-Z]/;
const number_regex = /[0-9]/;
const symbol_regex = /[^A-Za-z0-9]/; // Please do not use random unicode characters, I am begging. Special symbols like ! not 👼 please. Genuinely don't know if Argon2 can take it but I don't want to find out...

// For posting
const url = "127.0.0.1:5500";

// Cryptography
async function hashString(string) {
    const encoder = new TextEncoder();
    const dataToHash = encoder.encode(string);
    const hashBuffer = await crypto.subtle.digest('SHA-256', dataToHash);

    const hashArray = Array.from(new Uint8Array(hashBuffer));
    const hashHex = hashArray.map(byte => byte.toString(16).padStart(2, '0')).join('');

    return hashHex
};

function verifyEmail(email) { // See email regex if you dare...
    if (email_regex.test(email)) {
        return true
    } else {
        return false
    };
};

function verifyPassword(password) {
    /*
        Must contain:
        minimum 8 chars
        1 letter
        1 capital (letter not london)
        1 number
        1 symbol
    */
   
    // For registering, not for checking it is valid before submission to the server.
    if (!(password.length >= 12)) {
        console.log("Password fail - Too short");
    };
    if (!letter_regex.test(password)) {
        console.log("Pasword fail - No letters");
    };
    if (!capital_regex.test(password)) {
        console.log("Password fail - No capitals");
    };
    if (!number_regex.test(password)) {
        console.log("Password fail - No numbers");
    };
    if (!symbol_regex.test(password)) {
        console.log("Password fail - No symbols");
    };
};

async function postUsernamePassword(hashedUsername, hashedPassword, url) {
    let currentTime = new Date().getTime();
    let protocol = "http://"; // Change with cert?

    try {
        const response = await fetch(protocol + url, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                user: hashedUsername,
                pass: hashedPassword,
                time: currentTime,
            }),
        });

        if (!response.ok) {
            console.error("Server responded with status: {}", response.status);
            alert("Invalid credentials. Please try again. WARNING: Repeated incorrect attempts may result in rate limiting.")
            // ^^ Rate limiting: Not yet implemented
            // TODO Rate limiting
        } else {
            const responseData = await response.text();
            console.log("Response from server: ", responseData); // This is our token!

            console.log("Token received!");
            localStorage.clear(); // Get rid of any previous token that may exist (NOTE: This only affects this site, other storage is fine dw)
            localStorage.setItem("Token", responseData);


        }
    } catch (error) {
        console.error("Error verifying credentials: ", error);
    }
};

async function getUsernamePassword() {
    const username = (username_text.value).toLowerCase();
    const password = password_text.value;

    ////console.log(username + " - " + password);
    // Ensure email is valid, to avoid wasting server time.
    if (verifyEmail(username)) { // Note to self: try calling the function properly BEFORE you start trying to fix it.
        // Hash username and password
        const hashedUsername = await hashString(username);
        const hashedPassword = await hashString(password); // Kind of pointless as HTTPS is superior and this does nothing but its a feature!

        // AND THIS [although not really required as it is now hashed]
        ////console.log(hashedUsername + " - " + hashedPassword);

        // Now post the username and password for serverside verification.
        postUsernamePassword(hashedUsername, hashedPassword, url);
    } else {
        alert("Please enter a valid email adress.");
    };
};

function formatTeamOnly() { // Also vulnerable to script injection probably
    /*
    This is a convinience feature NOT a security feature. Access granting can only ever happen on a server side.
    Everything else is assumed tampered with by either inspect element or malicious request creation. This simply
    doesn't let people write a post without logging in previously (in theory). Someone will probably feel special
    and like they hacked us right up until the time they have to submit the post and they don't have a token, or
    they have the wrong token and the server rejects it.
    */
    security.style.display = "none";
    post_maker.style.display = "flex"; // I ❤️ flexbox (also this is a unicode heart? No, only on my default theme apparently)

    ////description_box.innerText = "";
    ////content_box.innerText = "";
}

submit_button.addEventListener("click", function(e) {
    getUsernamePassword();
});

if (auth) { // Just comment this out when its not needed tbh
    // DEBUG
    console.error("DEBUG FEATURE LEFT ON!!! CEASE PRODUCTION USE IMMEDIATELY");
    formatTeamOnly();
    localStorage.setItem("Token", "778a867a-08b8-44de-8c3e-0f96c5662a68");
}