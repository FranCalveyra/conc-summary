use connection_handler::handler::handle_connection;
use std::net::TcpListener;

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
