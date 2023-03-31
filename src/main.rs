use std::fs;
use std::io::prelude::*;
use std::net::{ TcpListener, TcpStream };

use clap::Parser;

struct HTTPRequest<'a> {
    method: &'a str,
    path: &'a str
}

#[derive(Parser)]
#[clap(author = "Jirayu Kaewsing", version, about)]
struct Args {
    #[clap(short, long)]
    port: Option<u16>
}

fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0; 1024];
    
    stream.read(&mut buffer).unwrap();
    
    let raw_request = String::from_utf8_lossy(&buffer[..]);
    let request = raw_request.split("\r\n").collect::<Vec<&str>>()[0];
    
    let req = HTTPRequest {
        method: request.split(" ").collect::<Vec<&str>>()[0].trim(),
        path: request.split(" ").collect::<Vec<&str>>()[1].strip_prefix("/").unwrap()
    };

    println!("{}", format!("Request: {}, Path: {}", req.method, req.path));

    let (filename, status) = match req.path {
        "" | "index.html" => ("index.html", "HTTP/1.1 200 OK"),
        _ => ("404.html", "HTTP/1.1 404 NOT FOUND")
    };

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

    let args = Args::parse();
    
    let port: u16 = args.port.unwrap_or(8080);
    let listener = TcpListener::bind(format!("localhost:{}", port)).unwrap();

    for stream in listener.incoming() {

        let stream = stream.unwrap();
        handle_connection(stream);

    }

}
