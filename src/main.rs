use std::fs;
use std::io::prelude::*;
use std::net::{ TcpListener, TcpStream };

fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    let (status, filename) = if buffer.starts_with(get) {("HTTP/1.1 200 OK", "index.html")} else {("HTTP/1.1 404 NOT FOUND", "404.html")};

    let contents = fs::read_to_string(format!("resources/{filename}")).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}

fn main() {
    
    let port: u16 = 8080;
    let listener = TcpListener::bind(format!("localhost:{}", port)).unwrap();

    for stream in listener.incoming() {

        let stream = stream.unwrap();
        handle_connection(stream);

    }

}
