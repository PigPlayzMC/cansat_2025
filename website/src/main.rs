use std::fs::{self, File};
use serde_json::Value;
use tiny_http::{Server, Request, Method, Response, Header};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Credentials {
    user: String,
    pass: String,
    time: Option<u64>,
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

    let parsed_credentials: serde_json::Value = serde_json::from_str(&raw_credentials).unwrap_or_else(|_| Value::Null);
    if parsed_credentials == Value::Null {
        eprintln!("ERROR: Failed to parse credentials.json");
        panic!("File parse failure: credentials.json");
    }

    println!("Successfully opened credentials.json!");

    // check for access_log.txt {or create}
    let mut access_log: String = fs::read_to_string("access.txt").unwrap_or_else(|_| "FAIL".to_string());
    if access_log == "FAIL".to_string() {
        eprintln!("ERROR: Access log not found. Creating now...");
        let _ = File::create("access.txt").unwrap(); // Assume this won't fail, because that would be bad and I am always an optimist.
        access_log = fs::read_to_string("access.txt").unwrap();
    }

    println!("Server at http://{} - Fully operational!", url);

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
    match serde_json::from_slice::<Credentials>(&buf) {
        Ok(creds) => {
            println!("Received credentials: {:?}", creds);
            let mut credentials: Credentials = Credentials {
                user: (creds.user),
                pass: (creds.pass),
                time: (creds.time)
            };

            credentials.pass = credentials.pass;

            // TODO verify from json file
            // Rehash password with salt (uuid)
            let salt = 0;

            // TODO Record login attempt and status
        
            // TODO give token for posting rights
            let mut resp: Response<std::io::Cursor<Vec<u8>>> = Response::from_string("OK");
            resp.add_header(Header::from_bytes(&b"Content-Type"[..], &b"text/plain"[..]).unwrap());
            resp.add_header(Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap());
            request.respond(resp)?;
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