use crate::connection_handler::handle_connection;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::net::TcpListener;
use thread_pool::thread_pool::ThreadPool;

pub struct Server {
    pub file_stats: HashMap<String, usize>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            file_stats: HashMap::new(),
        }
    }
    pub fn start(&mut self, thread_amount: usize) {
        let listener = TcpListener::bind("127.0.0.1:3030").unwrap();

        let pool = ThreadPool::new(thread_amount); // TODO: use tokio
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            pool.execute(move || handle_connection(stream, &mut self))
        }
    }
    pub fn get_exceptions(&mut self) -> i64 {
        self.file_stats.values().map(|&v| v as i64).sum()
    }
}

pub struct Response {
    pub status_code: i32,
    pub body: String,
}

impl Response {
    pub fn new(status_code: i32, body: String) -> Self {
        Response { status_code, body }
    }
    fn status(&self) -> String {
        let status_string = match self.status_code {
            200 => "OK",
            201 => "Created",
            429 => "Too Many Requests",
            400 => "Bad Request",
            500 => "Internal Server Error",
            _ => "Unknown Error",
        };
        String::from(status_string)
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = format!(
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
            self.status_code,
            self.status(),
            self.body.len(),
            self.body,
        );
        write!(f, "{}", string)
    }
}
