use std::io::prelude::*;
use std::net::{ TcpListener, TcpStream };

fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}

fn main() {
    
    let port: u16 = 8080;
    let listener = TcpListener::bind(format!("localhost:{}", port)).unwrap();

    for stream in listener.incoming() {

        let stream = stream.unwrap();
        handle_connection(stream)

    }

}
