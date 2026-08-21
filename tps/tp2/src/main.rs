use connection_handler::handler::handle_connection;
use std::net::TcpListener;
use std::thread;

fn main() {
    start_server();
}

fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| handle_connection(stream));
    }
}
// Concurrent approach

// fn main() { start_server() }
// Testing request with n = 500, c = 50
// With pi = 9_000_000, it lasts 3.5 secs avg.
