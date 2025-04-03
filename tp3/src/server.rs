use crate::thread_pool::ThreadPool;
use std::net::TcpListener;
use connection_handler::handler::handle_connection;

pub fn start_pool_server(thread_amount: usize) {
    /*

    Conceptually:
    ThreadPool {
        workers: Vec<Worker>
    }

    onRequest (req){
    if(none_available){
        workers.extend(8)
    }
        workers.any(isAvailable).first.process_request();
    }
     */
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // TODO: extend pool when full
    let pool = ThreadPool::new(thread_amount);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| handle_connection(stream))
    }
}