use std::{fs::{self, File}, io::Write};
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
    time: Option<u64>, // For logging authentication attempts NOT for uuids
}

#[derive(Deserialize, Serialize, Debug)]
struct TrueCredentials {
    user: String,
    pass: String,
    uuid: String
}

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
    }

    let parsed_credentials: Value = serde_json::from_str(&raw_credentials).unwrap_or_else(|_| Value::Null);
    if parsed_credentials == Value::Null {
        eprintln!("ERROR: Failed to parse credentials.json");
        panic!("File parse failure: credentials.json");
    }

    println!("Successfully opened credentials.json!");

    // check for access_log.txt {or create}
    let access_log: String = fs::read_to_string("access.txt").unwrap_or_else(|_| "FAIL".to_string());
    if access_log == "FAIL".to_string() {
        eprintln!("ERROR: Access log not found. Creating now...");
        let _ = File::create("access.txt").unwrap(); // Assume this won't fail, because that would be bad and I am always an optimist.
    }
    drop(access_log); // Only used to ensure the log exists BEFORE it is needed in logging a log in (yeah that sounds weird change later...)

    println!("Server at http://{} - Fully operational!", url);

    println!("Attempting to create example credentials...");
    ////let _ = new_credentials("charliehbird@gmail.com".to_string(), "password".to_string());
    println!("Example credentials created!");

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

    let content_type = if file_name.ends_with(".html") {
        "text/html"
    } else if file_name.ends_with(".css") {
        "text/css"
    } else if file_name.ends_with(".js") {
        "application/javascript"
    } else if file_name.ends_with(".svg") || file_name.ends_with(".xml") {
        "image/svg+xml"
    } else if file_name.ends_with(".jpeg") || file_name.ends_with(".jpg") {
        "image/jpeg" // Do not use .jpg please, to avoid confusion.
    } else if file_name.ends_with(".ico") {
        "image/ico"
    } else if file_name.ends_with(".json") {
        "application/json"
    } else {
        "text/plain"
    };

    if file_name.ends_with("salts.json") || file_name.ends_with("credentials.json") { // Maybe not for transmission, even if they ask nicely
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
    let mut buf: Vec<u8> = Vec::new();
    request.as_reader().read_to_end(&mut buf)?;

    // TODO determine post request purpose (register credentials, verify credentials, create post)

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

            credentials.pass = credentials.pass;

            // Find account to sign into
            let mut actual_credentials = TrueCredentials {
                user: "default".to_string(), //Not valid so no chance of natural occurence also hashed so dont be stupid
                pass: "default".to_string(),
                uuid: "default".to_string(),
            };

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
                } else {
                    // Username is not registered
                    // Return 
                }
            }

            // Convert string uuid to real uuid
            println!("UUID: {}", actual_credentials.uuid);
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
            let state: &'static str;
            let accepted: bool;
            if double_password_hash == actual_credentials.pass {
                println!("LOGIN: Success! User {} logged in.", actual_credentials.user); // This uses actual credentials to avoid compiler flipout
                state = "SUCCESS: "; // There is a more efficient way to do this
                accepted = true;
            } else {
                println!("LOGIN: Fail! Incorrect password for user {}.", actual_credentials.user);
                state = "FAIL: "; // And here
                accepted = false;
            }

            // Record login attempt and status
            let log: String = state.to_string() + &actual_credentials.user + " " + &credentials.pass + "\r\n";
            let _ = fs::OpenOptions::new().append(true).create(true).open("access.txt")?.write_all(log.as_bytes());
        
            if accepted {
                //TODO Generate random token

                // TODO Encrypt random token

                // TODO Transmit random token
                let mut resp: Response<std::io::Cursor<Vec<u8>>> = Response::from_string("OK");
                resp.add_header(Header::from_bytes(&b"Content-Type"[..], &b"text/plain"[..]).unwrap());
                resp.add_header(Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap());
                request.respond(resp)?;
            } else {
                let resp = Response::empty(403); // Not allowed!!!
                let _ = request.respond(resp);
            }
        }
        Err(err) => {
            eprintln!("Failed to parse POST JSON: {}", err);
            let mut resp: Response<std::io::Cursor<Vec<u8>>> = Response::from_string("Bad Request");
            resp = resp.with_status_code(400);
            resp.add_header(Header::from_bytes(&b"Content-Type"[..], &b"text/plain"[..]).unwrap());
            resp.add_header(Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap());
            request.respond(resp)?;
        }
    }

    Ok(())
}

#[allow(dead_code)] // Because this is a very useful debug function that I'm not commenting out
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

    // TODO Write new credentials
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

#[allow(dead_code)]
fn hash_string(string: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(string.as_bytes());
    let result = hasher.finalize();

    return hex::encode(result)
}