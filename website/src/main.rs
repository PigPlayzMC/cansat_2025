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

        if request_line.starts_with("GET") {
            let parts: Vec<&str> = request_line.split_whitespace().collect();
            handle_get_requests(&mut stream, parts[1]);
        } else if request_line.starts_with("POST") {
            handle_post_request(&mut stream);
        } else {
            println!("Unsupported HTTP method recieved.");
        }
    }
}

fn handle_get_requests(stream: &mut TcpStream, request_path: &str) {
    let file_path: String = if request_path == "/" {
        "home.html".to_string()
    } else {
        request_path.trim_start_matches('/').to_string()
    };

    println!("File path = {}", file_path);
    let mut mut_file_path: String = file_path.clone();
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
    } else if file_path.ends_with(".json") {
        "application/json"
    } else {
        "text/plain"
    }; //TODO JSON, IMAGES
    println!("Content Type = {}", content_type);
    
    let response: String = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}",
        content.len(),
        content_type,
        content
    );

    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_post_request(stream: &mut TcpStream) {
    let mut buf_reader: BufReader<&mut TcpStream> = BufReader::new(stream);
    let mut headers: String = String::new();
    let mut body: String = String::new();

    // Read headers until empty line with line break char
    while let Ok(bytes_read) = buf_reader.read_line(&mut headers) {
        if bytes_read == 0 || headers == "\r\n" {
            break;
        }
        headers.clear();
    }

    buf_reader.read_to_string(&mut body).unwrap();
    
    //TODO determine desired action
    
    //TODO rest of process

    let response: &'static str = "HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\nHello World!";
    stream.write_all(response.as_bytes()).unwrap();
}