#[forbid(unsafe_code)] // It's against the spirit of the language (and totally unneeded here)
// TODO Move certain functions to different files; this is unprofessional
// TODO Consider using a real database because these JSON files are completely insane

use std::{
    fs::{
        self,
        File,
    },
    io::Write,
};
use regex::Regex;
use serde_json::Value;
use tiny_http::{
    Server,
    Request,
    Method,
    Response,
    Header,
};
use serde::{
    Deserialize,
    Serialize,
};
use chrono::{
    Duration,
    Utc,
};

// Security only
use uuid::Uuid;
use argon2::{
    password_hash::SaltString,
    Argon2,
    PasswordHasher,
};
use sha2::{
    Digest,
    Sha256,
};

#[derive(Deserialize, Debug)]
struct Credentials {
    user: String,
    pass: String,
    time: Option<u64>, // For logging authentication attempts NOT for uuids (uuids are random because I used v4 by accident instead of v7)
}

#[derive(Deserialize, Serialize, Debug)]
struct TrueCredentials {
    user: String,
    pass: String,
    uuid: String
}

#[derive(Serialize)]
struct TokenLog {
    token: String,
    user: String,
    time_done: String,
    expires: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct TextContent {
    title: String,
    description: String,
    content: String
}

#[derive(Serialize)]
struct Post {
    post_id: String, // UUID v7
    title: String,
    thumbnail: String, // This should be interpreted as a path to the thumbnail as the filename will always be thumbnail.{file_extension}
    description: String,
    content: String, // ! Content logging is deprecated
    date_created: String, // convert from NaiveDate
}

// At some point this broke Live Server's auto reload function so just open like normal, run the server, then manually reload
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url: &'static str = "127.0.0.1:5500";
    let server: Server = Server::http(url)?;
    println!("Server connection open!");

    // open credentials.json
    let raw_credentials: String = fs::read_to_string("credentials.json").unwrap_or_else(|_| "FAIL".to_string());
    if raw_credentials == "FAIL".to_string() {
        let _ = File::create("credentials.json");
        eprintln!("ERROR: Failed to open credentials.json");
        panic!("File open failure: credentials.json");
    };

    let parsed_credentials: Value = serde_json::from_str(&raw_credentials).unwrap_or_else(|_| Value::Null);
    if parsed_credentials == Value::Null {
        eprintln!("ERROR: Failed to parse credentials.json");
        panic!("File parse failure: credentials.json");
    };

    println!("Successfully opened credentials.json!");

    // check for access_log.txt {or create}
    let access_log: String = fs::read_to_string("access.txt").unwrap_or_else(|_| "FAIL".to_string());
    if access_log == "FAIL".to_string() {
        eprintln!("ERROR: Access log not found. Creating now...");
        let _ = File::create("access.txt").unwrap(); // Assume this won't fail, because that would be bad and I am always an optimist.
    }
    println!("Successfully opened access.txt");
    drop(access_log); // Only used to ensure the log exists BEFORE it is needed in logging a log in (yeah that sounds weird change later...)

    println!("Server at http://{} - Fully operational!", url);

    ////println!("Attempting to create example credentials...");
    ////let _ = new_credentials("charliehbird@gmail.com".to_string(), "password".to_string());
    ////println!("Example credentials created!");

    for request in server.incoming_requests() {
        let _ = handle_requests(request);
    }

    Ok(())
}

fn handle_requests(request: Request) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match *request.method() {
        Method::Get => handle_get(request),
        Method::Post => handle_post(request),
        _ => {
            let mut resp = Response::empty(405);
            resp.add_header(Header::from_bytes(&b"Allow"[..], &b"GET, POST"[..]).unwrap());
            let _ = request.respond(resp);
            Ok(())
        }
    }
}

fn handle_get(request: Request) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let path = request.url();
    let file_path = if path == "/" { "home.html".to_string() } else {
        path.trim_start_matches('/').to_string()
    };

    let file_name = file_path.replace("website/", "");
    let content = fs::read_to_string(&file_name).unwrap_or_else(|_| "<h1>404 Not Found</h1>".to_string());

    let content_type = if file_name.ends_with(".html") { //TODO match?
        "text/html"
    } else if file_name.ends_with(".css") {
        "text/css"
    } else if file_name.ends_with(".js") {
        "text/javascript" // Should only be text/javascript (but application/javascript does actually work anyway because *ba...*)
    } else if file_name.ends_with(".svg") || file_name.ends_with(".xml") {
        "image/svg+xml"
    } else if file_name.ends_with(".jpeg") || file_name.ends_with(".jpg") {
        "image/jpeg" // Ignore previous comment. Don't use whatever an xml is
    } else if file_name.ends_with(".png") {
        "image/png"
    } else if file_name.ends_with(".ico") {
        "image/ico"
    } else if file_name.ends_with(".json") {
        "application/json"
    } else {
        "text/plain"
    };

    // VV Salts.json not in use (or existence) consider removal
    if file_name.ends_with("salts.json") || file_name.ends_with("credentials.json") || file_name.ends_with("tokens_granted.json"){ // Maybe not for transmission, even if they ask nicely
        println!("Attempted access to restricted files");
        let resp = Response::empty(403); // Could send 404 to obscure existence but this is open source
        let _ = request.respond(resp);
    } else { // We can and will share it
        let mut response = Response::from_string(content);
        response.add_header(Header::from_bytes(&b"Content-Type"[..], content_type.as_bytes()).unwrap());
        response.add_header(Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap());
        request.respond(response)?;
    }
    Ok(())
}

fn handle_post(mut request: Request) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut buf: Vec<u8> = Vec::new(); // This is empty body attribute!!
    request.as_reader().read_to_end(&mut buf)?; // This is request body!!

    let content_type: String  = request
        .headers() // The meta data
        .iter() // for _ in ___
        .find(|h| h.field.equiv("Content-Type")) // Where the data is about content-type
        .unwrap() // Do the operation
        .to_string() // As a string
        .replace("Content-Type: ", ""); // Remove "Content-Type: "
    ////println!("{}", content_type);

    // Determine post request purpose (verify credentials, create post)
    if let Some(header) = request.headers().iter().find(|h| h.field.equiv("Authorization")) && content_type == "application/json" { // [text] Post process, verify creds, make post, send 201
        println!("");
        println!("Text POST request received!");

        // Even if there is no token, the Bearer token will = Bearer Null thanks to JS magic
        // Which helps differentiate an invalid post maker POST from a(n) (in)valid 
        // function for verify token rather than this, as image posts will also require verification
        let token: String = header.value.to_string();
        let token_accepted = verify_token(token);

        if !token_accepted { // Token wrong or expired
            let resp: Response<std::io::Empty> = Response::empty(403); // Not a valid token or has expired.
            let _ = request.respond(resp);
        } else { // Token exists and is not expired
            // Read json body (only done now as untokened JSON might be dangerous? idk, just very scared for my precious opsec)
            match serde_json::from_slice::<TextContent>(&buf) {
                Ok(text) => {
                    let text_content: TextContent = TextContent {
                        title: text.title,
                        description: text.description,
                        content: text.content
                    };

                    ////println!("{:?}", text_content);
                    ////let title = &text_content.title;
                    ////let description = &text_content.description;

                    // HTMLify text content
                    let new_document: String = htmlify(text_content.clone()); // Compiler really doesn't like any other way of doing this
                    ////println!("{}", new_document);
                    
                    let post_id: String = Uuid::now_v7().to_string(); // Based on current system time
                    
                    // Save as post{id}.html
                    let path: String = "posts/".to_owned() + &post_id + ".html";
                    let _ = fs::write(&path ,new_document);
                    
                    // Create folder with same name for folders
                    let _ = fs::create_dir("posts/".to_owned() + &post_id);

                    // Struct for conversion to JSON object
                    let post_data: Post = Post {
                        post_id: post_id.clone(),
                        title: text_content.title,
                        thumbnail: "posts/".to_string() + &post_id, // Path to thumbnail rather than actual thumbnail filename
                        description: text_content.description,
                        content: "".to_string(), // ! Deprecated field - Do Not Use
                        date_created: Utc::now().date_naive().to_string(),
                    };

                    // Rewrite posts.json [this may be the most computationally expensive element of this file but Rust is fast...
                    // and this is a low demand service so these compiler acrobatics are fine]
                    let post_json: String = serde_json::to_string_pretty(&post_data).unwrap();

                    // Open posts/posts.json (copied from credentials creation code)
                    let mut posts: String = fs::read_to_string("posts/posts.json").unwrap();
                    posts = posts.replace("]", "").trim_end().to_string();

                    // Add new post
                    posts = posts + "," + &post_json + "]";

                    // Rewrite file (this destroys the original data but is required)
                    let _ = fs::write("posts/posts.json", posts);

                    println!("New HTML post file created and saved!");

                    let mut resp: Response<std::io::Cursor<Vec<u8>>> = Response::from_string("Created");
                    resp = resp.with_status_code(201);
                    resp.add_header(Header::from_bytes(&b"Content-Type"[..], &b"text/plain"[..]).unwrap());
                    resp.add_header(Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap());
                    request.respond(resp)?;
                }
                Err(err) => {
                    eprintln!("ERROR: Failed to parse POST text JSON: {}", err);
                    let mut resp: Response<std::io::Cursor<Vec<u8>>> = Response::from_string("Bad Request");
                    resp = resp.with_status_code(400);
                    resp.add_header(Header::from_bytes(&b"Content-Type"[..], &b"text/plain"[..]).unwrap());
                    resp.add_header(Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap());
                    request.respond(resp)?;
                }
            };
        }
    } else if let Some(header) = request.headers().iter().find(|h| h.field.equiv("Authorization")) {
        println!("");
        println!("Image POST request received!");

        // Verify token
        let token: String = header.value.to_string();
        let token_accepted = verify_token(token);

        if !token_accepted { // Token wrong or expired
            let resp = Response::empty(403); // Not a valid token or has expired.
            let _ = request.respond(resp);
        } else {
            println!("Image saving begun...");

            // Determine image type
            println!("Content type = {}", content_type);
            let extension: &'static str;
            if content_type == "image/png".to_string() {
                extension = "png";
            } else if content_type == "image/jpeg" {
                extension = "jpeg";
            } else if content_type == "image/svg+xml" {
                extension = "svg";
            } else {
                extension = "png";
            };
            // Any malicious file will find itself suddenly a png file, full of spite

            // TODO get file name
            let file_name = "bob_test";

            // TODO Find last post id for folderisation (TODO stop calling things Xerisation)
            let last_id = "019a2c18-8427-7292-ad0f-d5d6fc902368";

            let mut body = buf.clone();
            

            let mut new_image = File::create("posts/".to_string() + last_id + "/" + file_name + "." + extension).unwrap();
            new_image.write_all(&buf).unwrap();

            // TODO Save each verified file to the folder

            // TODO Respond with relevant code
        }
    } else { // Sign in process, verify creds, send token
        println!("");
        println!("No authorisation token located. Sign in process begun...");

        // open credentials.json
        let raw_credentials: String = fs::read_to_string("credentials.json").unwrap(); // Won't fail, its already passed the initial check

        let parsed_credentials: Value = serde_json::from_str(&raw_credentials).unwrap(); // Same here

        match serde_json::from_slice::<Credentials>(&buf) {
            Ok(creds) => {
                println!("Received credentials: {:?}", creds);
                let mut credentials: Credentials = Credentials {
                    user: (creds.user),
                    pass: (creds.pass),
                    time: (creds.time)
                };

                //TODO Uncover secret meaning behind this because, as a genius, I never do random stuff, just forget my masterstrokes.
                credentials.pass = credentials.pass; // ?? Whjat is this forr??

                // Find account to sign into
                let mut actual_credentials = TrueCredentials {
                    user: "default".to_string(), //Not valid so no chance of natural occurence also hashed so dont be stupid
                    pass: "default".to_string(),
                    uuid: "default".to_string(),
                };

                let mut valid: bool = false; // Whether the username exists
                if let Some(array) = parsed_credentials.as_array() {
                    if let Some(obj) = array.iter().find(|item| item["user"] == credentials.user) {
                        if let Some(username) = obj.get("user").and_then(|v| v.as_str()) 
                            && let Some(password) = obj.get("pass").and_then(|v| v.as_str())
                            && let Some(uuid) = obj.get("uuid").and_then(|v| v.as_str())
                        { // Assume this will always be true as it was parsed as proper JSON earlier.
                            actual_credentials = TrueCredentials {
                                user: username.to_string(), // Not strictly required.
                                pass: password.to_string(),
                                uuid:  uuid.to_string()
                            };
                        } else {
                            panic!("ERROR: Incorrect JSON?");
                        }
                        valid = true;
                    } else {
                        valid = false;
                    }
                }

                let state: &'static str;
                let accepted: bool;
                if valid {
                    // Convert string uuid to real uuid
                    ////println!("UUID: {}", actual_credentials.uuid);
                    let uuid: Uuid = Uuid::parse_str(&actual_credentials.uuid).unwrap(); // Assume valid because I did it
                    let salt_bytes: &[u8; 16] = uuid.as_bytes();

                    // Rehash password with salt (uuid)
                    let salt: SaltString = SaltString::encode_b64(salt_bytes)?; // Assume no errors again :/

                    let argon2: Argon2<'_> = Argon2::default(); // This is the recommended setting from OWASP

                    let double_password_hash: String = argon2
                        .hash_password(credentials.pass.as_bytes(), &salt)?
                        .to_string()
                        .replace("$argon2id$v=19$m=19456,t=2,p=1$", "")
                        .replace("$", "");

                    // Check whether these VERY VERY processed credentials actually justify any of the HOURS of work
                    if double_password_hash == actual_credentials.pass {
                        println!("LOGIN: Success! User {} logged in.", actual_credentials.user); // This uses actual credentials to avoid compiler flipout
                        state = "SUCCESS: "; // There is a more efficient way to do this
                        accepted = true;
                    } else {
                        println!("LOGIN: Fail! Incorrect password for user {}.", actual_credentials.user);
                        state = "FAIL: "; // And here
                        accepted = false;
                    }
                } else {
                    println!("LOGIN: FAIL! No such user {}", credentials.user);
                    state = "FAIL: ";
                    accepted = false;
                }
                

                // Record login attempt and status
                let log: String = state.to_string() + &credentials.user + " " + &credentials.pass + "\r\n"; // Logs to new line
                // Has to be credentials.user or falsely credentialed logins would be wrong
                let _ = fs::OpenOptions::new().append(true).create(true).open("access.txt")?.write_all(log.as_bytes());
            
                if accepted {
                    let token: String = generate_token(credentials.user);

                    let mut resp: Response<std::io::Cursor<Vec<u8>>> = Response::from_string(token);
                    resp.add_header(Header::from_bytes(&b"Content-Type"[..], &b"text/plain"[..]).unwrap());
                    resp.add_header(Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap());
                    request.respond(resp)?;
                } else { // Either wrong password or wrong username (Both can't happen because a false username doesn't have a password)
                    let resp = Response::empty(403); // Not allowed!!!
                    let _ = request.respond(resp);
                }
            }
            Err(err) => {
                eprintln!("ERROR: Failed to parse POST JSON: {}", err);
                let mut resp: Response<std::io::Cursor<Vec<u8>>> = Response::from_string("Bad Request");
                resp = resp.with_status_code(400);
                resp.add_header(Header::from_bytes(&b"Content-Type"[..], &b"text/plain"[..]).unwrap());
                resp.add_header(Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap());
                request.respond(resp)?;
            }
        };
    };

    Ok(())
}

#[allow(dead_code)] // Because this is a very useul debug function that I'm not commenting out
fn new_credentials(email: String, password: String, ) -> Result<TrueCredentials, Box<dyn std::error::Error>> {
    let hashed_email = hash_string(email);
    let once_hashed_password = hash_string(password);
    
    let uuid: Uuid = Uuid::new_v4(); // New random uuid of 16 bytes

    // Rehash password [Identical to what happens to posted passwords - Could become fn at some point if needed]
    let salt_bytes: &[u8; 16] = uuid.as_bytes();

    // Rehash password with salt (uuid)
    let salt: SaltString = SaltString::encode_b64(salt_bytes)?; // Assume no errors again :/

    let argon2: Argon2<'_> = Argon2::default(); // This is the recommended setting from OWASP

    let double_password_hash: String = argon2
        .hash_password(once_hashed_password.as_bytes(), &salt)?
        .to_string()
        .replace("$argon2id$v=19$m=19456,t=2,p=1$", "")
        .replace("$", "");

    let new_credentials: TrueCredentials = TrueCredentials {
        user: hashed_email,
        uuid: uuid.to_string(),
        pass: double_password_hash,
    };

    //Write new credentials
    let json_credentials: String = serde_json::to_string_pretty(&new_credentials).unwrap(); // So ✨pretty✨✨! (Not AI, just cringe)

    // Open credentials.json
    let mut creds_string: String = fs::read_to_string("credentials.json").unwrap(); // Will be fine as the initial check passed
    creds_string = creds_string.replace("]", "").trim_end().to_string(); // Remove last char

    // Add new creds
    creds_string = creds_string + "," + &json_credentials + "]";

    // Rewrite file (this destroys the original data but is required)
    let _ = fs::write("credentials.json", creds_string);

    Ok(new_credentials)
}

// Allow dead code is inherited for this as the compiler pretends it is called in new_credentials despite that not running!
fn hash_string(string: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(string.as_bytes());
    let result = hasher.finalize();

    return hex::encode(result)
}

fn generate_token(user: String) -> String {
    // Using v4 uuid again
    let token: Uuid = Uuid::new_v4(); // New random uuid of 16 bytes
    let token_string: String = token.to_string(); // Stwing!

    // Create log of token generated, user3 authed, time done, time expires (time gened + 30m)
    let now: chrono::DateTime<Utc> = Utc::now();
    let time: i64 = now.timestamp();
    let expiry: i64 = (now + Duration::minutes(30)).timestamp(); // 30 minutes to make a post should be fine but copy paste was invented for a reason

    let token_log: TokenLog = TokenLog {
        token: (token_string),
        user: (user),
        time_done: (time.to_string()),
        expires: (expiry.to_string())
    };

    let json_token: String = serde_json::to_string_pretty(&token_log).unwrap(); // So ✨pretty✨✨! (Not AI, just cringe)

    // Open credentials.json
    // DO NOT CONFUSE WITH token_string AS THIS WILL ____ UP THE DATA
    let mut tokens_string: String = fs::read_to_string("tokens_granted.json").unwrap(); // Will be fine as the initial check passed
    tokens_string = tokens_string.replace("]", "").trim_end().to_string(); // Remove last char

    // Add new creds
    tokens_string = tokens_string + "," + &json_token + "]"; // Will always be right to add a comma because of the example

    // Rewrite file (this destroys the original data but is required)
    let _ = fs::write("tokens_granted.json", tokens_string);

    // Not bothering to encrypt, a token is fine though HTTPS
    // TODO encrypt :\

    return token_log.token;
}

fn htmlify(text_json: TextContent) -> String {
    println!("HTML Formatting begun...");

    // HTML constants [UPDATE TO CHANGE APPEARANCE]
    // Consider storing in a file
    let header_part1: &'static str = r#"<!DOCTYPE html> <!-- NOTE: Do not use AI to generate code for this repository. Thank you for your understanding.-->
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link href="post_styling.css" rel="stylesheet">
    <title>CanSat | "#;

    // Title goes between these two string literals

    let header_part2: &'static str = r#"</title>
</head>
<body>
    <div id="header" class="header">
        <div id="title" class="title">
            CanSat 2025
        </div>
        <div id="navigation_bar" class="navigation">
            <a href="home.html" class="nav_link">Home</a>
             | 
            <a href="about_us.html" class="nav_link">About us</a>
             | 
            <a href="team_only.html" class="nav_link">Team only</a>
        </div>
    </div>
    <div class="header_section">
        <!-- Leave blank to avoid spacing issues with the header.
        If these issues manifest, please alter the height of this in style.css-->
    </div>
    <div class=posts>
    <b>
"#;

let post_title_header = r#"</b><br><br>"#;

    let footer: &'static str = r#"
    </div>
</body>
</html>
"#;
    // ^^ These make indentation weird so ignore it pls tysm

    let title: String = text_json.title;

    let content: String = format_content(text_json.content); // image_tag replaced with <img>
    
    // header + title + content + footer
    let document: String = header_part1.to_string() + &title + header_part2 + &title + post_title_header + &content + footer;

    // Consider doing verification that no body does something stupid like using a reserved pattern eg; I am so <a> nnoying to the server team
    // Never mind, they get what they deserve. I'll just complain on the group chat
    return document // Valid HTML (probably)
}

fn format_content(content: String) -> String {
    // Image tag regex borrowed from clientside
    // Altered to allow for any character before and after
    let image_tag_regex: Regex = Regex::new(r"\[\S+\.\S+]").unwrap();
    // ^^ Expensive; consider defining in greater scope :)

    // Replace the tags for actual HTML.
    let mut iter: usize = 0; // Operationally expensive but we shouldn't have more than 5 images per page.
    // Very little total usage
    let content_split_whitespace: Vec<&str> = content.split("\n").collect(); // Each individual word in content
    let mut content_split_replaced: Vec<String> = vec![];
    while iter < content_split_whitespace.len() {
        let content_substring: String;
        ////println!("Seperated whitespace: {}", content_split_whitespace[iter]);
        if image_tag_regex.is_match(content_split_whitespace[iter]) { // Image tag -> refactor to use HTML standards
            // TODO Alter to include actual path, likely using post_id folder when image post handling is added
            content_substring = content_split_whitespace[iter].replace("[",r#"<img src=""#) // [ => <img src="
                .replace("]", r#"">"#); // ] => ">
        } else { // No processing required
            content_substring = content_split_whitespace[iter].to_string();
        };

        ////println!("Substring: {}", content_substring);
        content_split_replaced.push(content_substring);
        content_split_replaced.push("<br>".to_string()); // New line char

        iter += 1;
    };

    // Now accounts for new line chars
    let new_content: String = content_split_replaced.concat();

    ////println!("{}", new_content);

    return new_content;
}

fn verify_token(mut token: String) -> bool { // Check token against tokens_granted.json
    // TODO Remove debug token from tokens_granted.json
    println!("Authorisation token located: {}", token);

    token = token.replace("Bearer ", "");

    let mut token_accepted: bool = false;

    // Verify token exists && is valid
    let raw_tokens: String = fs::read_to_string("tokens_granted.json").unwrap_or_else(|_| "FAIL".to_string());
    if raw_tokens == "FAIL".to_string() {
        let _ = File::create("tokens_granted.json");
        eprintln!("ERROR: Failed to open tokens_granted.json");
        panic!("File open failure: tokens_granted.json");
    };

    let parsed_tokens: Value = serde_json::from_str(&raw_tokens).unwrap_or_else(|_| Value::Null);
    if parsed_tokens == Value::Null {
        eprintln!("ERROR: Failed to parse tokens_granted.json");
        panic!("File parse failure: tokens_granted.json");
    };

    if let Some(array) = parsed_tokens.as_array() { // Token validation
        if let Some(obj) = array.iter().find(|item| item["token"] == token) {
            println!("Token found!");

            if let Some(expire_time) = obj.get("expires").and_then(|v| v.as_i64()) {
                let now: chrono::DateTime<Utc> = Utc::now();
                let time: i64 = now.timestamp();

                if time <= expire_time {
                    println!("Token valid!"); // Expiry time is in the future
                    token_accepted = true;
                } else {
                    println!("Token expired!");
                    token_accepted = false;
                };
            } else {
                panic!("ERROR: Incorrect JSON in tokens_granted.json");
            }
        } else {
            println!("No such token; Connection refused.");
            token_accepted = false;
        };
    }; // Assume this won't fail...

    println!(""); // Seperates token validation dialog from file creation dialog that follows.

    return token_accepted
}