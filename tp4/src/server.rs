use crate::connection_handler::handle_connection;
use std::collections::HashMap;
use std::fmt::Display;
use std::net::TcpListener;
use std::sync::{Arc, RwLock};
use thread_pool::thread_pool::ThreadPool;
use tokio::sync::Semaphore;

pub struct Server {
    pub file_stats: RwLock<HashMap<String, usize>>,
    pub file_semaphore: Semaphore,
}

impl Server {
    pub fn new() -> Self {
        Server {
            file_stats: RwLock::new(HashMap::new()),
            file_semaphore: Semaphore::new(4),
        }
    }
    pub fn start(self: Arc<Self>, thread_amount: usize) {
        let listener = TcpListener::bind("127.0.0.1:3030").unwrap();
        let pool = ThreadPool::new(thread_amount);
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let server_ref = Arc::clone(&self);
            pool.execute(move || handle_connection(stream, server_ref))
        }
    }
    pub fn get_exceptions(self: Arc<Self>) -> i64 {
        self.file_stats
            .try_read()
            .unwrap()
            .values()
            .map(|&v| v as i64)
            .sum()
    }
}
