use std::time::Duration;
use std::{fs, thread};
use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::net::TcpStream;

struct HTTPRequest<'a> {
    method: &'a str,
    endpoint: &'a str
}

pub fn handle_connection(mut stream: TcpStream, source_path: &PathBuf) {

    let mut buffer = [0; 1024];
    
    stream.read(&mut buffer).unwrap();
    
    let raw_request = String::from_utf8_lossy(&buffer[..]);
    let request = raw_request.split("\r\n").collect::<Vec<&str>>()[0];
    
    let req = HTTPRequest {
        method: request.split(" ").collect::<Vec<&str>>()[0].trim(),
        endpoint: request.split(" ").collect::<Vec<&str>>()[1].strip_prefix("/").unwrap()
    };
    
    println!("{}", format!("Request: {}, Path: {}", req.method, req.endpoint));
    
    let (filename, status) = match req.endpoint {
        "" | "index.html" => ("index.html", "HTTP/1.1 200 OK"),
        "sleep"  => {
            thread::sleep(Duration::from_secs(5));
            ("sleep.html", "HTTP/1.1 200 OK")
        },
        _ => ("404.html", "HTTP/1.1 404 NOT FOUND")
    };

    let contents = fs::read_to_string(source_path.join(filename)).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}