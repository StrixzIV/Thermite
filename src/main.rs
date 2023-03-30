use std::net::TcpListener;


fn main() {
    
    let port: u16 = 8080;
    let listener = TcpListener::bind(format!("localhost:{}", port)).unwrap();

    for stream in listener.incoming() {

        let stream = stream.unwrap();
        println!("Connection established.");

    }

}
