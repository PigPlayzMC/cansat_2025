use std::{io::BufReader, net::{TcpListener, TcpStream}};

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1").unwrap();

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();

        handle_connections(stream);
    }
}

fn handle_connections(mut stream: TcpStream) {
    let mut buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);

    
}