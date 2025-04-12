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
    pub fn start(&self, thread_amount: usize) {
        // Should we use tokio?
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

        let pool = ThreadPool::new(thread_amount);
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            pool.execute(|| handle_connection(stream, &self))
        }
    }
}

pub struct Response {
    status_code: i32,
    body: String,
}

impl Response {
    pub fn new(status_code: i32, body: String) -> Self {
        Response { status_code, body }
    }
}

impl Display for Response{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "HTTP")
    }
}