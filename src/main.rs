use std::fs;
use std::io::prelude::*;
use std::net::{ TcpListener, TcpStream };

struct HTTPRequest<'a> {
    method: &'a str,
    path: &'a str
}

fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0; 1024];
    
    stream.read(&mut buffer).unwrap();
    
    let raw_request = String::from_utf8_lossy(&buffer[..]);
    let request = raw_request.split("\r\n").collect::<Vec<&str>>()[0];
    
    let req = HTTPRequest {
        method: request.split(" ").collect::<Vec<&str>>()[0].trim(),
        path: request.split(" ").collect::<Vec<&str>>()[1].trim()
    };

    println!("{}", format!("Request: {}, Path: {}", req.method, req.path));

    let (status, filename) = if req.method == "GET" && req.path == "/" {("HTTP/1.1 200 OK", "index.html")} else {("HTTP/1.1 404 NOT FOUND", "404.html")};

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
