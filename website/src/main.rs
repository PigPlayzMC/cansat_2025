use std::{
    fs,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream}
};

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:5500").unwrap();
    
    println!("Server operational and ready to accept requests.");
    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();

        handle_connections(stream);
    };
}

fn handle_connections(mut stream: TcpStream) {
    let mut buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    let mut request_line: String = String::new();

    if buf_reader.read_line(&mut request_line).is_ok() {
        let request_line: &str = request_line.trim();
        println!("NEW request: {}", request_line);

        if request_line.starts_with("GET") {
            let parts: Vec<&str> = request_line.split_whitespace().collect();
            handle_get_requests(&mut stream, parts[1]);
        } else if request_line.starts_with("POST") {
            handle_post_request(&mut stream);
        } else { // Deny all non GET or POST requests, such as trace and put. We only do the bare minimum here!
            println!("ERROR 405: HTTP method {} received. This is not supported.", request_line);
            let response: String = format!(
                "HTTP/1.1 405 Method Not Allowed\r\nContent-Length: 0\r\nAllow: {}\r\n\r\n",
                "GET, POST"
            );
        
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}

fn handle_get_requests(stream: &mut TcpStream, request_path: &str) {
    let file_path: String = if request_path == "/" { // Catch wrong requests and redirect rather than serving an error
        "home.html".to_string()
    } else {
        request_path.trim_start_matches('/').to_string()
    };

    ////println!("File path = {}", file_path);
    let mut_file_path: String = file_path.clone();
    let file: String = mut_file_path.replace("website/", "");
    let content: String = fs::read_to_string(file).unwrap_or_else(|_| {
        String::from("<h1>404 Not Found</h1>")
    });

    // Get file type
    let content_type: &'static str = if file_path.ends_with(".html") {
        "text/html"
    } else if file_path.ends_with(".css") {
        "text/css"
    } else if file_path.ends_with(".js") {
        "application/javascript"
    } else if file_path.ends_with(".svg") || file_path.ends_with(".xml") {
        "image/svg+xml"
    } else if file_path.ends_with(".jpeg") || file_path.ends_with(".jpg") {
        "image/jpeg" // Do not use jpg, this is a safety catch just in case
    } else if file_path.ends_with(".ico") {
        "image/ico" //TODO Test
    } else if file_path.ends_with(".json") {
        "application/json"
    } else {
        "text/plain"
    };
    ////println!("Content Type = {}", content_type);
    
    let response: String = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}",
        content.len(),
        content_type,
        content
    );

    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_post_request(stream: &mut TcpStream) {
    
}