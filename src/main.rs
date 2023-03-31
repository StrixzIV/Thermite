use clap::Parser;
use std::net::TcpListener;

mod connection_handler;
use connection_handler::handle_connection;

#[derive(Parser)]
#[clap(author = "Jirayu Kaewsing", version, about)]
struct Args {
    #[clap(short, long)]
    port: Option<u16>
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
