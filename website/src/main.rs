use std::fs;
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

    //TODO Check for credentials.json

    //TODO check for 
    
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
            request.respond(resp);
            Ok(())
        }
    }
}

fn handle_get(mut request: Request) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let path = request.url();
    let file_path = if path == "/" { "home.html".to_string() } else {
        path.trim_start_matches('/').to_string()
    };

    let file_name = file_path.replace("website/", "");
    let content = fs::read_to_string(&file_name).unwrap_or_else(|_| "<h1>404 Not Found</h1>".to_string());

    // guess content-type (kept similar to your original)
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

    let mut response = Response::from_string(content);
    response.add_header(Header::from_bytes(&b"Content-Type"[..], content_type.as_bytes()).unwrap());
    response.add_header(Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap()); // useful for local frontend testing
    request.respond(response)?;
    Ok(())
}

fn handle_post(mut request: Request) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Read the entire body (tiny_http gives a reader for body)
    let mut buf = Vec::new();
    request.as_reader().read_to_end(&mut buf)?;

    // Try to parse JSON body into Credentials
    match serde_json::from_slice::<Credentials>(&buf) {
        Ok(creds) => {
            println!("Received credentials: {:?}", creds);

            // TODO verify from json file

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