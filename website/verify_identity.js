//* Don't put passwords in this file (or any...)
const submit_button = document.getElementById("submit_button");
const username_text = document.getElementById("username_box"); // This is actually an email, for simplicity
const password_text = document.getElementById("password_box");

// regexs
// VV Valid for RFC 5322 VV
const email_regex = /(?:[a-z0-9!#$%&'*+\x2f=?^_`\x7b-\x7d~\x2d]+(?:\.[a-z0-9!#$%&'*+\x2f=?^_`\x7b-\x7d~\x2d]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9\x2d]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9\x2d]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9\x2d]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])/
const letter_regex = /[a-zA-Z]/;
const capital_regex = /[A-Z]/;
const number_regex = /[0-9]/;
const symbol_regex = /[^A-Za-z0-9]/; // Please do not use random unicode characters, I am begging. Special symbols like ! not 👼 please.

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
}

async function getUsernamePassword() {
    const username = (username_text.value).toLowerCase();
    const password = password_text.value;

    ////console.log(username + " - " + password);
    // Ensure email is valid, to avoid wasting server time.
    if (verifyEmail(username)) { // Note to self: try call the function properly BEFORE you start trying to fix it.
        console.log("Email matches RegEx filter.");

        // Hash username and password
        const hashedUsername = await hashString(username);
        const hashedPassword = await hashString(password);

        // AND THIS [although not really required as it is now hashed]
        ////console.log(hashedUsername + " - " + hashedPassword);

        // Now post the username and password for serverside verification.
    } else {
        alert("Please enter a valid email adress.");
    };
};

submit_button.addEventListener("click", function(e) {
    getUsernamePassword();
});

/*
TODO:

- Authenticate team members
    - Hash uname and pwd for transmit
    - username and password check
- Check authentication on post
- Issue session/token (in requests)
*/