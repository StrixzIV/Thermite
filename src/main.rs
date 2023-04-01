use clap::Parser;
use std::path::PathBuf;
use std::net::TcpListener;

mod connection_handler;
mod utils;

use connection_handler::handle_connection;
use utils::ThreadPool;

#[derive(Parser)]
#[clap(author = "Jirayu Kaewsing", version, about)]
struct Args {
    source_path: PathBuf,
    #[clap(short, long)]
    port: Option<u16>
}

fn main() {

    let args = Args::parse();
    
    let port: u16 = args.port.unwrap_or(8080);
    let listener = TcpListener::bind(format!("localhost:{}", port)).unwrap();
    let thread_pool = ThreadPool::new(4);

    for stream in listener.incoming() {

        let stream = stream.unwrap();
        let path = args.source_path.clone();

        thread_pool.execute(move || {
            handle_connection(stream, &path);
        });

    }

}
